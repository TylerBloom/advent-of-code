pub use crate::solution::Solution;

#[derive(Clone)]
pub struct Entry {
    low: u32,
    high: u32,
    c: String,
    password: String,
}

impl Entry {
    pub fn load(input: String) -> Self {
        let filtered_input: String = input
            .chars()
            .filter(|c| c != &':')
            .map(|c| if c == '-' { ' ' } else { c })
            .collect();
        let data: Vec<&str> = filtered_input.split(" ").collect();
        Entry {
            low: data[0].parse().unwrap(),
            high: data[1].parse().unwrap(),
            c: String::from(data[2]),
            password: String::from(data[3]),
        }
    }

    pub fn is_valid_part_one(&self) -> bool {
        let digest: u32 = self
            .password
            .chars()
            .map(|c| (String::from(c) == self.c) as u32)
            .reduce(|a, b| a + b)
            .unwrap();
        (digest >= self.low) && (digest <= self.high)
    }

    pub fn is_valid_part_two(&self) -> bool {
        let digest: Vec<char> = self
            .password
            .chars()
            .enumerate()
            .filter(|t| t.0 as u32 == self.low - 1 || t.0 as u32 == self.high - 1)
            .map(|t| t.1)
            .collect();
        (String::from(digest[0]) == self.c) ^ (String::from(digest[1]) == self.c)
    }
}

pub struct Day2 {
    data: Vec<Entry>,
}

impl Solution<u32> for Day2 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<Entry> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(Entry::load(String::from(val)));
            }
        }
        Day2 { data }
    }

    fn solve_part_one(&self) -> u32 {
        self.data
            .clone()
            .into_iter()
            .map(|e| if e.is_valid_part_one() { 1 } else { 0 })
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn solve_part_two(&self) -> u32 {
        self.data
            .clone()
            .into_iter()
            .map(|e| if e.is_valid_part_two() { 1 } else { 0 })
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::{Day2, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        let solver = Day2::parse_input(input);
        assert_eq!(solver.solve_part_one(), 2);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        let solver = Day2::parse_input(input);
        assert_eq!(solver.solve_part_two(), 1);
    }
}

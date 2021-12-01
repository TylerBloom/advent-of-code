pub use crate::solution::Solution;

fn get_count(val: String) -> u64 {
    (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .filter(|c| val.contains(&c.to_string()))
        .count() as u64
}

pub struct Day6 {
    data: Vec<u64>,
}

impl Solution<u64> for Day6 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n\n").collect();
        let mut data: Vec<u64> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(get_count(val.to_string()));
            }
        }
        Day6 { data }
    }

    fn solve_part_one(&self) -> u64 {
        let mut digest: u64 = 0;
        for val in &self.data {
            digest += val;
        }
        digest
    }

    fn solve_part_two(&self) -> u64 {
        let mut digest: u64 = 0;
        for val in &self.data {
            digest += val;
        }
        digest
    }
}

#[cfg(test)]
mod tests {

    use super::{Day6, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
        let solver = Day6::parse_input(input);
        assert_eq!(solver.solve_part_one(), 11);
    }
}

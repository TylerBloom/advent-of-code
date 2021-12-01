pub use crate::solution::Solution;

fn get_loose_count(val: String) -> u64 {
    let parsed: Vec<String> = val.split("\n").map(|s| s.to_string()).collect();
    ('a'..='z')
        .filter(|c| parsed.iter().any(|p| p.contains(&c.to_string())))
        .count() as u64
}

fn get_strict_count(val: String) -> u64 {
    let parsed: Vec<String> = val.split("\n").map(|s| s.to_string()).collect();
    ('a'..='z')
        .filter(|c| parsed.iter().all(|p| p.contains(&c.to_string())))
        .count() as u64
}

pub struct Day6 {
    data: Vec<String>,
}

impl Solution<u64> for Day6 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n\n").collect();
        let mut data: Vec<String> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(val.to_string());
            }
        }
        Day6 { data }
    }

    fn solve_part_one(&self) -> u64 {
        self.data
            .iter()
            .map(|p| get_loose_count(p.to_string()))
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn solve_part_two(&self) -> u64 {
        self.data
            .iter()
            .map(|p| get_strict_count(p.to_string()))
            .reduce(|a, b| a + b)
            .unwrap()
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

    #[test]
    fn test_part_two() {
        let input = String::from("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb");
        let solver = Day6::parse_input(input);
        assert_eq!(solver.solve_part_two(), 6);
    }
}

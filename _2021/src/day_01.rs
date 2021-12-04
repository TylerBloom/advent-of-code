pub use crate::solution::Solution;
pub use crate::trailing::TrailingIterator;

pub struct Day1 {
    data: Vec<u32>,
}

impl Solution<u32> for Day1 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<u32> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(val.parse::<u32>().unwrap());
            }
        }
        Day1 { data }
    }

    fn solve_part_one(&mut self) -> u32 {
        self.data
            .iter()
            .trailing()
            .map(|(a, b)| a > b)
            .filter(|b| *b)
            .count() as u32
    }

    fn solve_part_two(&mut self) -> u32 {
        self.data
            .iter()
            .trailing()
            .trailing()
            .trailing()
            .map(|(((a1, a2), (_a3, a4)), ((b1, b2), (_b3, b4)))| (a1 + a2 + a4) > (b1 + b2 + b4))
            .filter(|b| *b)
            .count() as u32
    }
}

#[cfg(test)]
mod tests {

    use super::{Day1, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let mut solver = Day1::parse_input(input);
        assert_eq!(solver.solve_part_one(), 7);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("199\n200\n208\n210\n200\n207\n240\n269\n260\n263");
        let mut solver = Day1::parse_input(input);
        assert_eq!(solver.solve_part_two(), 5);
    }
}

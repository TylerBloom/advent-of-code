pub use crate::solution::Solution;

#[derive(Debug)]
struct BoardPass {
    row: String,
    col: String,
}

impl BoardPass {
    fn get_id(&self) -> u64 {
        self.row
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| if c == 'B' { 1_u64 << i } else { 0 })
            .reduce(|a, b| a + b)
            .unwrap()
            * 8
            + self
                .col
                .chars()
                .rev()
                .enumerate()
                .map(|(i, c)| if c == 'R' { 1_u64 << i } else { 0 })
                .reduce(|a, b| a + b)
                .unwrap()
    }
}

pub struct Day5 {
    data: Vec<BoardPass>,
}

impl Solution<u64> for Day5 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<BoardPass> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(BoardPass {
                    row: val[..7].to_string(),
                    col: val[7..].to_string(),
                });
            }
        }
        Day5 { data }
    }

    fn solve_part_one(&self) -> u64 {
        self.data.iter().map( |b| b.get_id() ).max().unwrap()
    }

    fn solve_part_two(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::{Day5, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL");
        let solver = Day5::parse_input(input);
        assert_eq!(solver.solve_part_one(), 820);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL");
        let solver = Day5::parse_input(input);
        assert_eq!(solver.solve_part_two(), 0);
    }
}

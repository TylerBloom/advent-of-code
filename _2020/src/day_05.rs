pub use crate::solution::Solution;
pub use crate::trailing::TrailingIterator;

fn get_id(row: String, col: String) -> u64 {
    row.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| if c == 'B' { 1_u64 << i } else { 0 })
        .reduce(|a, b| a + b)
        .unwrap()
        * 8
        + col
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| if c == 'R' { 1_u64 << i } else { 0 })
            .reduce(|a, b| a + b)
            .unwrap()
}

pub struct Day5 {
    data: Vec<u64>,
}

impl Solution<u64> for Day5 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<u64> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(get_id(val[..7].to_string(), val[7..].to_string()));
            }
        }
        data.sort();
        Day5 { data }
    }

    fn solve_part_one(&self) -> u64 {
        *self.data.last().unwrap()
    }

    fn solve_part_two(&self) -> u64 {
        self.data
            .iter()
            .trailing()
            .filter(|(&a, &b)| (a - b) == 2)
            .map( |(a,_b)| a - 1 )
            .collect::<Vec<u64>>()[0]
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
}

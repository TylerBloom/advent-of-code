pub use crate::solution::Solution;

pub fn run_solution(data: String) {
    let mut solver = Day9::parse_input(data);
    println!("Day 9 Part 1 Solution: {} ", solver.solve_part_one());
    println!("Day 9 Part 2 Solution: {} ", solver.solve_part_two());
}

pub struct Day9 {
    data: Vec<u8>,
    length: usize,
}

impl Day9 {
    fn is_low_point(&self, index: &usize) -> bool {
        let mut digest = true;
        // Look left
        if index % self.length != 0 {
            digest &= self.data[*index] < self.data[((*index as i32) - 1) as usize];
        }
        // Look left
        if index % self.length != self.length - 1 {
            digest &= self.data[*index] < self.data[index + 1];
        }
        // Look up
        if index >= &self.length {
            digest &= self.data[*index] < self.data[((*index as i32) - (*&self.length as i32)) as usize];
        }
        // Look down
        if (*index as i32) < (self.data.len() as i32 - *&self.length as i32) {
            digest &= self.data[*index] < self.data[index + self.length];
        }
        digest
    }
}

impl Solution<u64> for Day9 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let length = parsed[0].len();
        let mut data: Vec<u8> = Vec::with_capacity(length * parsed.len());
        for val in parsed.iter().filter(|v| !v.is_empty()) {
            for v in val.chars().map(|c| c.to_string().parse::<u8>().unwrap()) {
                data.push(v);
            }
        }
        Day9 { data, length }
    }

    fn solve_part_one(&mut self) -> u64 {
        self.data
            .iter()
            .enumerate()
            .filter(|(i, _)| self.is_low_point(i))
            .map(|(_, v)| (v + 1) as u64)
            .sum()
    }

    fn solve_part_two(&mut self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::{Day9, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("2199943210\n3987894921\n9856789892\n8767896789\n9899965678");
        let mut solver = Day9::parse_input(input);
        assert_eq!(solver.solve_part_one(), 15);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("2199943210\n3987894921\n9856789892\n8767896789\n9899965678");
        let mut solver = Day9::parse_input(input);
        //assert_eq!(solver.solve_part_two(), 5);
    }

    // Will be added when the solutions are found
    #[test]
    fn known_part_one_solution() {
        let mut solver = Day9::parse_input(read_to_string("data/day_09.txt").unwrap());
        assert_eq!(solver.solve_part_one(), 486);
    }

    #[test]
    fn known_part_two_solution() {
        let mut solver = Day9::parse_input(read_to_string("data/day_09.txt").unwrap());
        //assert_eq!(solver.solve_part_two(), 1543);
    }
}

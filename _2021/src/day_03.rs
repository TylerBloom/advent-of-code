pub use crate::solution::Solution;

use std::collections::HashSet;

pub fn run_solution(data: String) {
    let mut solver = Day3::parse_input(data);
    println!("Day 3 Part 1 Solution: {} ", solver.solve_part_one());
    println!("Day 3 Part 2 Solution: {} ", solver.solve_part_two());
}

fn bools_to_int(vals: &Vec<bool>) -> u64 {
    vals.iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| if b { 1 << i } else { 0 })
        .reduce(|a, b| a + b)
        .unwrap() as u64
}

pub struct Day3 {
    data: Vec<Vec<char>>,
}

impl Solution<u64> for Day3 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<Vec<char>> = parsed[0]
            .chars()
            .map(|_| Vec::with_capacity(parsed[0].len()))
            .collect();
        for val in parsed {
            for (i, c) in val.chars().enumerate() {
                data[i].push(c);
            }
        }
        Day3 { data }
    }

    fn solve_part_one(&mut self) -> u64 {
        let mut vals: Vec<bool> = Vec::with_capacity(self.data.len());
        for v in &self.data {
            vals.push(v.iter().filter(|&c| c == &'1').count() * 2 >= self.data[0].len());
        }
        let gamma = bools_to_int(&vals);
        let epsilon = bools_to_int(&vals.iter().map(|&b| !b).collect());
        gamma * epsilon
    }

    // This is a really gross solution...  The way data is stored should have
    // been changed between parts 1 and 2, though that solution was very nice
    fn solve_part_two(&mut self) -> u64 {
        let mut co2_vals: HashSet<usize> = HashSet::with_capacity(self.data[0].len() - 1);
        let mut life_vals: HashSet<usize> = HashSet::with_capacity(self.data[0].len() - 1);
        let mut co2: u64 = 0;
        let mut life: u64 = 0;
        let mut co2_bit: char;
        let mut life_bit: char;
        for v in &self.data {
            if v.iter()
                .enumerate()
                .filter(|(i, &c)| !co2_vals.contains(i) && c == '1')
                .count()
                * 2
                >= (self.data[0].len() - co2_vals.len())
            {
                co2_bit = '1';
            } else {
                co2_bit = '0';
            }
            if v.iter()
                .enumerate()
                .filter(|(i, &c)| !life_vals.contains(i) && c == '1')
                .count()
                * 2
                < (self.data[0].len() - life_vals.len())
            {
                life_bit = '1';
            } else {
                life_bit = '0';
            }
            for (i, &c) in v.iter().enumerate() {
                if c != co2_bit && co2_vals.len() + 1 != self.data[0].len() {
                    co2_vals.insert(i);
                }
                if c != life_bit && life_vals.len() + 1 != self.data[0].len() {
                    life_vals.insert(i);
                }
            }
            if co2_vals.len() + life_vals.len() + 2 == self.data[0].len() * 2 {
                let co2_index: usize = *(0..self.data[0].len())
                    .collect::<Vec<_>>()
                    .iter()
                    .filter(|i| !co2_vals.contains(i))
                    .collect::<Vec<_>>()[0];
                let life_index: usize = *(0..self.data[0].len())
                    .collect::<Vec<_>>()
                    .iter()
                    .filter(|i| !life_vals.contains(i))
                    .collect::<Vec<_>>()[0];
                co2 = bools_to_int(&self.data.iter().map(|vv| vv[co2_index] == '1').collect());
                life = bools_to_int(&self.data.iter().map(|vv| vv[life_index] == '1').collect());
                break;
            }
        }
        co2 * life
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::{Day3, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let mut solver = Day3::parse_input(input);
        assert_eq!(solver.solve_part_one(), 198);
    }

    #[test]
    fn test_part_two() {
        let input = String::from(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        let mut solver = Day3::parse_input(input);
        assert_eq!(solver.solve_part_two(), 230);
    }

    // Added after solution was accepted to ensure accuracy for refactoring
    #[test]
    fn known_part_one_solution() {
        let mut solver = Day3::parse_input(read_to_string("data/day_03.txt").unwrap());
        assert_eq!(solver.solve_part_one(), 4006064);
    }

    #[test]
    fn known_part_two_solution() {
        let mut solver = Day3::parse_input(read_to_string("data/day_03.txt").unwrap());
        assert_eq!(solver.solve_part_two(), 5941884);
    }
}

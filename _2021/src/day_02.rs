pub use crate::solution::Solution;

pub fn run_solution(data: String) {
    let mut solver = Day2::parse_input(data);
    println!("Day 2 Part 1 Solution: {} ", solver.solve_part_one());
    println!("Day 2 Part 2 Solution: {} ", solver.solve_part_two());
}

enum Direction {
    Forward,
    Up,
    Down,
}

fn get_direction(dir: &str) -> Direction {
    if dir == "forward" {
        Direction::Forward
    } else if dir == "up" {
        Direction::Up
    } else if dir == "down" {
        Direction::Down
    } else {
        panic!("Unknown value for direction: {}", dir);
    }
}

pub struct Day2 {
    data: Vec<(Direction, u64)>,
}

impl Solution<u64> for Day2 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<(Direction, u64)> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                let v = val.split(" ").collect::<Vec<_>>();
                data.push((get_direction(v[0]), v[1].parse::<u64>().unwrap()));
            }
        }
        Day2 { data }
    }

    fn solve_part_one(&mut self) -> u64 {
        let mut x_pos: u64 = 0;
        let mut y_pos: u64 = 0;
        for (dir, val) in &self.data {
            match dir {
                Direction::Forward => {
                    x_pos += val;
                }
                Direction::Up => {
                    y_pos -= val;
                }
                Direction::Down => {
                    y_pos += val;
                }
            }
        }
        x_pos * y_pos
    }

    fn solve_part_two(&mut self) -> u64 {
        let mut x_pos: i64 = 0;
        let mut y_pos: i64 = 0;
        let mut aim: i64 = 0;
        for (dir, val) in &self.data {
            match dir {
                Direction::Forward => {
                    x_pos += *val as i64;
                    y_pos += aim * (*val as i64);
                }
                Direction::Up => {
                    aim -= *val as i64;
                }
                Direction::Down => {
                    aim += *val as i64;
                }
            }
        }
        (x_pos * y_pos) as u64
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::{Day2, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let mut solver = Day2::parse_input(input);
        assert_eq!(solver.solve_part_one(), 150);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let mut solver = Day2::parse_input(input);
        assert_eq!(solver.solve_part_two(), 900);
    }

    // Added after solution was accepted to ensure accuracy for refactoring
    #[test]
    fn known_part_one_solution() {
        let mut solver = Day2::parse_input(read_to_string("data/day_02.txt").unwrap());
        assert_eq!(solver.solve_part_one(), 1480518);
    }

    #[test]
    fn known_part_two_solution() {
        let mut solver = Day2::parse_input(read_to_string("data/day_02.txt").unwrap());
        assert_eq!(solver.solve_part_two(), 1282809906);
    }
}

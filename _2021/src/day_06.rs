pub use crate::solution::Solution;


pub fn run_solution(data: String) {
    let mut solver = Day6::parse_input(data);
    println!("Day 6 Part 1 Solution: {} ", solver.solve_part_one());
    println!("Day 6 Part 2 Solution: {} ", solver.solve_part_two());
}

#[derive(Debug,Copy,Clone)]
struct Fish {
    timer: u8,
    count: u64,
}

impl Fish {
    fn next_day(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            true
        } else {
            self.timer -= 1;
            false
        }
    }
}

pub struct Day6 {
    data: Vec<Fish>,
}

impl Solution<u64> for Day6 {
    fn parse_input(mut input: String) -> Self {
        input.pop();
        let parsed: Vec<u8> = input.split(",").filter(|n| !n.is_empty()).map(|n| n.parse::<u8>().unwrap()).collect();
        let mut data: Vec<Fish> = Vec::new();
        let mut count: u64;
        for timer in 0_u8..7_u8 {
            count = parsed.iter().filter( |&n| n == &timer ).count() as u64;
            if count > 0 {
                data.push( Fish { timer, count } );
            }
        }
        Day6 { data }
    }

    fn solve_part_one(&mut self) -> u64 {
        let copy = self.data.clone();
        let mut count: u64 = 0;
        for _ in 0..80 {
            for fish in &mut self.data {
                if fish.next_day() {
                    count += fish.count;
                }
            }
            if count != 0 {
                self.data.push( Fish { timer: 8, count } );
                count = 0;
            }
        }
        let digest = self.data.iter().map(|f| &f.count).sum();
        self.data = copy;
        digest
    }

    fn solve_part_two(&mut self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::{Day6, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("3,4,3,1,2,");
        let mut solver = Day6::parse_input(input);
        assert_eq!(solver.solve_part_one(), 5934);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("3,4,3,1,2,");
        let mut solver = Day6::parse_input(input);
        assert_eq!(solver.solve_part_two(), 26984457539);
    }

    // Will add after solved
    #[test]
    fn known_part_one_solution() {
        let mut solver = Day6::parse_input(read_to_string("data/day_06.txt").unwrap());
        assert_eq!(solver.solve_part_one(), 360761);
    }

    #[test]
    fn known_part_two_solution() {
        let mut solver = Day6::parse_input(read_to_string("data/day_06.txt").unwrap());
        //assert_eq!(solver.solve_part_two(), 20666);
    }
}

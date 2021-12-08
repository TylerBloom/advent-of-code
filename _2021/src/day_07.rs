pub use crate::solution::Solution;

pub fn run_solution(data: String) {
    let mut solver = Day7::parse_input(data);
    println!("Day 7 Part 1 Solution: {} ", solver.solve_part_one());
    println!("Day 7 Part 2 Solution: {} ", solver.solve_part_two());
}

struct Capsule {
    position: i64,
    count: u64,
}

pub struct Day7 {
    data: Vec<Capsule>,
}

#[inline]
fn triangle_num(n: u64) -> u64 {
    (n*(n+1))/2
}

impl Day7 {
    fn get_shifted_count(&self, val: i64) -> u64 {
        self.data.iter().map(|c| c.count*((c.position - val).abs() as u64)).sum()
    }
    
    fn get_other_shifted_count(&self, val: i64) -> u64 {
        self.data.iter().map(|c| c.count*triangle_num((c.position - val).abs() as u64)).sum()
    }
}

impl Solution<u64> for Day7 {
    fn parse_input(mut input: String) -> Self {
        input.pop();
        let mut parsed: Vec<i64> = input
            .split(",")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        parsed.sort();
        let mut data: Vec<Capsule> = Vec::new();
        let mut cap = Capsule { position: 0, count: 0 };
        for i in parsed {
            if i != cap.position {
                data.push(cap);
                cap = Capsule { position: i, count: 1 };
            } else {
                cap.count += 1;
            }
        }
        data.push(cap);
        Day7 { data }
    }

    fn solve_part_one(&mut self) -> u64 {
        let mut i: i64 = 0;
        let mut last: u64 = u64::MAX;
        let mut current: u64 = self.get_shifted_count(i);
        while last >= current {
            i += 1;
            last = current;
            current = self.get_shifted_count(i);
        }
        last
    }

    fn solve_part_two(&mut self) -> u64 {
        let mut i: i64 = 0;
        let mut last: u64 = u64::MAX;
        let mut current: u64 = self.get_other_shifted_count(i);
        while last >= current {
            i += 1;
            last = current;
            current = self.get_other_shifted_count(i);
        }
        last
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::{Day7, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14,");
        let mut solver = Day7::parse_input(input);
        assert_eq!(solver.solve_part_one(), 37);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14,");
        let mut solver = Day7::parse_input(input);
        assert_eq!(solver.solve_part_two(), 168);
    }

    // Added after solution was accepted to ensure accuracy for refactoring
    #[test]
    fn known_part_one_solution() {
        let mut solver = Day7::parse_input(read_to_string("data/day_07.txt").unwrap());
        assert_eq!(solver.solve_part_one(), 355764);
    }

    #[test]
    fn known_part_two_solution() {
        let mut solver = Day7::parse_input(read_to_string("data/day_07.txt").unwrap());
        assert_eq!(solver.solve_part_two(), 99634572);
    }
}

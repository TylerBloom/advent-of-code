
pub use crate::solution::Solution;

use std::collections::HashSet;

pub struct Day1 {
    data: HashSet<u32>,
}

impl Solution<u32> for Day1 {
    fn parse_input( input: String ) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: HashSet<u32> = HashSet::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.insert( val.parse::<u32>().unwrap() );
            }
        }
        Day1{ data }
    }
    
    fn solve_part_one( &self ) -> u32 {
        let mut digest: u32 = 0;
        for val in &self.data {
            if self.data.contains( &(2020-val) ) {
                digest = val*(2020-val);
                break
            }
        }
        digest
    }
    
    fn solve_part_two( &self ) -> u32 {
        let mut digest: u32 = 0;
        let mut target: u32;
        for val in &self.data {
            target = 2020 - val;
            for v in &self.data {
                if v > &target {
                    continue;
                }
                if self.data.contains( &(target-v) ) {
                    digest = val*v*(target-v);
                    break
                }
            }
        }
        digest
    }
}

#[cfg(test)]
mod tests {
    
    use super::{Day1, Solution};
    
    #[test]
    fn test_part_one() {
        let input = String::from("1721\n979\n366\n299\n675\n1456");
        let solver = Day1::parse_input( input );
        assert_eq!( solver.solve_part_one(), 514579 );
    }
    
    #[test]
    fn test_part_two() {
        let input = String::from("1721\n979\n366\n299\n675\n1456");
        let solver = Day1::parse_input( input );
        assert_eq!( solver.solve_part_two(), 241861950 );
    }
}

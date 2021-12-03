pub use crate::solution::Solution;

fn bools_to_int( vals: &Vec<bool> ) -> u64 {
    vals.iter().rev().enumerate().map( |(i,&b)| if b { 1 << i } else { 0 } ).reduce( |a,b| a + b).unwrap() as u64
}

pub struct Day3 {
    data: Vec<Vec<char>>,
}

impl Solution<u64> for Day3 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<Vec<char>> = parsed[0].chars().map( |_| Vec::with_capacity(parsed[0].len()) ).collect();
        for val in parsed {
            for (i,c) in val.chars().enumerate() {
                data[i].push(c);
            }
        }
        Day3 { data }
    }

    fn solve_part_one(&self) -> u64 {
        let mut vals: Vec<bool> = Vec::with_capacity(self.data.len());
        for v in &self.data {
            vals.push( v.iter().filter( |&c| c == &'1' ).count()*2 >= self.data[0].len() );
        }
        let gamma = bools_to_int( &vals );
        let epsilon = bools_to_int( &vals.iter().map( |&b| !b ).collect() );
        gamma * epsilon
    }

    fn solve_part_two(&self) -> u64 {
        let mut vals: Vec<bool> = Vec::with_capacity(self.data.len());
        for v in &self.data {
            vals.push( v.iter().filter( |&c| c == &'1' ).count()*2 >= self.data[0].len() );
        }
        let gamma = bools_to_int( &vals );
        let epsilon = bools_to_int( &vals.iter().map( |&b| !b ).collect() );
        gamma * epsilon
    }
}

#[cfg(test)]
mod tests {

    use super::{Day3, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        let solver = Day3::parse_input(input);
        assert_eq!(solver.solve_part_one(), 198);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");
        let solver = Day3::parse_input(input);
        assert_eq!(solver.solve_part_two(), 198);
    }
}

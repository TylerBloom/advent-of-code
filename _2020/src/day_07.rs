pub use crate::solution::Solution;

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub struct Day7 {
    data: HashMap<String, Vec<String>>,
}

impl Day7 {
    // Note: This is an bad solution as it takes _at least_ n^2
    fn trace_bag(&self, bag: String) -> Vec<String> {
        let mut digest: Vec<String> = Vec::new();
        for (color, bags) in &self.data {
            if bags.contains(&bag) {
                digest.push(color.to_string());
                digest.append(&mut self.trace_bag(color.to_string()));
            }
        }
        //println!( "{} => {:?}", bag, digest );
        digest
    }
}

impl Solution<u64> for Day7 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let exp1 = Regex::new(r" bags?( |\.)?").unwrap();
        let exp2 = Regex::new(r"[0-9]+ ").unwrap();
        let exp3 = Regex::new(r"contain ").unwrap();
        let mut data: HashMap<String, Vec<String>> = HashMap::with_capacity(parsed.len());
        for val in parsed {
            if val.contains("no other") {
                continue;
            }
            if !val.is_empty() {
                let mut v = exp1.replace_all(val, "").to_string();
                v = exp2.replace_all(&v, "").to_string();
                v = exp3.replace_all(&v, ", ").to_string();
                let bags = v
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                data.insert(bags[0].to_string(), bags[1..].to_vec());
            }
        }
        Day7 { data }
    }

    fn solve_part_one(&self) -> u64 {
        self.trace_bag(String::from("shiny gold"))
            .iter()
            .unique()
            .count() as u64
    }

    fn solve_part_two(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::{Day7, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.");
        let solver = Day7::parse_input(input);
        println!("{:?}", solver.trace_bag("shiny gold".to_string()));
        assert_eq!(solver.solve_part_one(), 4);
    }
}

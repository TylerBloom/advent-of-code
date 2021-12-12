pub use crate::solution::Solution;

use std::collections::HashSet;

// This is not my best work... I will like refactor this, but I disliked this question greatly.

pub fn run_solution(data: String) {
    let mut solver = Day8::parse_input(data);
    println!("Day 8 Part 1 Solution: {} ", solver.solve_part_one());
    println!("Day 8 Part 2 Solution: {} ", solver.solve_part_two());
}

#[derive(Debug)]
struct SegmentMap {
    map: Vec<(HashSet<char>, u64)>,
}

impl SegmentMap {
    // Data needs to be stored by string length
    pub fn new(data: &Vec<HashSet<char>>) -> Self {
        let mut map = Vec::with_capacity(7);
        map.push((data[0].clone(), 1));
        map.push((data[1].clone(), 7));
        map.push((data[2].clone(), 4));
        map.push((data[9].clone(), 8));

        map.push((data[SegmentMap::find_two(&data)].clone(), 2));
        map.push((data[SegmentMap::find_three(&data)].clone(), 3));
        map.push((data[SegmentMap::find_five(&data)].clone(), 5));

        map.push((data[SegmentMap::find_zero(&data)].clone(), 0));
        map.push((data[SegmentMap::find_six(&data)].clone(), 6));
        map.push((data[SegmentMap::find_nine(&data)].clone(), 9));
        SegmentMap { map }
    }

    fn find_three(data: &Vec<HashSet<char>>) -> usize {
        if data[0].intersection(&data[3]).collect::<Vec<_>>().len() == 2 {
            3
        } else if data[0].intersection(&data[4]).collect::<Vec<_>>().len() == 2 {
            4
        } else {
            5
        }
    }

    fn find_two(data: &Vec<HashSet<char>>) -> usize {
        let index = SegmentMap::find_five(data);
        if data[index].intersection(&data[3]).collect::<Vec<_>>().len() == 3 {
            3
        } else if data[index].intersection(&data[4]).collect::<Vec<_>>().len() == 3 {
            4
        } else {
            5
        }
    }

    fn find_five(data: &Vec<HashSet<char>>) -> usize {
        let index = SegmentMap::find_six(data);
        if data[index].intersection(&data[3]).collect::<Vec<_>>().len() == 5 {
            3
        } else if data[index].intersection(&data[4]).collect::<Vec<_>>().len() == 5 {
            4
        } else {
            5
        }
    }

    fn find_zero(data: &Vec<HashSet<char>>) -> usize {
        let index = SegmentMap::find_five(data);
        if data[index].intersection(&data[6]).collect::<Vec<_>>().len() == 4 {
            6
        } else if data[index].intersection(&data[7]).collect::<Vec<_>>().len() == 4 {
            7
        } else {
            8
        }
    }

    fn find_six(data: &Vec<HashSet<char>>) -> usize {
        if data[1].intersection(&data[6]).collect::<Vec<_>>().len() == 2 {
            6
        } else if data[1].intersection(&data[7]).collect::<Vec<_>>().len() == 2 {
            7
        } else {
            8
        }
    }

    fn find_nine(data: &Vec<HashSet<char>>) -> usize {
        if data[2].intersection(&data[6]).collect::<Vec<_>>().len() == 4 {
            6
        } else if data[2].intersection(&data[7]).collect::<Vec<_>>().len() == 4 {
            7
        } else {
            8
        }
    }

    pub fn map(&self, data: &HashSet<char>) -> u64 {
        let mut digest: u64 = u64::MAX;
        for (s, v) in &self.map {
            if s == data {
                digest = v.clone();
                break;
            }
        }
        if digest == u64::MAX {
            panic!("That shouldn't happen.");
        }
        digest
    }
}

pub struct Day8 {
    input: Vec<Vec<HashSet<char>>>,
    output: Vec<Vec<HashSet<char>>>,
}

impl Solution<u64> for Day8 {
    fn parse_input(data: String) -> Self {
        let parsed: Vec<&str> = data.split("\n").collect();
        let mut input: Vec<Vec<HashSet<char>>> = Vec::with_capacity(parsed.len());
        let mut output: Vec<Vec<HashSet<char>>> = Vec::with_capacity(parsed.len());
        for val in parsed.iter().filter(|v| !v.is_empty()) {
            for (i, entry) in val.split("|").filter(|e| !e.is_empty()).enumerate() {
                let mut vals: Vec<HashSet<char>> = entry
                    .split(" ")
                    .filter(|num| !num.is_empty())
                    .map(|num| num.chars().collect())
                    .collect();
                if i == 0 {
                    vals.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
                    input.push(vals);
                } else {
                    output.push(vals);
                }
            }
        }
        Day8 { input, output }
    }

    fn solve_part_one(&mut self) -> u64 {
        self.output
            .iter()
            .map(|v| {
                v.iter()
                    .filter(|val| match val.len() {
                        2 | 3 | 4 | 7 => true,
                        _ => false,
                    })
                    .count() as u64
            })
            .sum()
    }

    fn solve_part_two(&mut self) -> u64 {
        let mut digest: Vec<u64> = Vec::with_capacity(self.input.len());
        let mut map: SegmentMap;
        for (i, entry) in self.input.iter().enumerate() {
            map = SegmentMap::new(&entry);
            digest.push(
                self.output[i]
                    .iter()
                    .map(|e| map.map(e))
                    .rev()
                    .enumerate()
                    .map(|(i, n)| 10_u64.pow(i as u32) * n)
                    .sum(),
            );
        }
        digest.iter().sum()
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::{Day8, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        let mut solver = Day8::parse_input(input);
        assert_eq!(solver.solve_part_one(), 26);
    }

    #[test]
    fn test_part_two_simple() {
        let input = String::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let mut solver = Day8::parse_input(input);
        assert_eq!(solver.solve_part_two(), 5353);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        let mut solver = Day8::parse_input(input);
        assert_eq!(solver.solve_part_two(), 61229);
    }

    // Added after solution was accepted to ensure accuracy for refactoring
    #[test]
    fn known_part_one_solution() {
        let mut solver = Day8::parse_input(read_to_string("data/day_08.txt").unwrap());
        assert_eq!(solver.solve_part_one(), 352);
    }

    #[test]
    fn known_part_two_solution() {
        let mut solver = Day8::parse_input(read_to_string("data/day_08.txt").unwrap());
        assert_eq!(solver.solve_part_two(), 936117);
    }
}

pub use crate::solution::Solution;

pub fn run_solution(data: String) {
    let mut solver = Day8::parse_input(data);
    println!("Day 8 Part 1 Solution: {} ", solver.solve_part_one());
    println!("Day 8 Part 2 Solution: {} ", solver.solve_part_two());
}

pub struct Day8 {
    data: Vec<String>,
}

impl Solution<u64> for Day8 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<String> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                for num in val.split("|").collect::<Vec<_>>()[1].split(" ") {
                    if !num.is_empty() {
                        data.push( num.to_string() );
                    }
                }
            }
        }
        Day8 { data }
    }

    fn solve_part_one(&mut self) -> u64 {
        self.data.iter().filter( |val| match val.len() { 2 | 3 | 4 | 7 => true, _ => false } ).count() as u64
    }

    fn solve_part_two(&mut self) -> u64 {
        0
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
    fn test_part_two() {
        let input = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        let mut solver = Day8::parse_input(input);
        //assert_eq!(solver.solve_part_two(), 168);
    }

    // Will be added after solution is found
    #[test]
    fn known_part_one_solution() {
        let mut solver = Day8::parse_input(read_to_string("data/day_08.txt").unwrap());
        //assert_eq!(solver.solve_part_one(), 355764);
    }

    #[test]
    fn known_part_two_solution() {
        let mut solver = Day8::parse_input(read_to_string("data/day_08.txt").unwrap());
        //assert_eq!(solver.solve_part_two(), 99634572);
    }
}

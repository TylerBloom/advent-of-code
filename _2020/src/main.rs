mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod solution;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;
use day_5::Day5;
use day_6::Day6;
use solution::Solution;

use std::fs;

fn load_datafile(file: &str) -> String {
    fs::read_to_string(format!("data/{}", file)).unwrap()
}

fn main() {
    let day_one = Day1::parse_input(load_datafile("day_1.txt"));
    println!("Day 1 Part 1 Solution: {} ", day_one.solve_part_one());
    println!("Day 1 Part 2 Solution: {} ", day_one.solve_part_two());

    let day_two = Day2::parse_input(load_datafile("day_2.txt"));
    println!("Day 2 Part 1 Solution: {} ", day_two.solve_part_one());
    println!("Day 2 Part 2 Solution: {} ", day_two.solve_part_two());

    let day_three = Day3::parse_input(load_datafile("day_3.txt"));
    println!("Day 3 Part 1 Solution: {} ", day_three.solve_part_one());
    println!("Day 3 Part 2 Solution: {} ", day_three.solve_part_two());

    let day_four = Day4::parse_input(load_datafile("day_4.txt"));
    println!("Day 4 Part 1 Solution: {} ", day_four.solve_part_one());
    println!("Day 4 Part 2 Solution: {} ", day_four.solve_part_two());

    let day_five = Day5::parse_input(load_datafile("day_5.txt"));
    println!("Day 5 Part 1 Solution: {} ", day_five.solve_part_one());
    println!("Day 5 Part 2 Solution: {} ", day_five.solve_part_two());

    let day_six = Day6::parse_input(load_datafile("day_6.txt"));
    println!("Day 6 Part 1 Solution: {} ", day_six.solve_part_one());
    println!("Day 6 Part 2 Solution: {} ", day_six.solve_part_two());
}

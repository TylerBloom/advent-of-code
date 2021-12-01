mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod solution;
mod trailing;

use day_01::Day1;
use day_02::Day2;
use day_03::Day3;
use day_04::Day4;
use day_05::Day5;
use day_06::Day6;
use day_07::Day7;
use solution::Solution;

use std::fs;

fn load_datafile(file: &str) -> String {
    fs::read_to_string(format!("data/{}", file)).unwrap()
}

fn main() {
    let day_one = Day1::parse_input(load_datafile("day_01.txt"));
    println!("Day 1 Part 1 Solution: {} ", day_one.solve_part_one());
    println!("Day 1 Part 2 Solution: {} ", day_one.solve_part_two());

    let day_two = Day2::parse_input(load_datafile("day_02.txt"));
    println!("Day 2 Part 1 Solution: {} ", day_two.solve_part_one());
    println!("Day 2 Part 2 Solution: {} ", day_two.solve_part_two());

    let day_three = Day3::parse_input(load_datafile("day_03.txt"));
    println!("Day 3 Part 1 Solution: {} ", day_three.solve_part_one());
    println!("Day 3 Part 2 Solution: {} ", day_three.solve_part_two());

    let day_four = Day4::parse_input(load_datafile("day_04.txt"));
    println!("Day 4 Part 1 Solution: {} ", day_four.solve_part_one());
    println!("Day 4 Part 2 Solution: {} ", day_four.solve_part_two());

    let day_five = Day5::parse_input(load_datafile("day_05.txt"));
    println!("Day 5 Part 1 Solution: {} ", day_five.solve_part_one());
    println!("Day 5 Part 2 Solution: {} ", day_five.solve_part_two());

    let day_six = Day6::parse_input(load_datafile("day_06.txt"));
    println!("Day 6 Part 1 Solution: {} ", day_six.solve_part_one());
    println!("Day 6 Part 2 Solution: {} ", day_six.solve_part_two());

    let day_seven = Day7::parse_input(load_datafile("day_07.txt"));
    println!("Day 7 Part 1 Solution: {} ", day_seven.solve_part_one());
    println!("Day 7 Part 2 Solution: {} ", day_seven.solve_part_two());
}

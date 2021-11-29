
mod solution;
mod day_1;
mod day_2;

use solution::Solution;
use day_1::Day1;
use day_2::Day2;

use std::fs;

fn load_datafile( file: &str ) -> String {
    fs::read_to_string( format!("data/{}", file) ).unwrap()
}

fn main() {
    let day_one = Day1::parse_input( load_datafile("day_1.txt") );
    println!( "Day 1 Part 1 Solution: {} ", day_one.solve_part_one() );
    println!( "Day 1 Part 2 Solution: {} ", day_one.solve_part_two() );
    
    let day_two = Day2::parse_input( load_datafile("day_2.txt") );
    println!( "Day 2 Part 1 Solution: {} ", day_two.solve_part_one() );
    println!( "Day 2 Part 2 Solution: {} ", day_two.solve_part_two() );
}

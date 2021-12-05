mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod solution;
mod trailing;

use std::fs;

fn load_datafile(file: &str) -> String {
    fs::read_to_string(format!("data/{}", file)).unwrap()
}

fn run_solver(num: u32) {
    let data: String;
    if num < 10 {
        data = load_datafile( &format!("day_0{}.txt", num) );
    } else {
        data = load_datafile( &format!("day_{}.txt", num) );
    }
    match num {
        1 => day_01::run_solution(data),
        2 => day_02::run_solution(data),
        3 => day_03::run_solution(data),
        4 => day_04::run_solution(data),
        _ => panic!("No solution for {} is implemened.", num)
    }
}

fn main() {
    for i in 1..5 {
        run_solver(i);
    }
}

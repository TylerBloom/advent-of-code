mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod solution;
mod trailing;

use std::fs;

fn load_datafile(file: &str) -> String {
    fs::read_to_string(format!("data/{}", file)).unwrap()
}

fn run_solver(num: u32) {
    let data: String;
    if num < 10 {
        data = load_datafile(&format!("day_0{}.txt", num));
    } else {
        data = load_datafile(&format!("day_{}.txt", num));
    }
    match num {
        1 => day_01::run_solution(data),
        2 => day_02::run_solution(data),
        3 => day_03::run_solution(data),
        4 => day_04::run_solution(data),
        5 => day_05::run_solution(data),
        6 => day_06::run_solution(data),
        _ => panic!("No solution for {} is implemened.", num),
    }
}

fn main() {
    for i in 1..7 {
        run_solver(i);
    }
}

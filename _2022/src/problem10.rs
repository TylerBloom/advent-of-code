#![feature(array_chunks)]

use std::{fmt::Write, fs};

fn main() {
    let data = fs::read_to_string("data/problem10.txt").expect("Could not find input file");
    /* ------ Part 1 ------ */
    let mut computer = Computer::new();
    let count: i32 = data
        .lines()
        .map(parse)
        .filter_map(|op| computer.operate_one(op))
        .sum();
    println!("Part 1) The count is: {count:?}");
    /* ------ Part 1 ------ */
    let mut computer = Computer::new();
    let count: Vec<_> = data
        .lines()
        .map(parse)
        .flat_map(|op| computer.operate_two(op).into_iter().filter_map(|b| b))
        .map(|b| if b { '#' } else { '.' })
        .collect();
    let mut answer = String::new();
    for line in count.array_chunks::<40>() {
        writeln!(answer, "{}", line.iter().collect::<String>()).unwrap();
    }
    println!("Part 2) The count is:\n{answer}");
}

struct Computer {
    reg: i32,
    cycle: i32,
    pixel: i32,
}

enum Operation {
    NoOp,
    Addx(i32),
}

fn parse(s: &str) -> Operation {
    match s {
        "noop" => Operation::NoOp,
        _ => Operation::Addx(s.split_at(5).1.parse().unwrap()),
    }
}

impl Computer {
    fn new() -> Self {
        Self {
            reg: 1,
            cycle: 0,
            pixel: 0,
        }
    }

    fn operate_one(&mut self, op: Operation) -> Option<i32> {
        let mut digest = self.tick();
        if let Operation::Addx(n) = op {
            digest = digest.or(self.tick());
            self.reg += n;
        }
        digest
    }

    fn operate_two(&mut self, op: Operation) -> [Option<bool>; 2] {
        let mut digest = [Some((self.reg - self.pixel).abs() < 2), None];
        self.tick();
        if let Operation::Addx(n) = op {
            digest[1] = Some((self.reg - self.pixel).abs() < 2);
            self.tick();
            self.reg += n;
        }
        digest
    }

    fn tick(&mut self) -> Option<i32> {
        self.cycle += 1;
        self.pixel += 1;
        self.pixel %= 40;
        match self.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => Some(self.cycle * self.reg),
            _ => None,
        }
    }
}

use std::{collections::HashSet, fs, str::FromStr};

fn main() {
    let data = fs::read_to_string("data/problem09.txt").expect("Could not find input file");
    let mut buffer_one = HashSet::new();
    let mut rope_one = Rope::<2>::new();
    let mut buffer_two = HashSet::new();
    let mut rope_two = Rope::<10>::new();
    for line in data.lines() {
        let mut iter = line.split_terminator(" ");
        let dir = iter.next().unwrap().parse().unwrap();
        let count: usize = iter.next().unwrap().parse().unwrap();
        for _ in 0..count {
            buffer_one.insert(rope_one.shift(dir));
            buffer_two.insert(rope_two.shift(dir));
        }
    }
    /* ------ Part 1 ------ */
    println!("Part 1) The count is: {}", buffer_one.len());
    /* ------ Part 2 ------ */
    println!("Part 2) The count is: {}", buffer_two.len());
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Rope<const N: usize> {
    knots: [(i32, i32); N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self { knots: [(0, 0); N] }
    }

    fn shift(&mut self, dir: Direction) -> (i32, i32) {
        match dir {
            Direction::Left => {
                self.knots[0].0 -= 1;
            }
            Direction::Right => {
                self.knots[0].0 += 1;
            }
            Direction::Up => {
                self.knots[0].1 += 1;
            }
            Direction::Down => {
                self.knots[0].1 -= 1;
            }
        }
        self.check()
    }

    fn check(&mut self) -> (i32, i32) {
        for i in 1..N {
            let dx = self.knots[i - 1].0 - self.knots[i].0;
            let dy = self.knots[i - 1].1 - self.knots[i].1;
            match (dx, dy) {
                (dx, dy) if dx.abs() < 2 && dy.abs() < 2 => {}
                (dx, dy) if dx.abs() == 2 && dy.abs() < 2 => {
                    self.knots[i].0 += dx/2;
                    self.knots[i].1 += dy;
                }
                (dx, dy) if dy.abs() == 2 && dx.abs() < 2 => {
                    self.knots[i].0 += dx;
                    self.knots[i].1 += dy/2;
                }
                (dx, dy) => {
                    self.knots[i].0 += dx/dx.abs();
                    self.knots[i].1 += dy/dy.abs();
                }
            }
        }
        *self.knots.last().unwrap()
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => unreachable!("Invalid direction!!"),
        }
    }
}

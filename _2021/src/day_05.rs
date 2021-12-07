pub use crate::solution::Solution;

use num::integer;

use std::iter::Iterator;
use std::collections::HashSet;

pub fn run_solution(data: String) {
    let mut solver = Day5::parse_input(data);
    println!("Day 5 Part 1 Solution: {} ", solver.solve_part_one());
    println!("Day 5 Part 2 Solution: {} ", solver.solve_part_two());
}

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(data: &str) -> Self {
        let mut iter = data.split(",").map(|n| n.parse::<i64>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        Point { x, y }
    }
}

struct Line {
    points: HashSet<Point>,
    xs: HashSet<i64>,
    ys: HashSet<i64>,
}

impl Line {
    pub fn new(data: &str) -> Self {
        let parsed: Vec<&str> = data.split(" ").collect();
        let mut p1 = Point::new(parsed[0]);
        let p2 = Point::new(parsed[2]);
        let mut slope = Point { x: p2.x - p1.x, y: p2.y - p1.y };
        let mut gcd = integer::gcd(slope.x, slope.y);
        while gcd != 1 {
            slope.x /= gcd;
            slope.y /= gcd;
            gcd = integer::gcd(slope.x, slope.y);
        }
        let mut points: HashSet<Point> = HashSet::new();
        let mut xs: HashSet<i64> = HashSet::new();
        let mut ys: HashSet<i64> = HashSet::new();
        points.insert(p1);
        xs.insert(p1.x);
        ys.insert(p1.y);
        while p1 != p2 {
            p1.x += slope.x;
            p1.y += slope.y;
            points.insert(p1);
            xs.insert(p1.x);
            ys.insert(p1.y);
        }
        Line { points, xs, ys }
    }
}

pub struct Day5 {
    data: Vec<Line>,
}

impl Solution<u64> for Day5 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<Line> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(Line::new(val));
            }
        }
        Day5 { data }
    }

    fn solve_part_one(&mut self) -> u64 {
        let mut digest: HashSet<Point> = HashSet::new();
        for (i,line) in self.data.iter().enumerate() {
            if line.xs.len() > 1 && line.ys.len() > 1 {
                continue;
            }
            for l in self.data[i+1..].iter() {
                if l.xs.len() > 1 && l.ys.len() > 1 {
                    continue;
                }
                for p in &line.points {
                    if l.points.contains(&p) {
                        digest.insert(*p);
                    }
                }
            }
        }
        digest.len() as u64
    }

    fn solve_part_two(&mut self) -> u64 {
        let mut digest: HashSet<Point> = HashSet::new();
        for (i,line) in self.data.iter().enumerate() {
            for l in self.data[i+1..].iter() {
                for p in &line.points {
                    if l.points.contains(&p) {
                        digest.insert(*p);
                    }
                }
            }
        }
        digest.len() as u64
    }
}

#[cfg(test)]
mod tests {

    //use std::fs::read_to_string;

    use super::{Day5, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2");
        let mut solver = Day5::parse_input(input);
        assert_eq!(solver.solve_part_one(), 5);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2");
        let mut solver = Day5::parse_input(input);
        assert_eq!(solver.solve_part_two(), 12);
    }

    // These will be added after the correct solution for each are is found
    #[test]
    fn known_part_one_solution() {
        //let mut solver = Day5::parse_input(read_to_string("data/day_02.txt").unwrap());
        //assert_eq!(solver.solve_part_one(), 1480518);
    }

    #[test]
    fn known_part_two_solution() {
        //let mut solver = Day5::parse_input(read_to_string("data/day_02.txt").unwrap());
        //assert_eq!(solver.solve_part_two(), 1282809906);
    }
}

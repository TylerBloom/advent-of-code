pub use crate::solution::Solution;

use num::integer;

use std::ops::{Mul,Sub};
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

struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    // Treat this 2D vector like a 3D vector with a z-value of 0
    pub fn cross(&self, other: &Vector) -> f64 {
        self.x*other.y - self.y*other.x
    }
}

impl Mul<i64> for &Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Point {
        Point { x: rhs*self.x, y: rhs*self.y }
    }
}

impl Sub for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Point {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Point {
    pub fn new(data: &str) -> Self {
        let mut iter = data.split(",").map(|n| n.parse::<i64>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        Point { x, y }
    }
    
    pub fn normalized(&self) -> Vector {
        let length = ((self.x*self.x + self.y*self.y) as f64).sqrt();
        if length == 0.0 {
            Vector { x: 0.0, y: 0.0 }
        } else {
            let x = (self.x as f64)/length;
            let y = (self.y as f64)/length;
            Vector { x, y }
        }
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
    slope: Point,
    length: i64,
}

impl Line {
    pub fn new(data: &str) -> Self {
        let parsed: Vec<&str> = data.split(" ").collect();
        let p1 = Point::new(parsed[0]);
        let p2 = Point::new(parsed[2]);
        let mut slope = &p2 - &p1;
        let mut gcd = integer::gcd(slope.x, slope.y);
        while gcd != 1 {
            slope.x /= gcd;
            slope.y /= gcd;
            gcd = integer::gcd(slope.x, slope.y);
        }
        let length: i64;
        if slope.x != 0 {
            length = ((&p2 - &p1).x/slope.x).abs();
        } else {
            length = ((&p2 - &p1).y/slope.y).abs();
        }
        Line { p1, p2, slope, length }
    }
    
    fn contains_point(&self, point: &Point) -> bool {
        let mut digest = true;
        let diff = point - &self.p1;
        if self.slope.x != 0 && self.slope.y != 0 {
            digest &= diff.x / self.slope.x == diff.y / self.slope.y;
        }
        if self.slope.x == 0 {
            digest &= diff.x == 0;
        } else {
            digest &= diff.x / self.slope.x <= self.length; 
            digest &= diff.x * self.slope.x >= 0;
            digest &= diff.x % self.slope.x == 0;
        }
        if self.slope.y == 0 {
            digest &= diff.y == 0;
        } else {
            digest &= diff.y / self.slope.y <= self.length; 
            digest &= diff.y * self.slope.y >= 0;
            digest &= diff.y % self.slope.y == 0;
        }
        digest
    }
    
    // We can use the right-hand rule for this.  By checking only the sign of
    // the cross product, we tell if the given line goes between this line.
    fn crosses(&self, line: &Line) -> bool {
        let rel_vec_1 = (&line.p1 - &self.p1).normalized();
        let rel_vec_2 = (&line.p1 - &self.p2).normalized();
        let rel_vec_3 = (&line.p2 - &self.p1).normalized();
        let rel_vec_4 = (&line.p2 - &self.p2).normalized();
        let prod_one = rel_vec_1.cross(&rel_vec_3);
        let prod_two = rel_vec_2.cross(&rel_vec_4);
        prod_one * prod_two <= 0.0
    }
    
    fn is_parallel(&self, line: &Line) -> bool {
        self.slope == line.slope
            || &self.slope*-1 == line.slope
    }
    
    pub fn add_intersecting_points( &self, line: &Line, points: &mut HashSet<Point> ) {
        if self.crosses(line) {
            let break_early = !self.is_parallel(line);
            let mut point = self.p1.clone();
            while point != self.p2 {
                if line.contains_point(&point) {
                    points.insert(point);
                    if break_early {
                        break;
                    }
                }
                point.x += self.slope.x;
                point.y += self.slope.y;
            }
            if line.contains_point(&point) {
                points.insert(point);
            }
        }
    }
    
    pub fn is_straight(&self) -> bool {
        self.slope.x == 0 || self.slope.y == 0
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
            if !line.is_straight() {
                continue;
            }
            for l in self.data[i+1..].iter() {
                if !l.is_straight() {
                    continue;
                }
                line.add_intersecting_points(l, &mut digest);
            }
        }
        digest.len() as u64
    }

    fn solve_part_two(&mut self) -> u64 {
        let mut digest: HashSet<Point> = HashSet::new();
        for (i,line) in self.data.iter().enumerate() {
            for l in self.data[i+1..].iter() {
                line.add_intersecting_points(l, &mut digest);
            }
        }
        digest.len() as u64
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

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
        let mut solver = Day5::parse_input(read_to_string("data/day_05.txt").unwrap());
        assert_eq!(solver.solve_part_one(), 6856);
    }

    #[test]
    fn known_part_two_solution() {
        let mut solver = Day5::parse_input(read_to_string("data/day_05.txt").unwrap());
        assert_eq!(solver.solve_part_two(), 20666);
    }
}

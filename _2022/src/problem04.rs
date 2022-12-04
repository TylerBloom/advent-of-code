#![feature(iter_array_chunks)]

use std::fs;

fn main() {
    let data: Vec<_> = fs::read_to_string("data/problem04.txt")
        .expect("Could not find part one input file")
        .lines()
        .filter_map(Range::make_pair)
        .collect();
    /* ------ Part 1 ------ */
    let count = data.iter().filter(|(a, b)| Range::contains(a, b)).count();
    println!("Part 1) The count is: {count}");
    /* ------ Part 2 ------ */
    let count = data.iter().filter(|(a, b)| Range::overlap(a, b)).count();
    println!("Part 2) The count is: {count}");
}

struct Range {
    lower: usize,
    upper: usize,
}

impl Range {
    fn make_pair(s: &str) -> Option<(Self, Self)> {
        let mut iter = s.split_terminator(",").filter_map(Range::from_str);
        let first = iter.next()?;
        let second = iter.next()?;
        Some((first, second))
    }

    fn from_str(s: &str) -> Option<Self> {
        let mut iter = s.split_terminator("-").filter_map(|n| n.parse().ok());
        let lower = iter.next()?;
        let upper = iter.next()?;
        Some(Self { lower, upper })
    }

    fn overlap(a: &Self, b: &Self) -> bool {
        (a.lower <= b.lower && a.upper >= b.lower) || (b.lower <= a.lower && b.upper >= a.lower)
    }

    fn contains(a: &Self, b: &Self) -> bool {
        (a.lower >= b.lower && a.upper <= b.upper) || (a.lower <= b.lower && a.upper >= b.upper)
    }
}

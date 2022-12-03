#![feature(iter_array_chunks)]

use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("data/problem03.txt").expect("Could not find input file");
    /* ------ Part 1 ------ */
    let mut buffer_one = HashSet::with_capacity(35);
    let mut buffer_two = HashSet::with_capacity(35);
    let mut part_one = 0;
    for (left, right) in data.lines().map(|l| l.split_at(l.len() / 2)) {
        buffer_one.extend(left.chars());
        buffer_two.extend(right.chars());
        part_one += buffer_one
            .drain()
            .filter(|c| buffer_two.contains(c))
            .map(char_to_val)
            .sum::<i32>();
        buffer_two.clear();
    }
    println!("Part 1) Total score: {part_one}");
    /* ------ Part 2 ------ */
    let mut buffer_three = HashSet::with_capacity(35);
    let mut part_two = 0;
    for [a, b, c] in data.lines().array_chunks() {
        buffer_one.extend(a.chars());
        buffer_two.extend(b.chars());
        buffer_three.extend(c.chars());
        part_two += buffer_one
            .drain()
            .filter(|c| buffer_two.contains(c))
            .filter(|c| buffer_three.contains(c))
            .map(char_to_val)
            .sum::<i32>();
        buffer_two.clear();
        buffer_three.clear();
    }
    println!("Part 2) Total score: {part_two}");
}

// Conversion factor:
// A = 65, needs to become 27
// a = 97, needs to become 1
fn char_to_val(c: char) -> i32 {
    if c.is_uppercase() {
        (c as i32) - 38
    } else if c.is_lowercase() {
        (c as i32) - 96
    } else {
        0
    }
}

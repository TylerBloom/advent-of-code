use std::fs;

fn main() {
    let data =
        fs::read_to_string("data/problem01.txt").expect("Could not find part one input file");
    let mut counts: Vec<i32> = data
        .split("\n\n")
        .map(|list| list.split("\n").filter_map(|i| i.parse::<i32>().ok()).sum())
        .collect();
    counts.sort();
    println!("Part 1) The max count is: {} cal.", counts.last().unwrap());
    let top_three: i32 = counts.iter().rev().take(3).sum();
    println!("Part 1) The top three count is: {top_three} cal.");
}

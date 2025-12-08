use std::{collections::HashMap, hash::Hash, str::Lines};

use crate::Problem;

pub struct DaySeven;

static DATA: &str = include_str!("../input/day_seven.txt");

impl Problem for DaySeven {
    fn problem_a() {
        let solution = problem_a(DATA);
        println!("Problem A's solution is: {solution}");
    }

    fn problem_b() {
        let solution = problem_b(DATA);
        println!("Problem B's solution is: {solution}");
    }
}

fn problem_a(input: &str) -> usize {
    let mut input = input.as_bytes().to_owned();
    let mut lines = input
        .split_mut(|c| *c == b'\n')
        .filter(|line| !line.is_empty());
    let start_index = lines
        .next()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(i, c)| (*c == b'S').then_some(i))
        .unwrap();
    let mut indices = vec![start_index; 1];

    let mut count = 0;
    for line in lines {
        for index in std::mem::take(&mut indices) {
            let c: u8 = line[index];
            match c {
                b'.' => {
                    line[index] = b'|';
                    indices.push(index);
                }
                b'^' => {
                    let mut tmp = 0;
                    if let Some(prior) = index.checked_sub(1) {
                        line[prior] = b'|';
                        indices.push(prior);
                        tmp += 1;
                    }
                    let next = index + 1;
                    if line[next] != b'\n' {
                        line[next] = b'|';
                        indices.push(next);
                        tmp += 1;
                    }
                    if tmp > 0 {
                        count += 1
                    }
                }
                // Ignore (includes newlines)
                _ => {}
            }
        }
    }

    count
}

fn problem_b(input: &str) -> usize {
    fn explore_paths<'a: 'b, 'b>(
        cache: &'b mut HashMap<State<'a>, usize>,
        index: usize,
        mut lines: Lines<'a>,
    ) -> usize {
        let Some(line) = lines.next() else {
            return 1;
        };
        let state = State {
            index,
            lines: lines.clone(),
        };
        if let Some(count) = cache.get(&state) {
            return *count
        }

        match line.chars().nth(index).unwrap() {
            '.' => {
                let digest = explore_paths(cache, index, lines);
                cache.insert(state, digest);
                digest
            }
            '^' => {
                let mut count = 0;
                if let Some(prior) = index.checked_sub(1) {
                    count += explore_paths(cache, prior, lines.clone());
                }
                let next = index + 1;
                if next < line.len() {
                    count += explore_paths(cache, next, lines)
                }
                cache.insert(state, count);
                count
            }
            c => panic!("Found {c} @ {index} in {line:?}"),
        }
    }

    let mut lines = input.lines();
    let start_index = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find_map(|(i, c)| (c == 'S').then_some(i))
        .unwrap();

    let mut cache = HashMap::new();
    explore_paths(&mut cache, start_index, lines)
}

struct State<'a> {
    index: usize,
    lines: Lines<'a>,
}

impl Hash for State<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.lines.clone().for_each(|line| line.hash(state));
    }
}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.index != other.index {
            return false
        }
        let mut this = self.lines.clone();
        let mut other = other.lines.clone();
        if !this.by_ref().zip(other.by_ref()).all(|(this, other)| this == other) {
            return false
        }
        this.next().is_none() && other.next().is_none()
    }
}

impl Eq for State<'_> { }

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT), 21)
    }

    #[test]
    fn test_problem_b_example() {
        assert_eq!(problem_b(INPUT), 40)
    }
}

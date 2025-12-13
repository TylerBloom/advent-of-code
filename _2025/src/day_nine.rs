use crate::Problem;

pub struct DayNine;

static DATA: &str = include_str!("../input/day_nine.txt");

impl Problem for DayNine {
    fn problem_a() {
        let solution = problem_a(DATA);
        println!("Problem A's solution is: {solution}");
    }

    fn problem_b() {
        let solution = problem_b(DATA);
        println!("Problem B's solution is: {solution}");
    }
}

fn length(a: usize, b: usize) -> usize {
    1 + std::cmp::max(a, b) - std::cmp::min(a, b)
}

fn problem_a(input: &str) -> usize {
    let tiles: Vec<(usize, usize)> = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    tiles
        .iter()
        .copied()
        .flat_map(|a| tiles.iter().copied().map(move |b| (a, b)))
        .fold(0, |acc, (a, b)| {
            std::cmp::max(acc, length(a.0, b.0) * length(a.1, b.1))
        })
}

fn problem_b(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT), 50)
    }

    #[test]
    fn test_problem_b_example() {
        assert_eq!(problem_b(INPUT), 25272)
    }
}

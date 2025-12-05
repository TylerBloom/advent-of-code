use crate::Problem;

pub struct DayFive;

static DATA: &str = include_str!("../input/day_five.txt");

impl Problem for DayFive {
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
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(start, end)| start.parse::<usize>().unwrap()..=end.parse().unwrap())
        .collect();

    ids.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn problem_b(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT), 3)
    }

    #[test]
    fn test_problem_b_example() {
        assert_eq!(problem_b(INPUT), 4174379265)
    }
}

use crate::Problem;

pub struct DayThree;

static DATA: &str = include_str!("../input/day_three.txt");

impl Problem for DayThree {
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
    input
        .lines()
        .map(|line| {
            let len = line.len();
            line.chars()
                .take(len - 1)
                .enumerate()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(i, c)| (c, line.chars().skip(i + 1).max().unwrap()))
                .map(|(a, b)| format!("{a}{b}").parse::<usize>().unwrap())
                .unwrap()
        })
        .sum()
}

fn problem_b(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT), 357);
    }

    #[test]
    fn test_problem_b_example() {
        todo!()
    }
}

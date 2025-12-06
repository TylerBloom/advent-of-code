use crate::Problem;

pub struct DaySix;

static DATA: &str = include_str!("../input/day_six.txt");

impl Problem for DaySix {
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
    enum Op {
        Add,
        Mult,
    }

    let mut lines = input.lines();
    let mut symbols: Vec<_> = lines
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|sy| match sy {
            "+" => (Op::Add, 0),
            _ => (Op::Mult, 1),
        })
        .collect();

    lines
        .flat_map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .enumerate()
        })
        .for_each(|(i, num)| match &mut symbols[i] {
            (Op::Add, val) => *val += num,
            (Op::Mult, val) => *val *= num,
        });

    symbols.iter().map(|(_, num)| num).sum()
}

fn problem_b(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT), 4277556)
    }

    #[test]
    fn test_problem_b_example() {
        assert_eq!(problem_b(INPUT), 14)
    }
}

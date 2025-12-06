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

enum Op {
    Add,
    Mult,
}

fn problem_a(input: &str) -> usize {
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

fn problem_b(input: &str) -> usize {
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

    let mut lines: Vec<_> = lines.map(|line| line.chars()).collect();

    let mut index = 0;
    loop {
        let mut str_buffer = String::new();
        let mut null_count = 0;
        for line in lines.iter_mut() {
            let Some(c) = line.next() else {
                null_count += 1;
                continue;
            };
            if c == ' ' {
                continue;
            } else {
                str_buffer.push(c);
            }
        }
        if str_buffer.is_empty() {
            if null_count == lines.len() {
                break
            }
            index += 1;
        } else {
            let num = str_buffer.parse::<usize>().unwrap();
            match &mut symbols[index] {
                (Op::Add, val) => *val += num,
                (Op::Mult, val) => *val *= num,
            }
        }
    }
    symbols.iter().map(|(_, num)| num).sum()
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
        assert_eq!(problem_b(INPUT), 3263827)
    }
}

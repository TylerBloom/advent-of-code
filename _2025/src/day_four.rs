use crate::Problem;

pub struct DayFour;

static DATA: &str = include_str!("../input/day_four.txt");

impl Problem for DayFour {
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
    const ADJS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
    ];

    let input: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    // Find all the indices of the rolls of paper
    let iter = input.iter().enumerate().flat_map(|(col, line)| {
        line.iter()
            .enumerate()
            .filter_map(move |(row, item)| item.then_some((col, row)))
    });

    let mut digest = 0;
    for (col, row) in iter {
        let mut count = 0;
        for offset in ADJS {
            let Some(col) = col.checked_add_signed(offset.0) else {
                continue;
            };
            let Some(r) = input.get(col) else {
                continue;
            };
            let Some(row) = row.checked_add_signed(offset.1) else {
                continue;
            };
            let Some(value) = r.get(row) else {
                continue;
            };
            count += (*value) as usize;
        }

        if count < 4 {
            digest += 1;
        }
    }

    digest
}

fn problem_b(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT), 13)
    }

    #[test]
    fn test_problem_b_example() {
        assert_eq!(problem_b(INPUT), 4174379265)
    }
}

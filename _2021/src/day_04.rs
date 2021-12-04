pub use crate::solution::Solution;

struct Bingo {
    val_grid: Vec<Vec<u64>>,
    check_grid: Vec<Vec<bool>>,
}

impl Bingo {
    pub fn new(vals: Vec<Vec<u64>>) -> Self {
        let len = vals.len();
        let grid = vec![vec!(false; len); len];
        Bingo {
            val_grid: vals,
            check_grid: grid,
        }
    }

    pub fn reset(&mut self) {
        let len = self.check_grid.len();
        self.check_grid = vec![vec!(false; len); len];
    }

    pub fn add_val(&mut self, val: u64) -> Option<(usize, usize)> {
        let mut digest: Option<(usize, usize)> = None;
        for (i, row) in self.val_grid.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if v == &val {
                    self.check_grid[i][j] = true;
                    digest = Some((i, j));
                    break;
                }
            }
            if digest.is_some() {
                break;
            }
        }
        digest
    }

    pub fn check_row(&self, i: usize) -> bool {
        self.check_grid[i].iter().all(|&v| v)
    }

    pub fn check_col(&self, j: usize) -> bool {
        self.check_grid.iter().all(|v| v[j])
    }

    pub fn sum_unmarked(&self) -> u64 {
        let len = self.check_grid.len();
        self.val_grid
            .iter()
            .flatten()
            .enumerate()
            .filter(|(i, _)| !self.check_grid[i / len][i % len])
            .map(|(_, v)| v)
            .sum()
    }
}

pub struct Day4 {
    nums: Vec<u64>,
    data: Vec<Bingo>,
}

impl Day4 {
    pub fn reset_bingo(&mut self) {
        for bingo in &mut self.data {
            bingo.reset();
        }
    }
}

impl Solution<u64> for Day4 {
    fn parse_input(input: String) -> Self {
        let mut parsed: Vec<&str> = input.split("\n\n").collect();
        let nums: Vec<u64> = parsed[0]
            .split(",")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        parsed.remove(0);
        let mut data: Vec<Bingo> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(Bingo::new(
                    val.split("\n")
                        .map(|row| {
                            row.split(" ")
                                .filter(|v| !v.is_empty())
                                .map(|v| v.parse::<u64>().unwrap())
                                .collect::<Vec<u64>>()
                        })
                        .collect(),
                ));
            }
        }
        Day4 { nums, data }
    }

    fn solve_part_one(&mut self) -> u64 {
        let mut digest: u64 = 0;
        for num in &self.nums {
            for bingo in &mut self.data {
                if let Some((i, j)) = bingo.add_val(*num) {
                    if bingo.check_row(i) || bingo.check_col(j) {
                        digest = bingo.sum_unmarked() * num;
                        break;
                    }
                }
            }
            if digest != 0 {
                break;
            }
        }
        digest
    }

    fn solve_part_two(&mut self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::{Day4, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7");
        let mut solver = Day4::parse_input(input);
        assert_eq!(solver.solve_part_one(), 4512);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7");
        let mut solver = Day4::parse_input(input);
        assert_eq!(solver.solve_part_one(), 4512);
    }
}

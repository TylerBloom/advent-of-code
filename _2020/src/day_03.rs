pub use crate::solution::Solution;

#[derive(Debug, Clone, Copy)]
pub enum Obstacle {
    Tree,
    Clear,
}

pub struct Day3 {
    data: Vec<Vec<Obstacle>>,
}

impl Day3 {
    fn get(&self, i: usize, j: usize) -> Option<Obstacle> {
        if self.data.len() <= i {
            None
        } else {
            Some(self.data[i][j % self.data[0].len()])
        }
    }

    pub fn trace_slope(&self, down: usize, right: usize) -> u64 {
        let mut digest: u64 = 0;
        let mut row: usize = 0;
        let mut col: usize = 0;
        while let Some(val) = self.get(row, col) {
            match val {
                Obstacle::Tree => {
                    digest += 1;
                }
                Obstacle::Clear => {}
            }
            row += down;
            col += right;
        }
        digest
    }
}

impl Solution<u64> for Day3 {
    fn parse_input(input: String) -> Self {
        let parsed: Vec<&str> = input.split("\n").collect();
        let mut data: Vec<Vec<Obstacle>> = Vec::with_capacity(parsed.len());
        for val in parsed {
            if !val.is_empty() {
                data.push(
                    val.chars()
                        .map(|c| {
                            if c == '#' {
                                Obstacle::Tree
                            } else {
                                Obstacle::Clear
                            }
                        })
                        .collect(),
                );
            }
        }
        Day3 { data }
    }

    fn solve_part_one(&self) -> u64 {
        self.trace_slope(1, 3)
    }

    fn solve_part_two(&self) -> u64 {
        let mut digest: u64 = 1;
        digest *= self.trace_slope(1, 1);
        digest *= self.trace_slope(1, 3);
        digest *= self.trace_slope(1, 5);
        digest *= self.trace_slope(1, 7);
        digest *= self.trace_slope(2, 1);
        digest
    }
}

#[cfg(test)]
mod tests {

    use super::{Day3, Solution};

    #[test]
    fn test_part_one() {
        let input = String::from("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#");
        let solver = Day3::parse_input(input);
        assert_eq!(solver.solve_part_one(), 7);
    }

    #[test]
    fn test_part_two() {
        let input = String::from("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#");
        let solver = Day3::parse_input(input);
        assert_eq!(solver.solve_part_two(), 336);
    }
}

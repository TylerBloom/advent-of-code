use std::{fs, str::FromStr};

fn main() {
    let cargo: Cargo = fs::read_to_string("data/problem05_cargo.txt")
        .expect("Could not find cargo input file")
        .parse()
        .unwrap();
    let moves = fs::read_to_string("data/problem05_moves.txt")
        .expect("Could not find moves input file")
        .parse()
        .unwrap();
    /* ------ Part 1 ------ */
    let mut cargo_one = cargo.clone();
    cargo_one.moves(&moves, true);
    println!(
        "Part 1) The tops of the stacks after moving are: {}",
        cargo_one.tops()
    );
    /* ------ Part 2 ------ */
    let mut cargo_two = cargo.clone();
    cargo_two.moves(&moves, false);
    println!(
        "Part 1) The tops of the stacks after moving are: {}",
        cargo_two.tops()
    );
}

#[derive(Clone)]
struct Cargo {
    stacks: Vec<Vec<char>>,
}

struct Moves {
    moves: Vec<(usize, usize, usize)>,
}

impl Cargo {
    fn tops(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }

    fn len(&self) -> usize {
        self.stacks.iter().map(|v| v.len()).sum()
    }

    fn moves(&mut self, moves: &Moves, rev: bool) {
        let mut buffer = Vec::with_capacity(10);
        let start = self.len();
        for (count, first, second) in moves.moves.iter() {
            let index = self.stacks[*first].len() - count;
            match rev {
                true => {
                    buffer.extend(self.stacks[*first].drain(index..).rev());
                }
                false => {
                    buffer.extend(self.stacks[*first].drain(index..));
                }
            }
            self.stacks[*second].extend(buffer.drain(0..));
            assert_eq!(start, self.len());
        }
    }
}

impl FromStr for Cargo {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks = s
            .lines()
            .map(|l| {
                l.split_terminator(" ")
                    .map(|c| c.parse().unwrap())
                    .collect()
            })
            .collect();
        Ok(Self { stacks })
    }
}

impl FromStr for Moves {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .lines()
            .map(|l| {
                let mut iter = l.split_terminator(" ").filter_map(|c| c.parse().ok());
                let a = iter.next().unwrap();
                let b = iter.next().unwrap() - 1;
                let c = iter.next().unwrap() - 1;
                (a, b, c)
            })
            .collect();
        Ok(Self { moves })
    }
}

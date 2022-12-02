use std::{fs, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

fn main() {
    let input = fs::read_to_string("data/problem02.txt").expect("Could not find input file");
    let data: Vec<_> = input
        .split("\n")
        .filter_map(|s| (s.len() == 3).then(|| s.split_at(1)))
        .collect();
    let score: usize = data.iter().filter_map(parse).map(points).sum();
    println!("Part 1) The points are {score}");
    let score: usize = data.iter().filter_map(parse_two).map(points).sum();
    println!("Part 1) The real points are {score}");
}

fn points((opp, me): (RPS, RPS)) -> usize {
    let mut digest = match me {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    };

    match (me, opp) {
        (RPS::Rock, RPS::Scissor) | (RPS::Paper, RPS::Rock) | (RPS::Scissor, RPS::Paper) => {
            digest += 6;
        }
        (a, b) if a == b => {
            digest += 3;
        }
        _ => {}
    }

    digest
}

fn parse((opp, me): &(&str, &str)) -> Option<(RPS, RPS)> {
    match (RPS::from_str(opp.trim()), RPS::from_str(me.trim())) {
        (Ok(a), Ok(b)) => Some((a, b)),
        _ => None,
    }
}

fn parse_two((opp, me): &(&str, &str)) -> Option<(RPS, RPS)> {
    let opp = RPS::from_str(opp).ok()?;
    match me.trim() {
        "X" => Some((opp, opp.loss())),
        "Y" => Some((opp, opp.draw())),
        "Z" => Some((opp, opp.win())),
        _ => None,
    }
}

impl RPS {
    fn win(&self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissor,
            RPS::Scissor => RPS::Rock,
        }
    }
    
    fn loss(&self) -> Self {
        match self {
            RPS::Rock => RPS::Scissor,
            RPS::Paper => RPS::Rock,
            RPS::Scissor => RPS::Paper,
        }
    }
    
    fn draw(&self) -> Self {
        *self
    }
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissor),
            _ => Err(()),
        }
    }
}

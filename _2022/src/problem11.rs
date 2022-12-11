use std::{fs, str::FromStr};

fn get_top_two(monkeys: &Vec<Monkey>) -> usize {
    monkeys
        .iter()
        .enumerate()
        .flat_map(|(i, m)| {
            monkeys
                .iter()
                .skip(i + 1)
                .map(|mm| m.counter.clone() * mm.counter.clone())
        })
        .max_by(|m, n| Ord::cmp(&m, &n))
        .unwrap()
}

fn process(monkeys: &mut Vec<Monkey>, n: usize, div: usize, rem: usize) {
    let len = monkeys.len();
    for i in (0..n).flat_map(|_| 0..len) {
        for (index, item) in monkeys[i].take_turn(div, rem) {
            monkeys[index].add_item(item);
        }
    }
}

fn main() {
    let data = fs::read_to_string("data/problem11.txt").expect("Could not find input file");
    /* ------ Part 1 ------ */
    let mut monkeys: Vec<Monkey> = data
        .split_terminator("\n\n")
        .map(|m| m.parse().unwrap())
        .collect();
    let rem = monkeys
        .iter()
        .map(|m| m.check)
        .reduce(|acc, n| acc * n)
        .unwrap();
    process(&mut monkeys, 20, 3, rem);
    println!("Part 1) The max count is: {}", get_top_two(&monkeys));
    /* ------ Part 2 ------ */
    let mut monkeys: Vec<Monkey> = data
        .split_terminator("\n\n")
        .map(|m| m.parse().unwrap())
        .collect();
    process(&mut monkeys, 10_000, 1, rem);
    println!("Part 2) The max count is: {}", get_top_two(&monkeys));
}

struct Monkey {
    counter: usize,
    check: usize,
    items: Vec<usize>,
    op: Box<dyn Fn(&mut usize) -> ()>,
    throw: Box<dyn Fn(usize) -> usize>,
}

impl Monkey {
    fn take_turn(&mut self, div: usize, rem: usize) -> Vec<(usize, usize)> {
        self.items
            .drain(0..)
            .map(|mut val| {
                self.counter += 1usize;
                (self.op)(&mut val);
                val /= div;
                val %= rem;
                ((self.throw)(val), val)
            })
            .collect()
    }

    fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines().map(|s| s.trim()).skip(1);
        let items = iter
            .next()
            .unwrap()
            .split_at(16)
            .1
            .split_terminator(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let op_and_val = iter.next().unwrap().split_at(21).1.split_at(1);
        let op: Box<dyn Fn(&mut usize) -> ()> =
            match (op_and_val.0, op_and_val.1.trim().parse::<usize>()) {
                ("+", Ok(val)) => Box::new(move |n| *n += val),
                ("+", Err(_)) => Box::new(|n| *n += *n),
                ("*", Ok(val)) => Box::new(move |n| *n *= val),
                ("*", Err(_)) => Box::new(|n| *n *= *n),
                _ => unreachable!("How did you get here?"),
            };
        let get_word = |s: &str, n: usize| {
            s.split_terminator(" ")
                .skip(n)
                .next()
                .unwrap()
                .parse()
                .unwrap()
        };
        let div = get_word(iter.next().unwrap(), 3);
        let t_path = get_word(iter.next().unwrap(), 5);
        let f_path = get_word(iter.next().unwrap(), 5);
        let throw = Box::new(move |n: usize| if n % div == 0 { t_path } else { f_path });
        Ok(Self {
            counter: 0,
            check: div,
            items,
            op,
            throw,
        })
    }
}

use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

use crate::Problem;

pub struct DayEight;

static DATA: &str = include_str!("../input/day_eight.txt");

impl Problem for DayEight {
    fn problem_a() {
        let solution = problem_a(DATA, 1000);
        println!("Problem A's solution is: {solution}");
    }

    fn problem_b() {
        let solution = problem_b(DATA);
        println!("Problem B's solution is: {solution}");
    }
}

type Curcuit = HashSet<Point>;

#[derive(Hash, Default, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Point(isize, isize, isize);

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:0>3}, {:0>3}, {:0>3})", self.0, self.1, self.2)
    }
}

impl Point {
    fn eulidian_distance(self, b: &Self) -> isize {
        let Point(a_x, a_y, a_z) = self;
        let Point(b_x, b_y, b_z) = b;
        let digest = (a_x - b_x).pow(2) + (a_y - b_y).pow(2) + (a_z - b_z).pow(2);
        digest.isqrt()
    }
}

#[allow(clippy::type_complexity)]
fn setup(
    input: &str,
) -> (
    Vec<Point>,
    Vec<(Point, Point, isize)>,
    HashMap<Point, HashSet<Point>>,
    HashMap<Point, Rc<Cell<HashSet<Point>>>>,
) {
    let boxes: Vec<_> = input
        .lines()
        .map(|line| {
            let mut line = line.split(',').map(str::parse).map(Result::unwrap);
            Point(
                line.next().unwrap(),
                line.next().unwrap(),
                line.next().unwrap(),
            )
        })
        .collect();
    let mut mins = Vec::with_capacity(2 * boxes.len());
    let mut iter = boxes.iter();

    while let Some(&point) = iter.next() {
        mins.extend(
            iter.as_slice()
                .iter()
                .map(|b| (point, *b, point.eulidian_distance(b))),
        );
    }
    mins.sort_by_key(|(_, _, dist)| *dist);

    let direct_connects = boxes
        .iter()
        .copied()
        .zip(std::iter::repeat_with(HashSet::<Point>::new))
        .collect();

    // We are using Cell here for fun and a bit of safety. Because the default values for HashSets
    // (and most std collections) don't allocate on construction, the value can be taken and
    // reinserted from/into the cell with little cost.
    let curcuits = boxes
        .iter()
        .copied()
        .map(|point| {
            (
                point,
                Rc::new(Cell::new(HashSet::from_iter(std::iter::once(point)))),
            )
        })
        .collect();
    (boxes, mins, direct_connects, curcuits)
}

fn problem_a(input: &str, take: usize) -> usize {
    let (_boxes, mins, mut direct_connects, mut curcuits) = setup(input);

    let mut count = 0;

    for (a, b, _dist) in mins {
        if count == take {
            break;
        }

        let conn = direct_connects.get_mut(&a).unwrap();
        if conn.contains(&b) {
            continue;
        }
        conn.insert(b);

        let c_a: Rc<Cell<Curcuit>> = Rc::clone(curcuits.get(&a).unwrap());
        let mut curcuit_a: Curcuit = c_a.take();

        count += 1;
        let curcuit_b: Curcuit = curcuits.get(&b).unwrap().take();
        // NOTE: It is not enough to have B's curcuit point at the (soon to be) adjoined
        // curcuit. Rather, all nodes in B's curcuit also need to point A's curcuit.
        for point in curcuit_b.iter() {
            *curcuits.get_mut(point).unwrap() = Rc::clone(&c_a);
        }
        curcuit_a.extend(curcuit_b);
        c_a.replace(curcuit_a);
    }

    let mut digest: Vec<Curcuit> = curcuits
        .into_values()
        .map(|c| c.take())
        .filter(|curcuit| !curcuit.is_empty())
        .collect();
    digest.sort_by_key(|a| a.len());
    digest.into_iter().rev().take(3).map(|c| c.len()).product()
}

fn problem_b(input: &str) -> usize {
    let (boxes, mins, mut direct_connects, mut curcuits) = setup(input);

    let mut digest = 0;

    for (a, b, _dist) in mins {
        let conn = direct_connects.get_mut(&a).unwrap();
        if conn.contains(&b) {
            continue;
        }
        conn.insert(b);

        let c_a: Rc<Cell<Curcuit>> = Rc::clone(curcuits.get(&a).unwrap());
        let mut curcuit_a: Curcuit = c_a.take();

        let curcuit_b: Curcuit = curcuits.get(&b).unwrap().take();
        // NOTE: It is not enough to have B's curcuit point at the (soon to be) adjoined
        // curcuit. Rather, all nodes in B's curcuit also need to point A's curcuit.
        for point in curcuit_b.iter() {
            *curcuits.get_mut(point).unwrap() = Rc::clone(&c_a);
        }
        curcuit_a.extend(curcuit_b);
        if curcuit_a.len() == boxes.len() {
            digest = a.0 * b.0;
            break;
        }
        c_a.replace(curcuit_a);
    }

    digest as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT, 10), 40)
    }

    #[test]
    fn test_problem_b_example() {
        assert_eq!(problem_b(INPUT), 25272)
    }
}

use crate::Problem;

pub struct DayTwo;

static DATA: &str = include_str!("../input/day_two.txt");

impl Problem for DayTwo {
    fn problem_a() {
        let solution = problem_a(DATA);
        println!("Problem A's solution is: {solution}");
    }

    fn problem_b() {
        let solution = problem_b(DATA);
        println!("Problem B's solution is: {solution}");
    }
}

fn construct_iter(input: &str) -> impl Iterator<Item = usize> {
    input
        .trim()
        .split(',')
        .map(|r| r.split_once('-').unwrap())
        .flat_map(|(a, b)| {
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            a..=b
        })
}

fn problem_a(input: &str) -> usize {
    fn find_false_ids(id: String) -> bool {
        let len = id.len() / 2;
        for i in 1..=len {
            // This should not panic as the ID only uses numeric chars, which are all ASCII (i.e.
            // one byte.
            let (a, b) = id.split_at(i);

            // An invalid ID, like 6464, is just repeated strings. When "6464" is split at "64",
            // the returned items are all empty strings
            //                                  v There is always a trailing empty sub-string
            if b.split(a).all(str::is_empty) && b.split(a).count() == 2 {
                return true;
            }
        }
        false
    }

    construct_iter(input)
        .filter(|i| find_false_ids(i.to_string()))
        .sum()
}

fn problem_b(input: &str) -> usize {
    fn find_false_ids(id: String) -> bool {
        let len = id.len() / 2;
        for i in 1..=len {
            // This should not panic as the ID only uses numeric chars, which are all ASCII (i.e.
            // one byte.
            let (a, b) = id.split_at(i);

            // An invalid ID, like 6464, is just repeated strings. When "6464" is split at "64",
            // the returned items are all empty strings
            //                                  v There is always a trailing empty sub-string
            if b.split(a).all(str::is_empty) {
                return true;
            }
        }
        false
    }

    construct_iter(input)
        .filter(|i| find_false_ids(i.to_string()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT), 1227775554)
    }

    #[test]
    fn test_problem_b_example() {
        assert_eq!(problem_b(INPUT), 4174379265)
    }
}

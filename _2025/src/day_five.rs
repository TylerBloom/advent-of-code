use crate::Problem;

pub struct DayFive;

static DATA: &str = include_str!("../input/day_five.txt");

impl Problem for DayFive {
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
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(start, end)| start.parse::<usize>().unwrap()..=end.parse().unwrap())
        .collect();

    ids.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn problem_b(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<_> = ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(start, end)| start.parse::<usize>().unwrap()..=end.parse().unwrap())
        .collect();

    let mut buffer = Vec::with_capacity(ranges.len());

    // We pop ranges off and compare that range to all other ranges, reducing the popped range
    // until it is null or disjoint. Then, that range is placed into a new buffer.
    //
    // Once reduced, the popped ranges do not need to be checked against as they are disjount for
    // all other ranges and newly popped-ranges are only ever reduced, not expanded.
    while let Some(range) = ranges.pop() {
        let (mut start, mut end) = range.into_inner();
        for range in ranges.iter_mut() {
            match (range.contains(&start), range.contains(&end)) {
                // There is some overlap, which we need to address
                (true, false) => {
                    start = *range.end() + 1;
                }
                (false, true) => {
                    end = *range.start() - 1;
                }
                // The new range is fully enveloped by the other range. Discard it
                (true, true) => {
                    start = usize::MAX;
                    end = usize::MIN;
                    break;
                }
                // The new range is either disjoint or envelopes the old range. No action needed
                // for the disjoint case. In the other, we swap the enveloped range with the popped
                // range and break. The swap puts us in the prior scenario. This is safe because we
                // know that all popped ranges and the checked ranges in this loop are disjoint
                // from the enveloping range. Thus, the enveloped range must also be disjoint from
                // them.
                (false, false) => {
                    let r = start..=end;
                    if r.contains(range.start()) && r.contains(range.end()) {
                        *range = r;
                        start = usize::MAX;
                        end = usize::MIN;
                        break;
                    }
                }
            }
        }
        if start <= end {
            buffer.push(start..=end);
        }
    }

    buffer
        .into_iter()
        // Range::count doesn't iterate through all values
        .map(Iterator::count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_problem_a_example() {
        assert_eq!(problem_a(INPUT), 3)
    }

    #[test]
    fn test_problem_b_example() {
        assert_eq!(problem_b(INPUT), 14)
    }
}

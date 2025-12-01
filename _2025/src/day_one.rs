use crate::Problem;

pub struct DayOne;

static DATA: &str = include_str!("../input/day_one_a.txt");

impl Problem for DayOne {
    fn problem_a() {
        let (pw, _) = DATA
            .lines()
            .fold((0_i32, 50_i32), |(mut pw, mut acc), line| {
                let mut chars = line.trim().chars();
                match chars.next().unwrap() {
                    'L' => acc -= chars.as_str().parse::<i32>().unwrap(),
                    'R' => acc += chars.as_str().parse::<i32>().unwrap(),
                    c => panic!("Unknown starting char: {c:?}"),
                }
                acc = acc.rem_euclid(100);
                pw += (acc == 0) as i32;
                (pw, acc)
            });
        println!("The password is: {pw}");
    }

    fn problem_b() {
        let pw = problem_b(DATA);
        println!("The password is: {pw}");
    }
}

fn problem_b(input: &str) -> i32 {
    let (pw, _) = input
        .lines()
        .flat_map(|line| {
            let mut chars = line.trim().chars();
            match chars.next().unwrap() {
                'L' => std::iter::repeat_n(-1, chars.as_str().parse::<usize>().unwrap()),
                'R' => std::iter::repeat_n(1, chars.as_str().parse::<usize>().unwrap()),
                c => panic!("Unknown starting char: {c:?}"),
            }
        })
        .fold((0_i32, 50_i32), |(mut pw, mut acc), val| {
            acc += val;
            if acc == -1 {
                acc = 99;
            } else if acc == 100 {
                acc = 0;
            }
            if acc == 0 {
                pw += 1;
            }
            (pw, acc)
        });
    pw
}

#[cfg(test)]
mod tests {
    use crate::day_one::problem_b;

    #[test]
    fn test_problem_b_simple() {
        static INPUT: &str = "L49
L1
R1
L3";
        let pw = problem_b(INPUT);
        assert_eq!(pw, 2);
    }

    #[test]
    fn test_problem_b() {
        static INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
R1000
L1000";
        let pw = problem_b(INPUT);
        assert_eq!(pw, 26);
    }
}

mod day_one;

fn main() {
    let mut args = std::env::args();

    // First arg is the path
    args.next();

    // Second arg is user input
    match args
        .next()
        .and_then(|arg| arg.parse::<usize>().ok())
        .expect("First argument must be a number between 1 and 12")
    {
        1 => day_one::DayOne::run(args.next()),
        2 => todo!("Day 2 is not impl-ed yet"),
        3 => todo!("Day 3 is not impl-ed yet"),
        4 => todo!("Day 4 is not impl-ed yet"),
        5 => todo!("Day 5 is not impl-ed yet"),
        6 => todo!("Day 6 is not impl-ed yet"),
        7 => todo!("Day 7 is not impl-ed yet"),
        8 => todo!("Day 8 is not impl-ed yet"),
        9 => todo!("Day 9 is not impl-ed yet"),
        10 => todo!("Day 10 is not impl-ed yet"),
        11 => todo!("Day 11 is not impl-ed yet"),
        12 => todo!("Day 12 is not impl-ed yet"),
        _ => panic!(
            "There are only a dozen puzzles this year. Please enter a number between 1 and 12"
        ),
    }
}

pub trait Problem {
    fn run(problem: Option<String>) {
        match problem.as_deref() {
            Some("A" | "a") => Self::problem_a(),
            Some("B" | "b") => Self::problem_b(),
            _ => panic!("Second argument must be either A or B"),
        }
    }

    fn problem_a();
    fn problem_b();
}

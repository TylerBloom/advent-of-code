pub trait Solution<T> {
    fn parse_input(input: String) -> Self;
    fn solve_part_one(&self) -> T;
    fn solve_part_two(&self) -> T;
}

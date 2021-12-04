pub trait Solution<T> {
    fn parse_input(input: String) -> Self;
    fn solve_part_one(&mut self) -> T;
    fn solve_part_two(&mut self) -> T;
}

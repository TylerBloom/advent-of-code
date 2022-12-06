use std::fs;

fn main() {
    let data =
        fs::read_to_string("data/problem06.txt").expect("Could not find part one input file");
    let mut buffer = Buffer::<4>::new();
    let _: String = data.chars().take_while(|c| buffer.insert(*c)).collect();
    println!("Part 1) The processed chars are: {}", buffer.counter);
    let mut buffer = Buffer::<14>::new();
    let _: String = data.chars().take_while(|c| buffer.insert(*c)).collect();
    println!("Part 2) The processed chars are: {}", buffer.counter);
}

struct Buffer<const N: usize> {
    buf: [char; N],
    counter: usize,
}

impl<const N: usize> Buffer<N> {
    fn new() -> Self {
        Self {
            buf: [' '; N],
            counter: 0,
        }
    }

    fn insert(&mut self, c: char) -> bool {
        self.buf[self.counter % N] = c;
        self.counter += 1;
        !self.check_buf()
    }

    /// Return true when the buffer should is valid
    fn check_buf(&self) -> bool {
        let mut digest = !self.buf[..].contains(&' ');
        for i in 0..(N - 1) {
            digest &= !self.buf[(i + 1)..].contains(&self.buf[i])
        }
        digest
    }
}

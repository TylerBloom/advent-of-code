#[derive(Clone)]
pub struct Trailing<I>
where
    I: Iterator + Clone,
{
    first: I,
    second: I,
}

impl<I> Trailing<I>
where
    I: Iterator + Clone,
{
    #[allow(dead_code)]
    pub fn with_gap(mut self, count: usize ) -> Self {
        for _ in 1..count {
            self.first.next();
        }
        self
    }
}

impl<I> Iterator for Trailing<I>
where
    I: Iterator + Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let val = self.first.next();
        match val {
            Some(v) => Some((v, self.second.next().unwrap())),
            None => None,
        }
    }
}

pub trait TrailingIterator
where
    Self: Iterator + Clone + Sized,
{
    fn trailing(self) -> Trailing<Self>;
}

impl<Iter> TrailingIterator for Iter
where
    Iter: Iterator + Clone,
{
    fn trailing(self) -> Trailing<Self> {
        let mut f = self.clone();
        f.next();
        Trailing {
            first: f,
            second: self,
        }
    }
}

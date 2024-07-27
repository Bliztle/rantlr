pub trait IteratorExt: Iterator {
    fn ensure(self, pred: P) -> Result<Self>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> Result<()>;
}

impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn ensure<P>(self, pred: P) -> LimitCount<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        LimitCount { iter: self, pred }
    }
}

pub struct Ensure<I, P> {
    iter: I,
    pred: P,
}

impl<I, P> Iterator for LimitCount<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            if (self.pred)(&item) {
                self.count += 1;
                if self.count > self.limit {
                    return None;
                }
            }
            return Some(item);
        };
        None
    }
}

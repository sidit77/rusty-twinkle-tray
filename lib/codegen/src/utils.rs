pub trait IterExpect<T> {
    fn continue_if(self, func: impl FnOnce(T) -> bool) -> Option<Self> where Self: Sized;
}

impl<T, I: Iterator<Item=T>> IterExpect<T> for I {
    fn continue_if(mut self, func: impl FnOnce(T) -> bool) -> Option<Self> where Self: Sized {
        self.next().and_then(|i| func(i).then_some(self))
    }
}
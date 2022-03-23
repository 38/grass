
pub trait Scored<T> {
    fn score(&self) -> Option<T>;
}

impl<S, T: Scored<S>> Scored<S> for Option<T> {
    fn score(&self) -> Option<S> {
        self.as_ref().map(|inner| inner.score()).flatten()
    }
}

impl<S, A: Scored<S>, B> Scored<S> for (A, B) {
    fn score(&self) -> Option<S> {
        self.0.score()
    }
}

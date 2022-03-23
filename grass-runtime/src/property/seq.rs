
pub enum Nuclide {
    A,
    T,
    C,
    G,
    U,
    N,
}

pub trait WithSequence {
    type RangeType: IntoIterator<Item = Nuclide>;
    fn at(&self, offset: usize) -> Nuclide;
    fn range(&self, from: usize, to: usize) -> Self::RangeType;
}
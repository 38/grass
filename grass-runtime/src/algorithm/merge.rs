use std::iter::Peekable;

use crate::{property::Region, record::{Bed3, CastTo, CastIter}};

use super::Sorted;

pub struct TwoWayMerge<IA, IB, R>
where
    IA: Iterator<Item = R> + Sorted,
    IB: Iterator<Item = R> + Sorted,
    R: Region,
{
    iter_a: Peekable<IA>,
    iter_b: Peekable<IB>,
}

impl<IA, IB, R> Sorted for TwoWayMerge<IA, IB, R>
where
    IA: Iterator<Item = R> + Sorted,
    IB: Iterator<Item = R> + Sorted,
    R: Region,
{
}

impl<IA, IB, R> Iterator for TwoWayMerge<IA, IB, R>
where
    IA: Iterator<Item = R> + Sorted,
    IB: Iterator<Item = R> + Sorted,
    R: Region,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter_a.peek(), self.iter_b.peek()) {
            (Some(a), Some(b)) if Bed3::new(a) <= Bed3::new(b) => self.iter_a.next(),
            (Some(_), Some(_)) => self.iter_b.next(),
            (_, None) => self.iter_a.next(),
            (None, _) => self.iter_b.next(),
        }
    }
}

pub trait TwoWayMergeExt
where
    Self: Iterator + Sorted + Sized,
    Self::Item: Region,
{
    fn merge_with<T>(self, other: T) -> TwoWayMerge<Self, CastIter<T, Self::Item>, Self::Item>
    where
        T: Iterator + Sorted + Sized,
        T::Item : CastTo<Self::Item>,
    {
        TwoWayMerge {
            iter_a: self.peekable(),
            iter_b: CastIter::cast(other).peekable(),
        }
    }
}

impl<T> TwoWayMergeExt for T
where
    T: Iterator + Sorted + Sized,
    T::Item: Region,
{
}

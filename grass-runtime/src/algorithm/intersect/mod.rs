mod heap;
mod inner;
mod outer;

use crate::property::Region;
use crate::{algorithm::markers::Sorted, record::ToSelfContained};

use inner::{Context, State};

pub use inner::SortedIntersectIter;

pub struct ToSelfContainedIter<T: Iterator>
where
    T::Item: ToSelfContained,
{
    inner: T,
}

impl<T: Iterator> Iterator for ToSelfContainedIter<T>
where
    T::Item: ToSelfContained,
{
    type Item = <T::Item as ToSelfContained>::SelfContained;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|what| what.to_self_contained())
    }
}

impl<T: Sorted> Sorted for ToSelfContainedIter<T> where T::Item: ToSelfContained {}

pub trait SortedIntersect: Iterator + Sorted + Sized {
    fn sorted_intersect<U, Other: Iterator<Item = U> + Sorted>(
        self,
        other: Other,
    ) -> SortedIntersectIter<ToSelfContainedIter<Self>, ToSelfContainedIter<Other>>
    where
        Self::Item: ToSelfContained,
        <Self::Item as ToSelfContained>::SelfContained: Clone + Region,
        U: ToSelfContained,
        U::SelfContained: Clone + Region,
    {
        SortedIntersectIter {
            context_a: Context::from_iter(ToSelfContainedIter { inner: self }),
            context_b: Context::from_iter(ToSelfContainedIter { inner: other }),
            state: State::FrontierA(0, 0, None),
        }
    }

    fn sorted_left_outer_intersect<U: Region + Clone, Other: Iterator<Item = U> + Sorted>(
        self,
        other: Other,
    ) -> outer::LeftOuterJoinIter<Self, Other>
    where
        Self::Item: Region + Clone,
    {
        outer::LeftOuterJoinIter::new(self, other)
    }
}

impl<I: Iterator + Sorted> SortedIntersect for I {}

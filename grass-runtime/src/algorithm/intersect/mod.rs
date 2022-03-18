mod heap;
mod inner;
mod outer;

use crate::property::Region;
use crate::algorithm::markers::Sorted;

use inner::{Context, State};

pub use inner::SortedIntersectIter;

pub trait SortedIntersect: Iterator + Sorted + Sized {
    fn sorted_intersect<
        U: Region + Clone,
        Other: Iterator<Item = U> + Sorted,
    >(
        self,
        other: Other,
    ) -> SortedIntersectIter<Self, Other>
    where
        Self::Item: Region + Clone,
    {
        SortedIntersectIter {
            context_a: Context::from_iter(self),
            context_b: Context::from_iter(other),
            state: State::FrontierA(0, 0, None),
        }
    }

    fn sorted_left_outer_intersect<
        U: Region + Clone,
        Other: Iterator<Item = U> + Sorted,
    >(
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

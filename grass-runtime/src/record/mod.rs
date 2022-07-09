mod bed3;
mod bed4;
mod bed5;
mod bed6;

#[cfg(feature = "htslib")]
mod bam;

use std::marker::PhantomData;

#[cfg(feature = "htslib")]
pub use bam::{BamIter, BamRecord, BamReader};

pub use bed3::Bed3;
pub use bed4::{Bed4, RcStr};
pub use bed5::Bed5;
pub use bed6::Bed6;

use crate::algorithm::Sorted;

pub trait ToSelfContained {
    type SelfContained: 'static;
    fn to_self_contained(&self) -> Self::SelfContained;
}

impl<A: ToSelfContained, B: ToSelfContained> ToSelfContained for (A, B) {
    type SelfContained = (A::SelfContained, B::SelfContained);
    fn to_self_contained(&self) -> Self::SelfContained {
        (self.0.to_self_contained(), self.1.to_self_contained())
    }
}

impl<T: ToSelfContained> ToSelfContained for Option<T> {
    type SelfContained = Option<T::SelfContained>;
    fn to_self_contained(&self) -> Self::SelfContained {
        self.as_ref().map(T::to_self_contained)
    }
}

pub trait CastTo<T>  {
    fn make_record(&self) -> T;
}

pub struct CastIter<I, O> {
    iter: I,
    _phantom : PhantomData<O>,
}

impl <I, O> CastIter<I, O> {
    pub fn cast(iter: I) -> Self {
        CastIter { iter, _phantom: Default::default() }
    }
}

impl <I, O> Iterator for CastIter<I, O> 
where 
    I: Iterator,
    I::Item : CastTo<O>,
{
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().as_ref().map(CastTo::make_record)
    }
}

impl <I: Sorted + Iterator, O> Sorted for CastIter<I, O> 
where I::Item : CastTo<O>
{}
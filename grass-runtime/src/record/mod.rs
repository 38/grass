mod bed3;
mod bed4;
mod bed5;
mod bed6;

#[cfg(feature = "htslib")]
mod bam;

pub use bed3::Bed3;
pub use bed4::{Bed4, RcCowString};
pub use bed5::Bed5;
pub use bed6::Bed6;

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

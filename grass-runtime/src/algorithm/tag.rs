use crate::{
    property::{
        Named, Nuclide, Region, RegionCore, Scored, Serializable, Strand, Stranded, Tagged,
        WithSequence,
    },
    record::ToSelfContained,
};

use super::Sorted;

pub struct TaggedIter<I: Iterator + Sized, T: Clone> {
    iter: I,
    tag: T,
}

impl<I: Iterator, T: Clone + Default> Iterator for TaggedIter<I, T> {
    type Item = TaggedItem<T, I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|value| TaggedItem {
            tag: self.tag.clone(),
            value,
        })
    }
}

impl<I: Iterator + Sorted, T: Clone + Default> Sorted for TaggedIter<I, T> {}

pub trait TaggedIterExt: Iterator + Sorted + Sized {
    fn tagged<T: Clone>(self, tag: T) -> TaggedIter<Self, T> {
        TaggedIter { iter: self, tag }
    }
}

impl<I: Iterator + Sorted + Sized> TaggedIterExt for I {}

#[derive(Clone, Copy, Debug)]
pub struct TaggedItem<T: Clone, V> {
    tag: T,
    value: V,
}

impl<T: Clone, V> Tagged<T> for TaggedItem<T, V> {
    fn tag(&self) -> Option<T> {
        Some(self.tag.clone())
    }
}

impl<T: Clone, V: Region> RegionCore for TaggedItem<T, V> {
    fn start(&self) -> u32 {
        self.value.start()
    }

    fn end(&self) -> u32 {
        self.value.end()
    }

    fn chrom(&self) -> crate::ChrRef<'static> {
        self.value.chrom()
    }
}

impl<'a, T: Clone, V: Named<'a>> Named<'a> for TaggedItem<T, V> {
    fn name(&self) -> &str {
        self.value.name()
    }
}

impl<S, T: Clone, V: Scored<S>> Scored<S> for TaggedItem<T, V> {
    fn score(&self) -> Option<S> {
        self.value.score()
    }
}

impl<T: Clone, V: WithSequence> WithSequence for TaggedItem<T, V> {
    type RangeType = V::RangeType;

    fn at(&self, offset: usize) -> Nuclide {
        self.value.at(offset)
    }

    fn range(&self, from: usize, to: usize) -> Self::RangeType {
        self.value.range(from, to)
    }
}

impl<T: Clone, V: Stranded> Stranded for TaggedItem<T, V> {
    fn strand(&self) -> Strand {
        self.value.strand()
    }
}

impl<T: Clone, V: Serializable> Serializable for TaggedItem<T, V> {
    fn dump<W: std::io::Write>(&self, fp: W) -> std::io::Result<()> {
        self.value.dump(fp)
    }
}

impl<T: Clone + 'static, V: ToSelfContained> ToSelfContained for TaggedItem<T, V> {
    type SelfContained = TaggedItem<T, V::SelfContained>;

    fn to_self_contained(&self) -> Self::SelfContained {
        TaggedItem {
            tag: self.tag.clone(),
            value: self.value.to_self_contained(),
        }
    }
}

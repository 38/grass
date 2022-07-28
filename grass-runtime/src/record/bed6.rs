use std::{
    fmt::Display,
    io::{Result, Write},
    ops::{Deref, DerefMut},
    str::FromStr, rc::Rc,
};

use crate::{
    property::{Named, Parsable, RegionCore, Scored, Serializable, Strand, Stranded, Tagged, Region},
    ChrRef, file::Buffer,
};

use super::{Bed5, RcStr, ToSelfContained, CastTo};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bed6<'a, T = f64> {
    inner: Bed5<'a, T>,
    pub strand: Strand,
}

impl<'a, T> Deref for Bed6<'a, T> {
    type Target = Bed5<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for Bed6<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, T: Display> Serializable for Bed6<'a, T> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        self.inner.dump(&mut fp)?;
        write!(fp, "\t{}", self.strand)?;
        Ok(())
    }
}

impl<'a, T: Display> Serializable for Option<Bed6<'a, T>> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        if let Some(inner) = self {
            inner.dump(fp)
        } else {
            fp.write_all(b".\t.\t.\t.\t.\t.")
        }
    }
}

impl<'a, T: FromStr> Parsable for Bed6<'a, T> {
    fn parse(s: &Rc<Buffer>) -> Option<(Self, usize)> {
        Bed5::parse(s).map(|(inner, mut start)| {
            if s[start..].starts_with('\t') {
                start += 1;
            }
            let s = &s[start..];
            let brk = memchr::memchr(b'\t', s.as_bytes()).unwrap_or(s.trim_end().len());
            let strand = match &s[..brk] {
                "+" => Strand::Positive,
                "-" => Strand::Negative,
                _ => Strand::Unknown,
            };
            (Self { inner, strand }, start + brk)
        })
    }
}

impl<'a, S> Bed6<'a, S> {
    pub fn new<T: RegionCore + Named<'a> + Scored<S> + Stranded>(region: &T) -> Self
    where
        S: Default,
    {
        let inner = Bed5::new(region);
        let strand = region.strand();
        Self { inner, strand }
    }

    #[inline(always)]
    pub fn set_strand(&mut self, strand: &str) {
        match strand {
            "+" => self.strand = Strand::Positive,
            "-" => self.strand = Strand::Negative,
            _ => self.strand = Strand::Unknown,
        }
    }
}

impl<'a, S> RegionCore for Bed6<'a, S> {
    #[inline(always)]
    fn start(&self) -> u32 {
        self.inner.start()
    }
    #[inline(always)]
    fn end(&self) -> u32 {
        self.inner.end()
    }
    #[inline(always)]
    fn chrom(&self) -> ChrRef<'static> {
        self.inner.chrom()
    }
}

impl<'a, T: Clone> Scored<T> for Bed6<'a, T> {
    #[inline(always)]
    fn score(&self) -> Option<T> {
        self.inner.score()
    }
}

impl<'a, T> Stranded for Bed6<'a, T> {
    fn strand(&self) -> Strand {
        self.strand
    }
}

impl<'a, T> Named<'a> for Bed6<'a, T> {
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn rc_name(&self) -> RcStr<'a> {
        self.inner.rc_name()
    }
}

impl<'a> ToSelfContained for Bed6<'a> {
    type SelfContained = Bed6<'static>;
    fn to_self_contained(&self) -> Self::SelfContained {
        Bed6 {
            inner: self.inner.to_self_contained(),
            strand: self.strand,
        }
    }
}

impl<'a, T: Clone> Tagged<T> for Bed6<'a> {}

impl <'a, S: Default, T: Region + Named<'a> + Scored<S> + Stranded> CastTo<Bed6<'a, S>> for T {
    fn make_record(&self) -> Bed6<'a, S> {
        Bed6::new(self)
    }
}
use std::{
    borrow::Cow,
    fmt::Display,
    io::{Result, Write},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{
    property::{Named, Parsable, RegionCore, Scored, Serializable, Stranded},
    ChrRef,
};

use super::Bed4;

#[derive(Clone, PartialEq)]
pub struct Bed5<'a, T = f64> {
    inner: Bed4<'a>,
    pub score: Option<T>,
}

impl<'a, T> Deref for Bed5<'a, T> {
    type Target = Bed4<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for Bed5<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, T: Display> Serializable for Bed5<'a, T> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        self.inner.dump(&mut fp)?;
        if let Some(score) = self.score.as_ref() {
            write!(fp, "\t{}", score)?;
        } else {
            write!(fp, "\t.")?;
        }
        Ok(())
    }
}

impl<'a, T: Display> Serializable for Option<Bed5<'a, T>> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        if let Some(inner) = self {
            inner.dump(fp)
        } else {
            fp.write_all(b".\t.\t.\t.\t.")
        }
    }
}

impl<'a, T: FromStr> Parsable<'a> for Bed5<'a, T> {
    fn parse(s: &'a str) -> Option<(Self, usize)> {
        let (inner, mut start) = Bed4::parse(s)?;
        if s[start..].starts_with('\t') {
            start += 1;
        }
        let s = &s[start..];
        let brk = memchr::memchr(b'\t', s.as_bytes()).unwrap_or(s.len());
        let score = s[..brk].parse().ok();
        Some((Self { inner, score }, start + brk))
    }
}

impl<'a, S> Bed5<'a, S> {
    pub fn new<T: RegionCore + Named<'a> + Scored<S>>(region: &T) -> Self
    where
        S: Default,
    {
        let score = region.score();
        let inner = Bed4::new(region);
        Self { inner, score }
    }

    #[inline(always)]
    pub fn set_score(&mut self, score: S) {
        self.score = Some(score);
    }
}

impl<'a, S> RegionCore for Bed5<'a, S> {
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

impl<'a, T: Clone> Scored<T> for Bed5<'a, T> {
    #[inline(always)]
    fn score(&self) -> Option<T> {
        self.score.clone()
    }
}

impl<'a, T> Stranded for Bed5<'a, T> {}

impl<'a, T> Named<'a> for Bed5<'a, T> {
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn to_cow(&self) -> Cow<'a, str> {
        self.inner.to_cow()
    }
}

use std::{io::{Write, Result}, borrow::Cow};

use crate::{property::{Serializable, Parsable, RegionCore, Scored, Stranded, Named}, ChrRef};

use super::Bed3;

#[derive(Clone, PartialEq)]
pub struct Bed4<'a> {
    inner: Bed3,
    name: Cow<'a, str>,
}

impl <'a> Serializable for Bed4<'a> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()>{
        self.inner.dump(&mut fp)?;
        write!(fp, "\t{}", self.name)?;
        Ok(())
    }
}

impl <'a> Serializable for Option<Bed4<'a>> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()>{
        if let Some(inner) = self {
            inner.dump(fp)
        } else {
            fp.write_all(b".\t.\t.\t.")
        }
    }
}

impl <'a> Parsable<'a> for Bed4<'a> {
    fn parse(s: &'a str) -> Option<(Self, usize)> {
        let (inner, mut start) = Bed3::parse(s)?;
        if s[start..].starts_with('\t') {
            start += 1;
        }
        let s = &s[start..];
        let brk = memchr::memchr(b'\t', s.as_bytes()).unwrap_or(s.len());
        let name = Cow::Borrowed(&s[..brk]);
        Some((Self {inner, name}, start + brk))
    }
}

impl <'a> Bed4<'a> {
    pub fn new<T: RegionCore>(region :T) -> Self {
        Self {
            inner: Bed3 { chrom: region.chrom(), start: region.start(), end: region.end() },
            name: Cow::Borrowed("."),
        }
    } 

    #[inline(always)]
    pub fn set_start(&mut self, start: f64) {
        self.inner.set_start(start);
    }

    #[inline(always)]
    pub fn set_end(&mut self, end: f64) {
        self.inner.set_end(end);
    }

    #[inline(always)]
    pub fn set_name(&mut self, name: &'a str) {
        self.name = Cow::Borrowed(name);
    }
}

impl <'a> RegionCore for Bed4<'a> {
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

impl <'a, T: Default> Scored<T> for Bed4<'a> {
   #[inline(always)]
   fn score(&self) -> T {
       T::default()
   } 
}

impl <'a> Stranded for Bed4<'a> {}

impl <'a> Named for Bed4<'a> {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl <'a> AsRef<Bed3> for Bed4<'a> {
    fn as_ref(&self) -> &Bed3 {
        &self.inner
    }
}
use std::{io::{Write, Result}, fmt::Display, str::FromStr};

use crate::{property::{Serializable, Parsable, RegionCore, Scored, Stranded, Named, Strand}, ChrRef};

use super::Bed5;

#[derive(Clone, PartialEq)]
pub struct Bed6<'a, T = f64> {
    inner: Bed5<'a, T>,
    strand: Strand,
}

impl <'a, T : Display> Serializable for Bed6<'a, T> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()>{
        self.inner.dump(&mut fp)?;
        write!(fp, "\t{}", self.strand)?;
        Ok(())
    }
}

impl <'a, T : Display> Serializable for Option<Bed6<'a, T>> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()>{
        if let Some(inner) = self {
            inner.dump(fp)
        } else {
            fp.write_all(b".\t.\t.\t.\t.\t.")
        }
    }
}

impl <'a, T: FromStr> Parsable<'a> for Bed6<'a, T> {
    fn parse(s: &'a str) -> Option<(Self, usize)> {
        let (inner, mut start) = Bed5::parse(s)?;
        if s[start..].starts_with('\t') {
            start += 1;
        }
        let s = &s[start..];
        let brk = memchr::memchr(b'\t', s.as_bytes()).unwrap_or(s.len());
        let strand = match &s[..brk] {
            "+" => Strand::Positive,
            "-" => Strand::Negative,
            _ => Strand::Unknown,
        };

        Some((Self {inner, strand}, start + brk))
    }
}

impl <'a, S> Bed6<'a, S> {
    pub fn new<T: RegionCore>(region :T) -> Self where S: Default {
        let inner = Bed5::new(region);
        Self { inner , strand: Strand::Unknown }
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
        self.inner.set_name(name);
    }

    #[inline(always)]
    pub fn set_score(&mut self, score: S) {
        self.inner.set_score(score);
    }

    #[inline(always)]
    pub fn set_strand(&mut self, strand: &str) {
        match strand {
            "+" => self.strand = Strand::Positive,
            "-" => self.strand = Strand::Positive,
            _ => self.strand = Strand::Unknown,
        }
    }
}

impl <'a, S> RegionCore for Bed6<'a, S> {
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

impl <'a, T: Clone> Scored<T> for Bed6<'a, T> {
   #[inline(always)]
   fn score(&self) -> T {
       self.inner.score()
   } 
}

impl <'a, T> Stranded for Bed6<'a, T> {
    fn strand(&self) -> Strand {
        self.strand
    }
}

impl <'a, T> Named for Bed6<'a, T> {
    fn name(&self) -> &str {
        self.inner.name()
    }
}
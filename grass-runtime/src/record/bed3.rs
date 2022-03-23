use std::{io::{Write, Result}};

use crate::{ChrRef, property::{RegionCore, Scored, Stranded, Serializable, Parsable, Named}};

#[derive(Clone, Copy, PartialEq)]
pub struct Bed3 {
    pub chrom: ChrRef<'static>,
    pub start: u32,
    pub end: u32,
}

impl Serializable for Bed3 {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()>{
        fp.write_all(self.chrom().get_chr_name().as_bytes())?;
        fp.write(b"\t")?;
        crate::ioutils::write_number(&mut fp, self.start() as i32)?;
        fp.write(b"\t")?;
        crate::ioutils::write_number(&mut fp, self.end() as i32).map(|_| ())
    }
}

impl Serializable for Option<Bed3> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()>{
        if let Some(inner) = self {
            inner.dump(fp)
        } else {
            fp.write_all(b".\t.\t.\t")
        }
    }
}

impl <'a> Parsable<'a> for Bed3 {
    fn parse(s: &'a str) -> Option<(Self, usize)> {
        let mut bytes = s.as_bytes();

        if bytes.last() == Some(&b'\n') {
            bytes = &bytes[..bytes.len() - 1];
        }

        let mut token_pos_iter = memchr::Memchr::new(b'\t', bytes);
        let end_1 = token_pos_iter.next()?;
        let end_2 = token_pos_iter.next()?;
        let end_3 = token_pos_iter.next().unwrap_or(bytes.len());
        let chrom = &s[..end_1];

        Some((
            Self {
                chrom: crate::Genome::query_chr(chrom).to_static(),
                start: s[end_1 + 1..end_2].parse().ok()?,
                end: s[end_2 + 1..end_3].parse().ok()?,
            },
            end_3,
        ))
    }
}

impl Bed3 {
    pub fn new<T: RegionCore>(region :T) -> Self {
        Self {
            chrom: region.chrom(),
            start: region.start(),
            end: region.end(),
        }
    } 
    #[inline(always)]
    pub fn set_start(&mut self, start: f64) {
        self.start = start as u32;
    }
    #[inline(always)]
    pub fn set_end(&mut self, end: f64) {
        self.end = end as u32;
    }
}

impl RegionCore for Bed3 {
    #[inline(always)]
    fn start(&self) -> u32 {
        self.start
    }
    #[inline(always)]
    fn end(&self) -> u32 {
        self.end
    }
    #[inline(always)]
    fn chrom(&self) -> ChrRef<'static> {
        self.chrom
    }
}

impl <T: Default> Scored<T> for Bed3 {
   #[inline(always)]
   fn score(&self) -> T {
       T::default()
   } 
}

impl Stranded for Bed3 {}

impl Named for Bed3 {}
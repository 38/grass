use std::{io::{Result, Write}, rc::Rc};

use crate::{
    property::{Named, Parsable, RegionCore, Scored, Serializable, Stranded, Tagged, Region},
    ChrRef, file::Buffer,
};

use super::{ToSelfContained, RcStr, CastTo};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bed3 {
    pub chrom: ChrRef<'static>,
    pub start: u32,
    pub end: u32,
}

impl Serializable for Bed3 {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        fp.write_all(self.chrom().get_chr_name().as_bytes())?;
        fp.write(b"\t")?;
        crate::ioutils::write_number(&mut fp, self.start() as i32)?;
        fp.write(b"\t")?;
        crate::ioutils::write_number(&mut fp, self.end() as i32).map(|_| ())
    }
}

impl Serializable for Option<Bed3> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        if let Some(inner) = self {
            inner.dump(fp)
        } else {
            fp.write_all(b".\t.\t.\t")
        }
    }
}

#[inline(always)]
fn parse_u32_fast(s: &str) -> u32 {
    s.bytes().fold(0, |r, b| r * 10 + (b - b'0') as u32)
}

impl Parsable for Bed3 {
    fn parse(s: &Rc<Buffer>) -> Option<(Self, usize)> {
        let mut bytes = s.as_bytes();

        if bytes.last() == Some(&b'\n') {
            bytes = &bytes[..bytes.len() - 1];
        }

        let mut token_pos_iter = memchr::Memchr::new(b'\t', bytes);
        if let Some(end_1) = token_pos_iter.next() {
            if let Some(end_2) = token_pos_iter.next() {
                let end_3 = token_pos_iter.next().unwrap_or(bytes.len());
                let chrom = &s[..end_1];
                return Some((
                    Self {
                        chrom: crate::Genome::query_chr(chrom).to_static(),
                        start: parse_u32_fast(&s[end_1 + 1..end_2]),
                        end: parse_u32_fast(&s[end_2 + 1..end_3]),
                    },
                    end_3,
                ));
            }
        }
        None
    }
}

impl Bed3 {
    pub fn new<T: RegionCore>(region: &T) -> Self {
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

impl Scored<f64> for Bed3 {
    #[inline(always)]
    fn score(&self) -> Option<f64> {
        Default::default()
    }
}

impl Stranded for Bed3 {}

impl Named<'static> for Bed3 {
    fn rc_name(&self) -> RcStr<'static> {
        RcStr::from_str(".")
    }
}

impl ToSelfContained for Bed3 {
    type SelfContained = Bed3;
    fn to_self_contained(&self) -> Self::SelfContained {
        *self
    }
}

impl<T: Clone> Tagged<T> for Bed3 {}

impl <T: Region> CastTo<Bed3> for T {
    fn make_record(&self) -> Bed3 {
        Bed3::new(self)
    }
}
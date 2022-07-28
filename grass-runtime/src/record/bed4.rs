use std::io::{Result, Write};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::file::Buffer;
use crate::property::{Tagged, Region};
use crate::{
    property::{Named, Parsable, RegionCore, Scored, Serializable, Stranded},
    ChrRef,
};

use super::{Bed3, ToSelfContained, CastTo};

#[derive(Clone)]
pub enum RcStr<'a> {
    BufRef {
        data: Rc<Buffer>,
        view_start: usize,
        view_end: usize,
    },
    Shared (&'a str),
}

impl <'a> RcStr<'a> {
    pub fn from_buffer(data: &Rc<Buffer>, start: usize, end: usize) -> RcStr<'static> {
        RcStr::BufRef { data: data.clone(), view_start: start, view_end: end }
    }
    pub fn from_str(shared: &'a str) -> RcStr<'a> {
        Self::Shared(shared)
    }
    pub fn to_static(&self) -> RcStr<'static> {
        match self { 
            Self::BufRef { data, view_start, view_end } => 
                RcStr::BufRef { data: data.clone(), view_start: *view_start, view_end: *view_end },
            Self::Shared(s) => 
                RcStr::BufRef { data: Rc::new(Buffer::from_str(s)), view_start: 0, view_end: s.len() }
        }
    }
}

impl <'a> Deref for RcStr<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::BufRef{data, view_start, view_end } => {
                &data[*view_start..*view_end]
            },
            Self::Shared(data) => data
        }
    }
}

impl <'a> PartialEq for RcStr<'a> {
    fn eq(&self, other: &Self) -> bool {
        str::eq(self.deref(), other.deref())
    }
}

impl <'a> PartialOrd for RcStr<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        str::partial_cmp(self.as_ref(), other.as_ref())
    }
}

impl <'a> Eq for RcStr<'a> {}

impl <'a> Ord for RcStr<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.deref().cmp(other)
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bed4<'a> {
    inner: Bed3,
    pub name: RcStr<'a>,
}

impl <'a> Deref for Bed4<'a> {
    type Target = Bed3;
    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl <'a> DerefMut for Bed4<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl <'a> Serializable for Bed4<'a> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        self.inner.dump(&mut fp)?;
        write!(fp, "\t{}", self.name.deref())?;
        Ok(())
    }
}

impl <'a> Serializable for Option<Bed4<'a>> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        if let Some(inner) = self {
            inner.dump(fp)
        } else {
            fp.write_all(b".\t.\t.\t.")
        }
    }
}

impl <'a> Parsable for Bed4<'a> {
    fn parse(s: &Rc<Buffer>) -> Option<(Self, usize)> {
        Bed3::parse(s).map(|(inner, mut start)| {
            if s[start..].starts_with('\t') {
                start += 1;
            }
            let (name, pos) = if let Some(brk) = memchr::memchr(b'\t', s[start..].as_bytes()) {
                (RcStr::from_buffer(s, start, start + brk), start + brk)
            }  else {
                let end = s.trim_end().len();
                (RcStr::from_buffer(s, start, end), end)
            };
            (Self { inner, name }, pos)
        })
    }
}

impl <'a> Bed4<'a> {
    pub fn new<T: RegionCore + Named<'a>>(region: &T) -> Self {
        let inner = Bed3::new(&region);
        let name = region.rc_name();
        Self { inner, name }
    }

    #[inline(always)]
    pub fn set_name(&mut self, name: &'a str) {
        self.name = RcStr::from_str(name);
    }

    pub fn get_self_contained_name(&self) -> RcStr<'static> {
        self.name.to_static()
    }
}

impl<'a> RegionCore for Bed4<'a> {
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

impl<'a> Scored<f64> for Bed4<'a> {
    #[inline(always)]
    fn score(&self) -> Option<f64> {
        Default::default()
    }
}

impl<'a> Stranded for Bed4<'a> {}

impl<'a> Named<'a> for Bed4<'a> {
    fn name(&self) -> &str {
        self.name.as_ref()
    }
    fn rc_name(&self) -> RcStr<'a> {
        self.name.clone()
    }
}

impl<'a> AsRef<Bed3> for Bed4<'a> {
    fn as_ref(&self) -> &Bed3 {
        &self.inner
    }
}

impl<'a> ToSelfContained for Bed4<'a> {
    type SelfContained = Bed4<'static>;
    fn to_self_contained(&self) -> Self::SelfContained {
        Bed4 {
            inner: self.inner,
            name: self.get_self_contained_name(),
        }
    }
}

impl<'a, T: Clone> Tagged<T> for Bed4<'a> {}

impl <'a, T: Region + Named<'a>> CastTo<Bed4<'a>> for T {
    fn make_record(&self) -> Bed4<'a> {
        Bed4::new(self)
    }
}
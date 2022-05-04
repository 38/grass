use std::io::{Result, Write};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::{
    property::{Named, Parsable, RegionCore, Scored, Serializable, Stranded},
    ChrRef,
};

use super::{Bed3, ToSelfContained};

#[derive(Clone)]
pub enum RcCowString<'a> {
    Borrowed(&'a str),
    RcOwned(Rc<String>),
}

impl<'a> Deref for RcCowString<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            RcCowString::Borrowed(val) => val,
            RcCowString::RcOwned(rc_val) => rc_val.as_str(),
        }
    }
}

impl<'a> PartialEq for RcCowString<'a> {
    fn eq(&self, other: &Self) -> bool {
        str::eq(self.deref(), other.deref())
    }
}

impl<'a> PartialOrd for RcCowString<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        str::partial_cmp(self.as_ref(), other.as_ref())
    }
}

impl<'a> Eq for RcCowString<'a> {}

impl<'a> Ord for RcCowString<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.deref().cmp(other)
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Bed4<'a> {
    inner: Bed3,
    pub name: RcCowString<'a>,
}

impl<'a> Deref for Bed4<'a> {
    type Target = Bed3;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for Bed4<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> Serializable for Bed4<'a> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        self.inner.dump(&mut fp)?;
        write!(fp, "\t{}", self.name.deref())?;
        Ok(())
    }
}

impl<'a> Serializable for Option<Bed4<'a>> {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        if let Some(inner) = self {
            inner.dump(fp)
        } else {
            fp.write_all(b".\t.\t.\t.")
        }
    }
}

impl<'a> Parsable<'a> for Bed4<'a> {
    fn parse(s: &'a str) -> Option<(Self, usize)> {
        let (inner, mut start) = Bed3::parse(s)?;
        if s[start..].starts_with('\t') {
            start += 1;
        }
        let s = &s[start..];
        let brk = memchr::memchr(b'\t', s.as_bytes()).unwrap_or(s.len());
        let name = RcCowString::Borrowed(&s[..brk]);
        Some((Self { inner, name }, start + brk))
    }
}

impl<'a> Bed4<'a> {
    pub fn new<T: RegionCore + Named<'a>>(region: &T) -> Self {
        let name = region.to_cow();
        let inner = Bed3::new(&region);
        Self { inner, name }
    }

    #[inline(always)]
    pub fn set_name(&mut self, name: &'a str) {
        self.name = RcCowString::Borrowed(name);
    }

    pub fn get_self_contained_name(&self) -> RcCowString<'static> {
        match &self.name {
            RcCowString::Borrowed(name) => RcCowString::RcOwned(Rc::new(name.to_string())),
            RcCowString::RcOwned(rc_name) => RcCowString::RcOwned(rc_name.clone()),
        }
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
    fn to_cow(&self) -> RcCowString<'a> {
        match &self.name {
            RcCowString::Borrowed(name) => RcCowString::Borrowed(name),
            RcCowString::RcOwned(name) => RcCowString::RcOwned(name.clone()),
        }
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

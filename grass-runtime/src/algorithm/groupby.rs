#![allow(unused)]

use std::{rc::Rc, cell::RefCell};

use crate::{record::ToSelfContained, property::{RegionCore, Region}, ChrRef};

pub struct GroupBuffer<K:ToOwned, T : 'static> {
    key: <K as ToOwned>::Owned,
    buffer: Vec<T>,
    overlap: Option<(ChrRef<'static>, u32, u32)>,
    outline: Option<(ChrRef<'static>, u32, u32)>,
}

impl <K: ToOwned, T: 'static + Region> GroupBuffer<K, T> {
    pub fn overlap(self) -> GroupOverlap<K, T> 
    where T: Region
    {
        GroupOverlap(self)   
    }
    fn compute_overlap(&mut self) {
        let mut ret:Option<(_, u32, u32)> = None;
        for region in self.buffer.iter() {
            if let Some(cur) = ret {
                if region.overlaps(&cur) {
                    ret = None;
                    break;
                } else {
                    ret = Some((
                        region.chrom(), 
                        region.start().max(cur.1), 
                        region.end().min(cur.2),
                    ));
                }
            } else {
                ret = Some((region.chrom(), region.start(), region.end()))
            }
        }
        self.overlap = ret;
    }
    fn compute_outline(&mut self) {
        let mut ret:Option<(_, u32, u32)> = None;
        for region in self.buffer.iter() {
            if let Some(cur) = ret {
                if region.chrom() != cur.0 {
                    ret = None;
                    break;
                } else {
                    ret = Some((
                        region.chrom(), 
                        region.start().min(cur.1), 
                        region.end().max(cur.2),
                    ));
                }
            } else {
                ret = Some((region.chrom(), region.start(), region.end()))
            }
        }
        self.outline = ret;
    }
    fn get_outline(&self) -> Option<(ChrRef<'static>, u32, u32)> {
        self.outline
    }
    fn get_overlap(&self) -> Option<(ChrRef<'static>, u32, u32)> {
        self.overlap
    }
}

impl <K: ToOwned, T: 'static + Region> RegionCore for GroupBuffer<K, T> {
    fn start(&self) -> u32 {
        self.get_outline().map_or(0, |(_, s, _)| s)
    }

    fn end(&self) -> u32 {
        self.get_outline().map_or(0, |(_, _, e)| e)
    }

    fn chrom(&self) -> crate::ChrRef<'static> {
        self.get_outline().map_or(ChrRef::Dummy, |(c, _, _)| c)
    }
}
pub struct GroupOverlap<K: ToOwned, T: 'static + Region>(GroupBuffer<K, T>);

impl <K: ToOwned, T: 'static + Region> RegionCore for GroupOverlap<K, T> {
    fn start(&self) -> u32 {
        self.0.get_overlap().map_or(0, |(_, s, _)| s)
    }

    fn end(&self) -> u32 {
        self.0.get_overlap().map_or(0, |(_, _, e)| e)
    }

    fn chrom(&self) -> crate::ChrRef<'static> {
        self.0.get_overlap().map_or(ChrRef::Dummy, |(c, _, _)| c)
    }
}

pub struct Groups<'a, K, I, F> 
where
    K: ToOwned + PartialEq,
    I: Iterator,
    I::Item : ToSelfContained,
{
    inner: itertools::Groups<'a, K, I, F>,
}

impl <'a, K, I, F> Iterator for Groups<'a, K, I, F>
where
    K: ToOwned + PartialEq,
    I: Iterator,
    I::Item : ToSelfContained,
    F: FnMut(&I::Item) -> K,
{
    type Item = GroupBuffer<K, <I::Item as ToSelfContained>::SelfContained>;
    fn next(&mut self) -> Option<Self::Item> {
        let (key, inner_group) = self.inner.next()?;
        Some(GroupBuffer {
            key: key.to_owned(),
            buffer: inner_group.map(|item| item.to_self_contained()).collect(),
            overlap: None,
            outline: None,
        })
    }
}
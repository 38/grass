#![allow(unused)]

use std::{rc::Rc, cell::{RefCell, Cell}, borrow::Borrow};

use crate::{record::ToSelfContained, property::{RegionCore, Region}, ChrRef};


pub struct GroupBuffer<K:ToOwned, T : 'static> {
    key: <K as ToOwned>::Owned,
    buffer: Vec<T>,
    overlap: Cell<Option<Option<(ChrRef<'static>, u32, u32)>>>,
    outline: Cell<Option<Option<(ChrRef<'static>, u32, u32)>>>,
}

impl <K: ToOwned, T: 'static + Region> GroupBuffer<K, T> {
    pub fn overlap(self) -> GroupOverlap<K, T> 
    where T: Region
    {
        GroupOverlap(self)   
    }
    fn compute_overlap(&self) {
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
        self.overlap.set(Some(ret));
    }
    fn compute_outline(&self) {
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
        self.outline.set(Some(ret));
    }
    fn get_outline(&self) -> Option<(ChrRef<'static>, u32, u32)> {
        if let Some(ret) = self.outline.clone().take() {
            ret
        } else {
            self.compute_outline();
            self.outline.clone().take().unwrap()
        }
    }
    fn get_overlap(&self) -> Option<(ChrRef<'static>, u32, u32)> {
        if let Some(ret) = self.overlap.clone().take() {
            ret
        } else {
            self.compute_overlap();
            self.overlap.clone().take().unwrap()
        }
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
            overlap: Cell::new(None),
            outline: Cell::new(None),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{LineRecordStreamExt, record::Bed3, algorithm::{AssumeSorted, Components}};

    #[test]
    fn test_group_by() -> Result<(), Box<dyn std::error::Error>> {
        let input = include_bytes!("../../../data/a.bed");
        let bed3 = input.into_record_iter::<Bed3>().assume_sorted();
        let comp_iter = bed3.components();
        /*let mut idx = 0;
        comp_iter.grou(move |comp| {
            if comp.depth() == 0 {
            idx += 1;
            }
            idx
        )*/
        Ok(())
    }
}
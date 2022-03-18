use super::heap::RegionHeap;
use super::Sorted;
use crate::{
    property::{Region, RegionCore}, ChrRef,
};

pub struct LeftOuterJoinIter<IA, IB>
where
    IA: Iterator + Sorted,
    IB: Iterator + Sorted,
    IA::Item: Region + Clone,
    IB::Item: Region + Clone,
{
    iter_a: IA,
    iter_b: IB,
    active_regions: RegionHeap<IB::Item>,
    current_chrom: Option<ChrRef<'static>>,
    limit: u32,
    current_a: Option<IA::Item>,
    current_b: Option<IB::Item>,
    current_b_idx: usize,
}

impl<IA, IB> LeftOuterJoinIter<IA, IB>
where
    IA: Iterator + Sorted,
    IB: Iterator + Sorted,
    IA::Item: Region + Clone,
    IB::Item: Region + Clone,
{
    pub(super) fn new(iter_a: IA, mut iter_b: IB) -> Self {
        let current_b = iter_b.next();
        let mut ret = Self {
            iter_a,
            iter_b,
            active_regions: Default::default(),
            current_chrom: None,
            limit: 0,
            current_a: None,
            current_b,
            current_b_idx: 0,
        };
        ret.read_next_a();
        ret
    }

    fn read_next_a(&mut self) -> Option<()> {
        self.current_a = self.iter_a.next();
        let cur_a = self.current_a.as_ref()?;

        if Some(&cur_a.chrom()) != self.current_chrom.as_ref() {
            self.current_chrom = Some(cur_a.chrom().clone());
            self.limit = 0;
            self.active_regions.data.clear();
        }

        self.limit = self.limit.max(cur_a.end());

        while let Some(ref b) = self.current_b {
            if Some(&b.chrom()) > self.current_chrom.as_ref() || self.limit <= b.end() {
                break;
            }
            self.active_regions.push(self.current_b.take().unwrap());
            self.current_b = self.iter_b.next();
        }

        while let Some(top) = self.active_regions.peek() {
            if top.end() < cur_a.start() {
                self.active_regions.pop();
            } else {
                break;
            }
        }
        self.current_b_idx = 0;
        Some(())
    }
}

impl<IA, IB> Iterator for LeftOuterJoinIter<IA, IB>
where
    IA: Iterator + Sorted,
    IB: Iterator + Sorted,
    IA::Item: Region + Clone,
    IB::Item: Region + Clone,
{
    type Item = (IA::Item, Option<IB::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let cur_a = self.current_a.as_ref()?;
            if self.current_b_idx < self.active_regions.data.len() {
                self.current_b_idx += 1;
                return Some((
                    cur_a.clone(),
                    Some(self.active_regions.data[self.current_b_idx - 1].clone()),
                ));
            } else if self.current_b_idx == 0 && self.active_regions.data.len() == 0 {
                self.current_b_idx += 1;
                return Some((cur_a.clone(), None));
            }
            self.read_next_a()?;
        }
    }
}

use std::iter::Peekable;

use crate::{property::Region, record::Bed3, Genome};

use super::{Components, ComponentsIter, Sorted};

pub struct SortedInversion<I>
where
    I: Iterator + Sorted,
    I::Item: Region + Clone,
{
    iter: Peekable<ComponentsIter<I>>,
    chrom_id: usize,
    last_end_pos: usize,
}

impl<I> Sorted for SortedInversion<I>
where
    I: Iterator + Sorted,
    I::Item: Region + Clone,
{
}

impl<I> Iterator for SortedInversion<I>
where
    I: Iterator + Sorted,
    I::Item: Region + Clone,
{
    type Item = Bed3;

    fn next(&mut self) -> Option<Self::Item> {
        // First, find the next covered region
        while let Some(next_comp) = self.iter.peek() {
            if next_comp.depth != 0 {
                break;
            }
            self.iter.next();
        }

        if let Some((end_chr, end)) = self.iter.peek().map(|c| c.position()) {
            // Then we have a good end point for the inverted interval
            if let Some(current_chr) = Genome::get_chr_by_id(self.chrom_id) {
                if current_chr == end_chr {
                    // This means the start point and the end point are on the same genome
                    let region = Bed3 {
                        chrom: current_chr,
                        start: self.last_end_pos as u32,
                        end: end,
                    };

                    // Then we need to fastward the frontier to the end of this covered region
                    while self.iter.peek().map_or(false, |c| c.depth != 0) {
                        self.iter.next();
                    }
                    self.last_end_pos = self.iter.peek().unwrap().position().1 as usize;

                    return Some(region);
                } else {
                    // Otherwise, it means the suggested end point is in a different chromosome
                    // So we need to report the rest of current chromosome
                    let region = Bed3 {
                        chrom: current_chr,
                        start: self.last_end_pos as u32,
                        end: current_chr.get_chr_size().map_or(u32::MAX, |x| x as u32),
                    };

                    self.chrom_id += 1;
                    self.last_end_pos = 0;

                    return Some(region);
                }
            }
        }

        // Otherwise, means we can't find any end point, then we just report regions that
        // covers reset of the genome
        if let Some(chr) = Genome::get_chr_by_id(self.chrom_id) {
            let start = self.last_end_pos as u32;
            let end = chr.get_chr_size().unwrap_or(usize::MAX) as u32;
            self.chrom_id += 1;
            self.last_end_pos = 0;
            return Some(Bed3 {
                chrom: chr,
                start,
                end,
            });
        } else {
            return None;
        }
    }
}

pub trait SortedInversionExt
where
    Self: Iterator + Sorted + Sized,
    Self::Item: Region + Clone,
{
    fn invert(self) -> SortedInversion<Self> {
        SortedInversion {
            iter: self.components().peekable(),
            chrom_id: 0,
            last_end_pos: 0,
        }
    }
}

impl<I> SortedInversionExt for I
where
    I: Iterator + Sorted + Sized,
    I::Item: Region + Clone,
{
}

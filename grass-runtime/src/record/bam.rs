use std::{rc::Rc, path::Path, error::Error};

use d4_hts::{Alignment, BamFile, AlignmentReader, AlignmentIter};

use crate::{ChrRef, Genome, property::{RegionCore, Scored, Stranded, Strand}};

#[derive(Clone)]
pub struct BamRecord<'a> {
    chrom_name: ChrRef<'a>,
    record: Rc<Alignment<'a>>,
}

pub struct BamReader {
    file:BamFile,
    chroms: Vec<ChrRef<'static>>,
}

pub struct BamIter<'a>(AlignmentIter<'a, &'a BamFile>, &'a [ChrRef<'static>]);

impl <'a> Iterator for BamIter<'a> {
    type Item = BamRecord<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let BamIter(ref mut iter, ref chroms) = self;
        loop {
            if let Ok(alignment) = iter.next()? {
                let chrom_name = chroms[alignment.ref_id() as usize].clone();
                let record = Rc::new(alignment);
                return Some(BamRecord{
                    chrom_name,
                    record,
                })
            }
        }
    }
}

impl BamReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file = BamFile::open(path.as_ref())?;
        let chroms = file.chroms().iter().map(|(name, size)|{
            let chr = Genome::query_chr(name.as_str()).to_static();
            chr.verify_size_or_update(*size);
            chr
        }).collect();
        Ok(Self{
            file,
            chroms,
        })
    }
    pub fn iter(&self) -> BamIter {
        BamIter(self.file.into_alignment_iter(), self.chroms.as_slice())
    }
}

impl<'a> RegionCore for BamRecord<'a> {
    fn end(&self) -> u32 {
        self.record.ref_end() as u32
    }

    fn chrom(&self) -> ChrRef<'static> {
        self.chrom_name.to_static()
    }

    fn start(&self) -> u32 {
        self.record.ref_begin() as u32
    }
}

impl <'a> Scored<f64> for BamRecord<'a> {
    fn score(&self) -> Option<f64> {
        Some(self.record.map_qual() as f64)
    }
}

impl <'a> Stranded for BamRecord<'a> {
    fn strand(&self) -> Strand {
        if self.record.flag() & 16 > 0 {
            Strand::Negative
        } else {
            Strand::Positive
        }
    }
}
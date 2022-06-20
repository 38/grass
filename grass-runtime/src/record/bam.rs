use crate::{property::RegionCore, Genome, ChrRef};

use d4_hts::{Alignment, AlignmentReader};
use std::rc::Rc;

use d4_hts::BamFile;

#[derive(Clone)]
pub struct BamRecord<'a> {
    chrom_name: ChrRef<'a>,
    record: Rc<Alignment<'a>>,
}

pub struct BamReader {

}

// TODO
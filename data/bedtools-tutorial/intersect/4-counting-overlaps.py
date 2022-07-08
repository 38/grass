#!/usr/bin/python3

"""
This is equivalent to bedtools command:

    bedtools intersect -a cpg.bed -b exons.bed -c \
    | head
    chr1    28735   29810   CpG:_116    1
    chr1    135124  135563  CpG:_30 1
    chr1    327790  328229  CpG:_29 3
    chr1    437151  438164  CpG:_84 0
    chr1    449273  450544  CpG:_99 0
    chr1    533219  534114  CpG:_94 0
    chr1    544738  546649  CpG:_171    0
    chr1    713984  714547  CpG:_60 1
    chr1    762416  763445  CpG:_115    10
    chr1    788863  789211  CpG:_28 9

"""

from pygrass import IntervalFile, item, parse_args, RustEnv, Bed4

parse_args()

afile = IntervalFile("../data/cpg.bed").tagged(0)
bfile = Bed4(IntervalFile("../data/exons.bed")).tagged(1)

RustEnv(input = afile.merge_with(bfile)).inline_rust("""
    use grass_runtime::algorithm::Components;
    use grass_runtime::property::{Named, RegionCore, Tagged};
    let mut cnt = (0, 0);
    let mut active_a = std::collections::HashMap::<usize, usize>::new();
    for comp in input.components() {
        if comp.tag() == Some(1) {
            if comp.is_open {
                cnt.0 += 1;
            } else {
                cnt.1 += 1;
            }
        } else {
            if comp.is_open {
                active_a.insert(comp.index, cnt.1);
            } else {
                let count = cnt.0 - active_a.remove(&comp.index).unwrap();
                println!("{}\t{}\t{}\t{}", comp.chrom(), comp.start(), comp.end(), count);
            }
        }
    }
""")

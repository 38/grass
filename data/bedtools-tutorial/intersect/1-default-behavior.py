#!/usr/bin/python3

"""
This is equivalent to bedtools command:

    bedtools intersect -a cpg.bed -b exons.bed | head -5
    chr1    29320   29370   CpG:_116
    chr1    135124  135563  CpG:_30
    chr1    327790  328229  CpG:_29
    chr1    327790  328229  CpG:_29
    chr1    327790  328229  CpG:_29
"""

from pygrass import IntervalFile, chr as chrom, start, end, name, parse_args

parse_args()

afile = IntervalFile("../data/cpg.bed")
bfile = IntervalFile("../data/exons.bed")

afile.intersect(bfile).format(
    "{chrom}\t{start}\t{end}\t{name}", 
    # chrom refers to the chromosome name of the overlap
    chrom = chrom,   
    start = start,   
    end = end,
    # name[0] refers the name of the overlapping interval from file A
    name = name[0],  
).print_to_stdout()

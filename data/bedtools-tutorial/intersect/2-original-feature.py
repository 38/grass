#!/usr/bin/python3

"""
This is equivalent to bedtools command:

bedtools intersect -a cpg.bed -b exons.bed -wa -wb \
| head -5
chr1    28735   29810   CpG:_116    chr1    29320   29370   NR_024540_exon_10_0_chr1_29321_r        -
chr1    135124  135563  CpG:_30 chr1    134772  139696  NR_039983_exon_0_0_chr1_134773_r    0   -
chr1    327790  328229  CpG:_29 chr1    324438  328581  NR_028322_exon_2_0_chr1_324439_f    0   +
chr1    327790  328229  CpG:_29 chr1    324438  328581  NR_028325_exon_2_0_chr1_324439_f    0   +
chr1    327790  328229  CpG:_29 chr1    327035  328581  NR_028327_exon_3_0_chr1_327036_f    0   +

"""

from pygrass import IntervalFile, parse_args

parse_args()

afile = IntervalFile("../data/cpg.bed")
bfile = IntervalFile("../data/exons.bed")

# Actually, this is the default behavior of the intersect function in pygrass
afile.intersect(bfile).print_to_stdout()

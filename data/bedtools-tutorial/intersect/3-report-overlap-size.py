#!/usr/bin/python3

"""
This is equivalent to bedtools command:

bedtools intersect -a cpg.bed -b exons.bed -wo \
| head -10
chr1    28735   29810   CpG:_116    chr1    29320   29370   NR_024540_exon_10_0_chr1_29321_r        -   50
chr1    135124  135563  CpG:_30 chr1    134772  139696  NR_039983_exon_0_0_chr1_134773_r    0       439
chr1    327790  328229  CpG:_29 chr1    324438  328581  NR_028322_exon_2_0_chr1_324439_f    0       439
chr1    327790  328229  CpG:_29 chr1    324438  328581  NR_028325_exon_2_0_chr1_324439_f    0       439
chr1    327790  328229  CpG:_29 chr1    327035  328581  NR_028327_exon_3_0_chr1_327036_f    0       439
chr1    713984  714547  CpG:_60 chr1    713663  714068  NR_033908_exon_6_0_chr1_713664_r    0       84
chr1    762416  763445  CpG:_115    chr1    761585  762902  NR_024321_exon_0_0_chr1_761586_r        -   486
chr1    762416  763445  CpG:_115    chr1    762970  763155  NR_015368_exon_0_0_chr1_762971_f        +   185
chr1    762416  763445  CpG:_115    chr1    762970  763155  NR_047519_exon_0_0_chr1_762971_f        +   185
chr1    762416  763445  CpG:_115    chr1    762970  763155  NR_047520_exon_0_0_chr1_762971_f        +   185

"""

from pygrass import IntervalFile, item, length, parse_args

parse_args()

afile = IntervalFile("../data/cpg.bed")
bfile = IntervalFile("../data/exons.bed")

afile.intersect(bfile).format(
    "{a}\t{b}\t{overlap_size}", 
    a = item[0].str_repr,
    b = item[1].str_repr,
    overlap_size = length,
).print_to_stdout()

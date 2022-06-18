#!/usr/bin/python3

"""
This is equivalent to bedtools command:

    bedtools intersect -a cpg.bed -b exons.bed -v \
    | head
    chr1    437151  438164  CpG:_84
    chr1    449273  450544  CpG:_99
    chr1    533219  534114  CpG:_94
    chr1    544738  546649  CpG:_171
    chr1    801975  802338  CpG:_24
    chr1    805198  805628  CpG:_50
    chr1    839694  840619  CpG:_83
    chr1    844299  845883  CpG:_153
    chr1    912869  913153  CpG:_28
    chr1    919726  919927  CpG:_15

"""

from pygrass import IntervalFile, length, item

afile = IntervalFile("../data/cpg.bed")
bfile = IntervalFile("../data/exons.bed")

afile.intersect(bfile.invert()) \
    .filter(length[0] == length) \
    .format("{a}", a = item[0].str_repr) \
    .print_to_stdout()

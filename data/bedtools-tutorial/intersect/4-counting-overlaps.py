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

from pygrass import IntervalFile, item, parse_args, tag
from grass_ext import count_overlaps

parse_args()

afile = IntervalFile("../data/cpg.bed")
bfile = IntervalFile("../data/exons.bed")

# In this example, we demonstrate how to make your own library on top of pygrass.
# Originally, pygrass doesn't directly support counting overlaps, but this example 
# shows how to make your own library on top of it.
# See lib/grass_ext/__init__.py for more details.
count_overlaps(afile, bfile)\
        .format("{bed}\t{count}", bed = item.str_repr, count = tag)\
        .print_to_stdout()

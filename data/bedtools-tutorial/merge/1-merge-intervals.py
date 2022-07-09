#!/usr/bin/python3

"""
This is equivalent to bedtools command:

    bedtools merge -i exons.bed | head -n 20
    chr1    11873   12227
    chr1    12612   12721
    chr1    13220   14829
    chr1    14969   15038
    chr1    15795   15947
    chr1    16606   16765
    chr1    16857   17055
    chr1    17232   17368
    chr1    17605   17742
    chr1    17914   18061
    chr1    18267   18366
    chr1    24737   24891
    chr1    29320   29370
    chr1    34610   35174
    chr1    35276   35481
    chr1    35720   36081
    chr1    69090   70008
    chr1    134772  139696
    chr1    139789  139847
    chr1    140074  140566
"""

from pygrass import IntervalFile, chr as chrom, start, end, name, parse_args

parse_args()

file = IntervalFile("../data/exons.bed")

# This is directly supported by pygrass
file.merge_overlaps().print_to_stdout()

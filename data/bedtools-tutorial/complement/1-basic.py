#!/usr/bin/python3

"""
This is equivalent to bedtools command:

    bedtools complement -i exons.bed -g genome.txt \
    > non-exonic.bed
    head non-exonic.bed
    chr1    0   11873
    chr1    12227   12612
    chr1    12721   13220
    chr1    14829   14969
    chr1    15038   15795
    chr1    15947   16606
    chr1    16765   16857
    chr1    17055   17232
    chr1    17368   17605
    chr1    17742   17914

"""

from pygrass import IntervalFile, load_genome_file

load_genome_file("../data/genome.txt")

afile = IntervalFile("../data/exons.bed")

afile.invert().print_to_stdout()

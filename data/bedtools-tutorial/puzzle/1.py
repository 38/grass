#!/usr/bin/python3

# 1. Create a BED file representing all of the intervals in the genome that are NOT exonic.

from pygrass import IntervalFile, load_genome_file

load_genome_file("../data/genome.txt")
input = IntervalFile("../data/exons.bed")

input.invert().print_to_stdout()

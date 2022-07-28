#!/usr/bin/python3

# 4. Are there any exons that are completely overlapped by an enhancer? If so, how many?

from pygrass import IntervalFile, name, length, load_genome_file

load_genome_file("../data/genome.txt")

exons = IntervalFile("../data/exons.bed")
# Instead of producing a filtered BED file, we can filter the input BED file on the fly.
enhancer = IntervalFile("../data/hesc.chromHmm.bed").filter(name.matches("Enhancer"))

# length == length[0] means that the length of the intersection is same to the exon length.
# Which means the overlap is completely covering the exon.
exons.intersect(enhancer).filter(length == length[0]).print_to_stdout()
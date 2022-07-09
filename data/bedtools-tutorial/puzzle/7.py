#!/usr/bin/python3

# 7. Create intervals representing the canonical 2bp splice sites on either side of each exon

from pygrass import load_genome_file, IntervalFile, RustEnv, parse_args, name
from grass_ext import flank

parse_args()

load_genome_file("../data/genome.txt")

exons = IntervalFile("../data/exons.bed")

flank(exons, 2, 2).print_to_stdout()

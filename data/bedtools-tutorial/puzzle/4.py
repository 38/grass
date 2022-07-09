#!/usr/bin/python3

# 4. Are there any exons that are completely overlapped by an enhancer? If so, how many?

from pygrass import IntervalFile, RustEnv, name, length, load_genome_file

load_genome_file("../data/genome.txt")

exons = IntervalFile("../data/exons.bed")
enhancer = IntervalFile("../data/hesc.chromHmm.bed").filter(name.contains("Enhancer"))

exons.intersect(enhancer).filter(length == length[0]).print_to_stdout()

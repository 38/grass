#!/usr/bin/python3

# 3. Count how many exons occur in each 500kb interval (“window”) in the human genome. 

from pygrass import load_genome_file, IntervalFile, item, tag
from grass_ext import make_window, count_overlaps

load_genome_file("../data/genome.txt")

exons = IntervalFile("../data/exons.bed")

count_overlaps(make_window(500000), exons)\
        .format("{bed}\t{count}", bed = item.str_repr, count = tag)\
        .print_to_stdout()
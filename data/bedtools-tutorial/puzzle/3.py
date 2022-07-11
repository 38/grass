#!/usr/bin/python3

# 3. Count how many exons occur in each 500kb interval (“window”) in the human genome. 

from pygrass import load_genome_file, IntervalFile, item, tag, parse_args
from grass_ext import make_window, count_overlaps

parse_args()

load_genome_file("../data/genome.txt")

exons = IntervalFile("../data/exons.bed")

# `make_window` and `count_overlaps` function is a high-level function that build 
# on top of pygrass.
# This demonstrate how to extend pygrass by defining your new reusable function.
count_overlaps(make_window(500000), exons)\
        .format("{region}\t{count}", region = item.str_repr, count = tag)\
        .print_to_stdout()

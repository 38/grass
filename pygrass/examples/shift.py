#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile, strand, If, start, end, length

input = IntervalFile(argv[1])

# This is similar to bedtools shift -m 0.5 -pct

input.alter(
    start = If(strand == "+", start + length * 0.5, start),
    end   = If(strand == "+", end + length * 0.5, end),
).print_to_stdout()

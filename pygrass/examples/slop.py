#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile, start, end, length

input = IntervalFile(argv[1], sorted = True)

altered = input.alter(
    start = start - length * 0.1,
    end = end + length * 0.1,
)

altered.print_to_stdout()
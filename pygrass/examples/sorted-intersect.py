#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile

# Using the "Input" class, GRASS will automatically detect the file format
first_file = IntervalFile(argv[1], sorted = True)
second_file = IntervalFile(argv[2], sorted = True)

# Run the actual intersection
result = first_file.intersect(second_file)

result.print_to_stdout()
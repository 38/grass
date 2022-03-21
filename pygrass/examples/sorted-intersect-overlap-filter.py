#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile, length

# Using the "Input" class, GRASS will automatically detect the file format
first_file = IntervalFile(argv[1], sorted = True)
second_file = IntervalFile(argv[2], sorted = True)

# Run the actual intersection
result = first_file.intersect(second_file)

# Then we can filter the result
# Note: python doesn't allow overloading boolean operators. So use &,| and ~ instead
result.filter(
    (length / length[0] > 0.8) & (length / length[1] > 0.7)
).print_to_stdout()
#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile, start, end

# Using the "Input" class, GRASS will automatically detect the file format
first_file = IntervalFile(argv[1], sorted = True)
second_file = IntervalFile(argv[2], sorted = True)

# Create a virtual input that extends the interval 1000 bp further on each side
windowed_first_file = first_file.alter(
    start = start - 1000,
    end = end + 1000
)

# Since the alter function may break the order of the file, so we need to manually 
# convince GRASS the altered file is still sorted.
windowed_first_file = windowed_first_file.assume_sorted()

# Run the actual intersection
result = windowed_first_file.intersect(second_file)

result.print_to_stdout()

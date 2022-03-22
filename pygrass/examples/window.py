#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile, start, end, item, CmdArg

# Using the "Input" class, GRASS will automatically detect the file format
first_file = IntervalFile(CmdArg(1), sorted = True)
second_file = IntervalFile(CmdArg(2), sorted = True)

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

result.format(
    "{item_a}\t{item_b}", 
    item_a = item[0].str_repr, 
    item_b = item[1].str_repr
).print_to_stdout()

#!/usr/bin/env python3

#TODO: Macro
from sys import argv
from pygrass import IntervalFile, chr, start, end, item

# Using the "Input" class, GRASS will automatically detect the file format
first_file = IntervalFile(argv[1], sorted = True)
second_file = IntervalFile(argv[2], sorted = True)

# Run the actual intersection
result = first_file.intersect(second_file)

group_result = result.group_by(chr, start, end)

group_result.format("{item}\t{count}", item = item[0], count = item.count()).print_to_stdout()
#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile, item, length,start,end

# Using the "Input" class, GRASS will automatically detect the file format
first_file = IntervalFile(argv[1], sorted = True)
second_file = IntervalFile(argv[2], sorted = True)

# Run the actual intersection
result = first_file.left_outer_intersect(second_file)

# We want the out put like "bedtools intersect -wao"
formated_result = result.format(
    "{a}\t{b}\t{length}", 
    a = item[0].str_repr, 
    b = item[1].str_repr, 
    length = length
)

formated_result.print_to_stdout()
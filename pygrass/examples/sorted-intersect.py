from sys import argv
from pygrass import IntervalFile, chr, start, end, item

# Using the "Input" class, GRASS will automatically detect the file format
first_file = IntervalFile(argv[1], sorted = True)
second_file = IntervalFile(argv[2], sorted = True)

# Run the actual intersection
result = first_file.intersect(second_file)

result.group_by(chr[0], start[0], end[0]).format("{interval_a}\t{count}", interval_a = item[0], count = item.count()).print_to_stdout()
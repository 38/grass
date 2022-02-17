from sys import argv
from pygrass import IntervalFile, start, end

# This means the file format should be automatically detected, but it should be a file representing interval
input = IntervalFile(argv[1])

output = input.alter(
    start = start - 100, 
    end = end + 100
)

output.print_to_stdout()
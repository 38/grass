#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile, CmdArg, RustEnv

# Using the "Input" class, GRASS will automatically detect the file format
first_file = IntervalFile(CmdArg(1), sorted = True)
second_file = IntervalFile(CmdArg(2), sorted = True)

# Run the actual intersection
result = first_file.intersect(second_file)

result.print_to_stdout()

#!/usr/bin/env python3

from pygrass import IntervalFile, start, end, CmdArg

# This means the file format should be automatically detected, but it should be a file representing interval
input = IntervalFile(CmdArg(1))

output = input.alter(
    start = start - 100, 
    end = end + 100
)

output.print_to_stdout()

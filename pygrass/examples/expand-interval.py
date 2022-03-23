#!/usr/bin/env python3

from pygrass import IntervalFile, start, end, CmdArg

# You could use specific input file format, for example: Bed3File, Bed6File, BamFile, etc
# Alternatively, you can tell pygrass to detect the file format automatically, in this case
# you can use IntervalFile class to open it
input = IntervalFile(CmdArg(1))

# Then you can use alter method to change the begin and end coordinate of each reocrd
output = input.alter(
    start = start - 100, 
    end = end + 100
)

# Finally, we dump the output to the standard output
output.print_to_stdout()

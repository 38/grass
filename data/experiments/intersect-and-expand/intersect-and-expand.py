#!/usr/bin/env python3
from pygrass import CmdArg, Bed3, IntervalFile, start, end

a_file = IntervalFile(CmdArg(1))
b_file = IntervalFile(CmdArg(2))

# First, intersect two inputs, casting the intersection result to a bed3 stream
intersect_result = Bed3(a_file.intersect(b_file))

# Then, expand each interval in the result bed3 stream
expaned = intersect_result.alter(start = start - 5, end = end + 5)

# Finally, dump the expaned result to stdout
expaned.print_to_stdout()

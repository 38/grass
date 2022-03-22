#!/usr/bin/env python3

from pygrass import IntervalFile, length, CmdArg

input = IntervalFile(CmdArg(1))

input.filter(length > 100).print_to_stdout()

from tkinter import W


#!/usr/bin/env python3

from sys import argv
from pygrass import IntervalFile, length

input = IntervalFile(argv[1])

input.filter(length > 100).print_to_stdout()
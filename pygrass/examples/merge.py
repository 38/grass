from sys import argv
from pygrass import IntervalFile

input = IntervalFile(argv[1])

input.merge().print_to_stdout()
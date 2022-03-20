#!/usr/bin/env python3

from sys import argv
from pygrass import RustEnv, IntervalFile, length

# This is an example for defining high-level operation and extend GRASS
def flank(input, before, after):
    return RustEnv(input = input).iter_processor("""
use grass_runtime::property::*;

input.map(|item| {{
    // Create the interval before the original interval
    let mut before = item.clone();
    before.start = item.start().max({before}) - {before};
    before.end = item.start();

    // Create the interval after the original interval
    let mut after = item;
    after.start = after.end();
    after.end = after.end() + {after};

    // Chain the interval and return it
    std::iter::once(before).chain(std::iter::once(after))
}}).flatten()
""".format(before = before, after = after))


flank(IntervalFile(argv[1]), 10, 10).print_to_stdout()
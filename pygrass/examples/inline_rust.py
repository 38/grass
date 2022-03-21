#!/usr/bin/env python3
from sys import argv
from pygrass import RustEnv, IntervalFile

RustEnv(input_file = IntervalFile(argv[1])).inline_rust("""
use grass_runtime::property::*;

println!("Hello World!");
println!("This is Rust code inlined in pygrass");
for item in input_file {
    println!("chr={} start={} end={}", item.chrom(), item.start(), item.end());
}
""")

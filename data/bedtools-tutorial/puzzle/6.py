#!/usr/bin/python3

# 6. What fraction of the GWAS SNPs are lie in either enhancers or promoters 
# in the hESC data we have?

from pygrass import load_genome_file, IntervalFile, RustEnv, parse_args, name
from grass_ext import count_overlaps

parse_args()

load_genome_file("../data/genome.txt")

gwas = IntervalFile("../data/gwas.bed")
hesc = IntervalFile("../data/hesc.chromHmm.bed").filter(name.matches("Enhancer|Promoter"))

RustEnv(input = count_overlaps(gwas, hesc)).inline_rust("""
    use grass_runtime::property::*;
    let mut total = 0;
    let mut overlapping = 0;
    for item in input {
        if item.tag().unwrap_or(0) > 0 {
            overlapping += 1;
        }
        total += 1;
    }
    println!("{}", overlapping as f64 / total as f64);
""")

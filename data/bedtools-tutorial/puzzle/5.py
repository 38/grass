#!/usr/bin/python3

# 5. What fraction of the GWAS SNPs are exonic?

from pygrass import load_genome_file, IntervalFile, RustEnv, parse_args
from grass_ext import make_window, count_overlaps

parse_args()

load_genome_file("../data/genome.txt")

gwas = IntervalFile("../data/gwas.bed")
exons = IntervalFile("../data/exons.bed")

RustEnv(input = count_overlaps(gwas, exons)).inline_rust("""
    use grass_runtime::property::*;
    let mut total = 0;
    let mut overlaping = 0;
    for item in input {
        if item.tag().unwrap_or(0) > 0 {
            overlaping += 1;
        }
        total += 1;
    }
    println!("{}", overlaping as f64 / total as f64);
""")

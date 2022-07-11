#!/usr/bin/python3

# 5. What fraction of the GWAS SNPs are exonic?

from pygrass import load_genome_file, IntervalFile, RustEnv, parse_args
from grass_ext import make_window, count_overlaps

# 5. What fraction of the GWAS SNPs are exonic?

parse_args()

load_genome_file("../data/genome.txt")

gwas = IntervalFile("../data/gwas.bed")
exons = IntervalFile("../data/exons.bed")

overlaps = count_overlaps(gwas, exons)

# Then we can use arbitrary Rust code to perform further computations.
RustEnv(input = count_overlaps(gwas, exons)).inline_rust("""
    use grass_runtime::property::*;
    println!("{}", input
        .map(|i| (i.tag() != Some(0)) as i64 as f64)
        .enumerate()
        .fold(0f64, |m, (i, v)| (m * i as f64 + v) / (i + 1) as f64)
    );
""")

# And this can be done in a imperative way:
# RustEnv(input = count_overlaps(gwas, exons)).inline_rust("""
#    use grass_runtime::property::*;
#    let mut total = 0;
#    let mut overlapping = 0;
#    for item in input {
#        if item.tag().unwrap_or(0) > 0 {
#            overlapping += 1;
#        }
#        total += 1;
#    }
#    println!("{}", overlapping as f64 / total as f64);
#""")

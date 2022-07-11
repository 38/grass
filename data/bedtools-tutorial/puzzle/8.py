#!/usr/bin/python3

# 8. What is the Jaccard statistic between CpG and hESC enhancers? 
# Compare that to the Jaccard statistic between CpG and hESC promoters. 
# Does the result make sense?

from pygrass import load_genome_file, IntervalFile, name, parse_args
from grass_ext import jaccard

parse_args()

load_genome_file("../data/genome.txt")

def run_jaccard(kind):
    # Currently, grass doesn't handle references to previous queries yet.
    # Thus we need to open the file for each time we run the query.
    hesc = IntervalFile("../data/hesc.chromHmm.bed").filter(name.contains(kind))
    cpg = IntervalFile("../data/cpg.bed")
    jaccard(cpg, hesc)

run_jaccard("Promoter")
run_jaccard("Enhancer")
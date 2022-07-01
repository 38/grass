#!/usr/bin/python3
from pygrass import parse_args, IntervalFile, load_genome_file, score
import genomecovlib

parse_args()

load_genome_file("../data/genome.txt")
file = IntervalFile("../data/exons.bed")

genomecovlib.genomecov(file)

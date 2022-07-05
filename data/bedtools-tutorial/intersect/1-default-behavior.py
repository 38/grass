#!/usr/bin/python3

"""
This is equivalent to bedtools command:

    bedtools intersect -a cpg.bed -b exons.bed | head -5
    chr1    29320   29370   CpG:_116
    chr1    135124  135563  CpG:_30
    chr1    327790  328229  CpG:_29
    chr1    327790  328229  CpG:_29
    chr1    327790  328229  CpG:_29

"""

from pygrass import IntervalFile, chr as chrom, start, end, name, parse_args

parse_args()

# To open a interval file, you can use either reader for the sepcific file format
# or the generic reader like IntervalFile or BedFile. This will make pygrass detect
# the actual file format and emit a proper IR for this file format.
# You can also use os.argv to make a generic intersection program.
afile = IntervalFile("../data/cpg.bed")
bfile = IntervalFile("../data/exons.bed")

# Note: you can also the code to a chained method call like this:
# afile.intersect(bfile).format(...).print_to_stdout()

# Actually run the intersection
intersect = afile.intersect(bfile)

# Format the output matches the bedtools output
formatted = intersect.format(
    "{chrom}\t{start}\t{end}\t{name}", 
    # chrom refers to the chromosome name of the overlap
    chrom = chrom,   
    start = start,   
    end = end,
    # name[0] refers the name of the overlapping interval from file A
    name = name[0],  
)

# Print the results to standard output
formatted.print_to_stdout()

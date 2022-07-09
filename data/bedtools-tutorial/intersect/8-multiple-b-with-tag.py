#!/usr/bin/python3

"""
This is equivalent to bedtools command:

    bedtools intersect -a exons.bed -b cpg.bed gwas.bed hesc.chromHmm.bed -sorted | head
    chr1    11873   11937   NR_046018_exon_0_0_chr1_11874_f 0   +
    chr1    11937   12137   NR_046018_exon_0_0_chr1_11874_f 0   +
    chr1    12137   12227   NR_046018_exon_0_0_chr1_11874_f 0   +
    chr1    12612   12721   NR_046018_exon_1_0_chr1_12613_f 0   +
    chr1    13220   14137   NR_046018_exon_2_0_chr1_13221_f 0   +
    chr1    14137   14409   NR_046018_exon_2_0_chr1_13221_f 0   +
    chr1    14361   14829   NR_024540_exon_0_0_chr1_14362_r 0   -
    chr1    14969   15038   NR_024540_exon_1_0_chr1_14970_r 0   -
    chr1    15795   15947   NR_024540_exon_2_0_chr1_15796_r 0   -
    chr1    16606   16765   NR_024540_exon_3_0_chr1_16607_r 0   -

"""

from pygrass import IntervalFile, item, tag, parse_args

parse_args()

afile = IntervalFile("../data/exons.bed")

# When we need to know which file each record came from in b file, we can assign
# different tag to each b file. Then we can distinguish then by the tag.
bfile_1 = IntervalFile("../data/cpg.bed").tagged(1)
bfile_2 = IntervalFile("../data/gwas.bed").tagged(2)
bfile_3 = IntervalFile("../data/hesc.chromHmm.bed").tagged(3)

bfile = bfile_1.merge_with(bfile_2).merge_with(bfile_3)

afile.intersect(bfile).format(
    "{a}\t{tag}\t{b}",
    a = item[0].str_repr,
    tag = tag[1],
    b = item[1].str_repr
).print_to_stdout()

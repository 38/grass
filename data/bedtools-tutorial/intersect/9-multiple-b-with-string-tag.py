#!/usr/bin/python3

"""
This is equivalent to bedtools command:

    bedtools intersect -a exons.bed -b cpg.bed gwas.bed hesc.chromHmm.bed -sorted -wa -wb -names cpg gwas chromhmm \
      | head -10000 \
      | tail -10
    chr1    27632676    27635124    NM_001276252_exon_15_0_chr1_27632677_f  0   +   chromhmm    chr1    27633213    27635013    5_Strong_Enhancer
    chr1    27632676    27635124    NM_001276252_exon_15_0_chr1_27632677_f  0   +   chromhmm    chr1    27635013    27635413    7_Weak_Enhancer
    chr1    27632676    27635124    NM_015023_exon_15_0_chr1_27632677_f 0   +   chromhmm    chr1    27632613    27632813    6_Weak_Enhancer
    chr1    27632676    27635124    NM_015023_exon_15_0_chr1_27632677_f 0   +   chromhmm    chr1    27632813    27633213    7_Weak_Enhancer
    chr1    27632676    27635124    NM_015023_exon_15_0_chr1_27632677_f 0   +   chromhmm    chr1    27633213    27635013    5_Strong_Enhancer
    chr1    27632676    27635124    NM_015023_exon_15_0_chr1_27632677_f 0   +   chromhmm    chr1    27635013    27635413    7_Weak_Enhancer
    chr1    27648635    27648882    NM_032125_exon_0_0_chr1_27648636_f  0   +   cpg chr1    27648453    27649006    CpG:_63
    chr1    27648635    27648882    NM_032125_exon_0_0_chr1_27648636_f  0   +   chromhmm    chr1    27648613    27649413    1_Active_Promoter
    chr1    27648635    27648882    NR_037576_exon_0_0_chr1_27648636_f  0   +   cpg chr1    27648453    27649006    CpG:_63
    chr1    27648635    27648882    NR_037576_exon_0_0_chr1_27648636_f  0   +   chromhmm    chr1    27648613    27649413    1_Active_Promoter
"""

from pygrass import IntervalFile, item, tag, parse_args

parse_args()

afile = IntervalFile("../data/exons.bed")

# The tag can be a string as well
bfile_1 = IntervalFile("../data/cpg.bed").tagged("cpg")
bfile_2 = IntervalFile("../data/gwas.bed").tagged("gwas")
bfile_3 = IntervalFile("../data/hesc.chromHmm.bed").tagged("chromhmm")

bfile = bfile_1.merge_with(bfile_2).merge_with(bfile_3)

afile.intersect(bfile).format(
    "{a}\t{tag}\t{b}",
    a = item[0].str_repr,
    tag = tag[1],
    b = item[1].str_repr
).print_to_stdout()

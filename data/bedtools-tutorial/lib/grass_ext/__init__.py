"""
This is a proof of concept of high-level API build on top of pygrass.
"""

from pygrass import RustEnv, load_genome_file
from .load_rust import import_rust

@import_rust("genomecov.rs")
def print_genomecov(rust_code, input):
    """ 
        Print the coverage for given range

        Although the functionality needs to be implemented with inline Rust code mostly,
        But GRASS make this code fragments reusable for any intervals. 
        For example, you can also call 
            
            genomecov(input.merge_overlaps())

    """
    RustEnv(input = input).inline_rust(rust_code)

@import_rust("overlapcount.rs")
def count_overlaps(source, input_a, input_b):
    """
        Count number of overlaps for each interval from input_a with intervals from input_b

        The count result is put to the tag for the output interval
    """
    tagged_input_a = input_a.tagged(0)
    tagged_input_b = input_b.tagged(1)
    merged_input = tagged_input_a.merge_with(tagged_input_b)
    return RustEnv(input = merged_input).iter_processor(source)

@import_rust("jaccard.rs")
def jaccard(jaccard_rs, file_a, file_b):
    tagged_a = file_a.tagged(0)
    tagged_b = file_b.tagged(1)
    merged = tagged_a.merge_with(tagged_b)
    return RustEnv(input = merged).inline_rust(jaccard_rs)

@import_rust("window.rs")
def make_window(source, size):
    return RustEnv(bin_size = size).iter_processor(source)

@import_rust("flank.rs")
def flank(source, input, before, after):
    return RustEnv(input = input, before_bases = before, after_bases = after)\
        .iter_processor(source)

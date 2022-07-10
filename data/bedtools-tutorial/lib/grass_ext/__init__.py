"""
This is a proof of concept of high-level API build on top of pygrass.
"""

from pygrass import RustEnv, load_genome_file
from pathlib import Path

_genomecov_src = open(Path(__file__).parent.joinpath("genomecov.rs")).read()
_overlapcount_src = open(Path(__file__).parent.joinpath("overlapcount.rs")).read()
_jaccard_src = open(Path(__file__).parent.joinpath("jaccard.rs")).read()

def print_genomecov(input):
    """ 
        Print the coverage for given range

        Although the functionality needs to be implemented with inline Rust code mostly,
        But GRASS make this code fragement reusable for any intervals. 
        For example, you can also call 
            
            genomecov(input.merge_overlaps())

    """
    RustEnv(input = input).inline_rust(_genomecov_src)

def count_overlaps(input_a, input_b):
    """
        Count number of overlaps for each interval from input_a with intervals from input_b

        The count result is put to the tag for the output interval
    """
    tagged_input_a = input_a.tagged(0)
    tagged_input_b = input_b.tagged(1)
    merged_input = tagged_input_a.merge_with(tagged_input_b)
    return RustEnv(input = merged_input).iter_processor(_overlapcount_src)

def make_window(size):
    return RustEnv().iter_processor("""
            use grass_runtime::{{Genome, Itertools, record::Bed3, algorithm::AssumeSorted}};
            Genome::get_chrom_sizes().into_iter().flat_map(|(name, size)| {{
                let chrom = Genome::query_chr(name);
                let size = size as u32;
                (0..size).step({bin_size}).map(move |start| Bed3 {{
                    chrom,
                    start,
                    end: (start + {bin_size}).min(size)
                }})
            }}).assume_sorted()
        """.format(bin_size = size))

def flank(input, before, after):
    return RustEnv(input = input).iter_processor("""
            use grass_runtime::property::*;
            input.map(|item| {{
                // Create the interval before the original interval
                let mut before = item.clone();
                before.start = item.start().max({before}) - {before};
                before.end = item.start();

                // Create the interval after the original interval
                let mut after = item;
                after.start = after.end();
                after.end = after.end() + {after};

                // Chain the interval and return it
                std::iter::once(before).chain(std::iter::once(after))
            }}).flatten()
        """.format(before = before, after = after))

def jaccard(file_a, file_b):
    tagged_a = file_a.tagged(0)
    tagged_b = file_b.tagged(1)
    merged = tagged_a.merge_with(tagged_b)
    return RustEnv(input = merged).inline_rust(_jaccard_src)

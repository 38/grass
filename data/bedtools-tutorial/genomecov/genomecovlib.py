from pygrass import RustEnv

from pathlib import Path

_genomecov_src = open(Path(__file__).parent.joinpath("genomecov.rs")).read()

def genomecov(input):
    """ Although the functionality needs to be implemented with inline Rust code mostly,
        But GRASS make this code fragement reusable for any intervals. 
        For example, you can also call 
            
            genomecov(input.merge_overlaps())

    """
    RustEnv(input = input).inline_rust(_genomecov_src)
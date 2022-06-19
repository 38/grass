"""A fast, flexible genomics data record processing infrastructure 

PyGRASS enables rapid manipulation of numbers of genomics data record data format, inlcuding but not limited in:

- BED file variantions: BED3, BED4, BED5, BED6, BED12 and BEDPE
- VCF
- GFF
- SAM, BAM and CRAM
- FASTA and FASTQ

Unlike normal python package, PyGRASS doesn't actually use python for data manipulation. 
It capture the semantics that written in Python script and transcompile it down to Rust source code. 
And the actual data manipulation is done by the native binary artifact compiled from the Rust source code. 
This approach gives pygrass a performance that is similar or even faster than carefully written C/C++ program. 
At the time time, pygrass is very flexible and configurable. 

Quick example of pygrass:
```
    from pygrass import *

    file_a = IntervalFile(CmdArg(1))
    file_b = IntervalFile(CmdArg(2))
    file_a.intersect(file_b).print_to_stdout()
```

"""
from abc import abstractclassmethod
from typing import Callable

from pygrass.interval import IntervalBase, IntervalFile, SortedRandomBed3
from pygrass.interval.formats import BedFile, BamFile, Bed3File, CmdArg
from pygrass.interval.cast import Bed3, Bed4, Bed5, Bed6
from pygrass.interval.field_expr import length, start, end, length, name, chr, strand, item, tag, If
from pygrass.backend import DumpIR, BackendBase, RustBackend
from pygrass.record_base import RustEnv, load_genome_file

import os
import importlib

def _load_default_backend():
    if "GRASS_BACKEND_CLASS" in os.environ:
        class_path_token = os.environ["GRASS_BACKEND_CLASS"].split(".")
        module = ".".join(class_path_token[:-1])
        class_name = class_path_token[-1] 
        return getattr(importlib.import_module(module), class_name)
    else:
        return RustBackend


_ActiveBackendCtr : Callable[[], BackendBase] = _load_default_backend()

_backend_session : BackendBase = None

def set_active_backend(backend_type: Callable[[], BackendBase]):
    """Set the currently active GRASS backend. The parameter is any callable that returning a pygrass.BackendBase object"""
    global _backend_session, _ActiveBackendCtr
    _ActiveBackendCtr = backend_type
    _backend_session = None

def get_backend_session() -> BackendBase:
    """Get the currently active GRASS backend session. This allows you to actually touch the backend session object.

    One of the important usage of this is when pygrass is being used interactively, you need either quite the REPR shell
    or manually call pygrass.get_backend_session().flush() to initialize the compile and run process.

    NOTE: When the pygrass is used within a script file, the compile and run process happends at the time the backend session
    gets deleted.
    """
    global _backend_session
    if _backend_session == None:
        _backend_session = _ActiveBackendCtr()
    return _backend_session

def parse_args():
    """Configure GRASS backend according to the command line arguments. """
    get_backend_session().load_config_from_args()

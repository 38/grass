from abc import abstractclassmethod
from typing import Callable

from pygrass.interval import IntervalBase, IntervalFile
from pygrass.interval.formats import BedFile, BamFile, Bed3File
from pygrass.interval.cast import Bed3
from pygrass.interval.field_expr import length, start, end, length, name, chr, strand, item
from pygrass.backend import DumpIR, BackendBase

ActiveBackendCtr : Callable[[], BackendBase] = DumpIR

backend_session = None

def set_active_backend(backend_type: Callable[[], BackendBase]):
    global backend_session, ActiveBackendCtr
    ActiveBackendType = backend_type
    backend_session = None

def get_backend_session() -> BackendBase:
    global backend_session
    if backend_session == None:
        backend_session = ActiveBackendCtr()
    return backend_session
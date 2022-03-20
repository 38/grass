from abc import abstractclassmethod
from typing import Callable

from pygrass.interval import IntervalBase, IntervalFile
from pygrass.interval.formats import BedFile, BamFile, Bed3File
from pygrass.interval.cast import Bed3
from pygrass.interval.field_expr import length, start, end, length, name, chr, strand, item, If
from pygrass.backend import DumpIR, BackendBase, RustBackend
from pygrass.record_base import RustEnv

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

ActiveBackendCtr : Callable[[], BackendBase] = _load_default_backend()

backend_session = None

def set_active_backend(backend_type: Callable[[], BackendBase]):
    global backend_session, ActiveBackendCtr
    ActiveBackendCtr = backend_type
    backend_session = None

def get_backend_session() -> BackendBase:
    global backend_session
    if backend_session == None:
        backend_session = ActiveBackendCtr()
    return backend_session

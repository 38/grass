from pygrass.interval.base import IntervalBase
from pygrass.file_format import detect_file_format
from pygrass.ir import IRBase, OpenFile

import os
import sys

class CmdArg(object):
    def __init__(self, nth):
        self._nth = nth
    def get_value(self):
       return sys.argv[self._nth]

class IntervalFormatBase(IntervalBase):
    def __init__(self, sorted : bool = True):
        super().__init__()

class IntervalFile(IntervalFormatBase):
    def __init__(self, path, sorted : bool = True):
        super().__init__()
        arg_bag = dict()
        actual_path = path if type(path) == str else path.get_value()
        file_type = detect_file_format(actual_path, arg_bag)
        self._sorted = sorted
        if file_type == "cram":
            self._inner = CramFile(path, sorted)
        elif file_type == "bam":
            self._inner = BamFile(path, sorted)
        elif file_type == "bed":
            self._inner = BedFile(path, sorted, **arg_bag)
        elif file_type == "vcf":
            self._inner = VcfFile(path, sorted, **arg_bag)
        else:
            raise RuntimeError("Unsupported file format " + file_type)
    def emit_eval_code(self) -> IRBase:
        return self._inner.emit_eval_code()

class BamFile(IntervalFormatBase):
    def from_stdin(**kwargs):
        ret = BedFile(None, **kwargs)
        ret._target = { "FileNo": 0 }
        return ret
    def __init__(self, path, sorted : bool = True):
        super().__init__()
        self._target = { "CmdArg" : path._nth } if type(path) == CmdArg else { "Path": path }
        self._sorted = sorted
    def emit_eval_code(self) -> IRBase:
        return OpenFile(
            target = self._target,
            format = "Bam",
            sorted = self._sorted
        )

class CramFile(IntervalFormatBase):
    def from_stdin(**kwargs):
        ret = BedFile(None, **kwargs)
        ret._target = { "FileNo": 0 }
        return ret
    def __init__(self, path, sorted : bool = True, ref : str = None):
        super().__init__()
        self._verb = "open-cram"
        self._target = { "CmdArg" : path._nth } if type(path) == CmdArg else { "Path": path }
        self._ref = ref
        self._sorted = sorted
    def emit_eval_code(self) -> IRBase:
        return OpenFile(
            target = self._target,
            format = "Cram",
            sorted = self._sorted,
            ref = self._ref
        )

class BedFile(IntervalFormatBase):
    def from_stdin(**kwargs):
        ret = BedFile(None, **kwargs)
        ret._target = { "FileNo": 0 }
        return ret
    def __init__(self, path, sorted : bool = True, num_of_fields : int = 3, compressed : bool = False):
        super().__init__()
        self._sorted = sorted
        self._target = { "CmdArg" : path._nth } if type(path) == CmdArg else { "Path": path }
        self._compressed = compressed
        self._nof = num_of_fields
    def emit_eval_code(self) -> IRBase:
        return OpenFile(
            target = self._target,
            format = "Bed",
            sorted = self._sorted,
            compression = self._compressed,
            num_of_fields = self._nof
        )

class VcfFile(IntervalFormatBase):
    def from_stdin(**kwargs):
        ret = BedFile(None, **kwargs)
        ret._target = { "FileNo": 0 }
        return ret
    def __init__(self, path, sorted : bool = True, compressed : bool = False):
        super().__init__()
        self._sorted = sorted
        self._target = { "CmdArg" : path._nth } if type(path) == CmdArg else { "Path": path }
        self._compressed = compressed
    def emit_eval_code(self) -> IRBase:
        return OpenFile(
            target = self._target,
            format = "Vcf",
            sorted = self._sorted,
            compression = self._compressed,
        )

class Bed3File(BedFile):
    def __init__(self, path : str, sorted : bool = True):
        super().__init__(path, sorted, num_of_fields= 3)

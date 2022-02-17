from pygrass.interval.base import IntervalBase
from pygrass.file_format import detect_file_format

class IntervalFormatBase(IntervalBase):
    def __init__(self, sorted = True):
        super().__init__()
        self._verb = "dummy-interval-input"
        self._args = []
        self._sorted = sorted
    def emit_eval_code(self):
        if self._sorted:
            return "(assume-sorted ({verb} {args}))".format(verb = self._verb, args = " ".join(self._args))
        else:
            return "({verb} {args})".format(verb = self._verb, args = " ".join(self._args))

class IntervalFile(IntervalFormatBase):
    def __init__(self, path, sorted = True):
        super().__init__()
        arg_bag = dict()
        file_type = detect_file_format(path, arg_bag)
        if file_type == "cram" or file_type == "bam":
            self._inner = BamFile(path, sorted)
        elif file_type == "bed":
            self._inner = BedFile(path, sorted, **arg_bag)
        elif file_type == "vcf":
            self._inner = VcfFile(path, sorted, **arg_bag)
    def emit_eval_code(self):
        return self._inner.emit_eval_code()

class BamFile(IntervalFormatBase):
    def __init__(self, path, sorted = True):
        super().__init__()
        self._verb = "open-bam"
        self._args = ["{}".format(repr(path))]
        self._sorted = sorted

class BedFile(IntervalFormatBase):
    def __init__(self, path, sorted = True, num_of_fields = 3, compressed = False):
        super().__init__()
        self._verb = "open-bed"
        self._sorted = sorted
        self._args = [str(num_of_fields), "{}".format(repr(path)), "compressed" if compressed else "uncompressed"]

class VcfFile(IntervalFormatBase):
    def __init__(self, path, sorted = True, compressed = False):
        super().__init__()
        self._verb = "open-vcf"
        self._sorted = sorted
        self._args = ["{}".format(repr(path)), "compressed" if compressed else "uncompressed"]

class Bed3File(BedFile):
    def __init__(self, path, sorted = True):
        super().__init__(path, sorted, num_of_fields= 3)

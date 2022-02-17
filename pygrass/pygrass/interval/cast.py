from pygrass.interval import IntervalBase

class Bed3(IntervalBase):
    def __init__(self, inner):
        super().__init__()
        self._sorted = inner._sorted
        self._inner = inner
    def emit_eval_code(self):
        return "(cast-to-bed3 {origin})".format(origin = self._inner.lower_to_ir())
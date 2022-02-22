from pygrass.interval import IntervalBase
from pygrass.ir import CastToBed3, IRBase

class Bed3(IntervalBase):
    def __init__(self, inner : IntervalBase):
        super().__init__()
        self._sorted = inner._sorted
        self._inner = inner
    def emit_eval_code(self) -> IRBase:
        return CastToBed3(
            inner = self._inner.lower_to_ir()
        )
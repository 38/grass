from pygrass.interval import IntervalBase
from pygrass.ir import CastToBed, IRBase

class BedBase(IntervalBase):
    def __init__(self, inner : IntervalBase, num_of_fields: int):
        super().__init__()
        self._sorted = inner._sorted
        self._inner = inner
        self._nof = num_of_fields
    def emit_eval_code(self) -> IRBase:
        return CastToBed(self._inner.lower_to_ir(), self._nof, self._sorted)

class Bed3(BedBase):
    def __init__(self, inner: IntervalBase):
        super().__init__(inner, 3)

class Bed4(BedBase):
    def __init__(self, inner: IntervalBase):
        super().__init__(inner, 4)

class Bed5(BedBase):
    def __init__(self, inner: IntervalBase):
        super().__init__(inner, 5)
class Bed6(BedBase):
    def __init__(self, inner: IntervalBase):
        super().__init__(inner, 6)

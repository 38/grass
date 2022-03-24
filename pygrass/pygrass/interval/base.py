from pygrass.interval.field_expr import FieldExpr, make_field_expression
from pygrass.record_base import RecordCollectionBase
from pygrass.ir import AssumeSortedIR, Alter, And, Filter as FilterIR, Format, GroupBy as GroupByIR, IRBase, InlineRust, Merge, Intersection as IntersectionIR, SortedRandomInterval 

class IntervalBase(RecordCollectionBase):
    def __init__(self):
        super().__init__()
        self._sorted = False
    def format(self, fmt : str, **kwargs):
        return FormatedInterval(self, fmt, **kwargs)
    def alter(self, **kwargs):
        return AlteredInterval(self, **kwargs)
    def assume_sorted(self):
        return AssumeSorted(self)
    def filter(self, cond, *args):
        return FilteredInterval(self, cond, *args)
    def merge(self):
        return MergedInterval(self)
    def intersect(self, other):
        return Intersection(self, other, flavor = "inner")
    def outter_intersect(self, other):
        return Intersection(self, other, flavor = "outter")
    def left_outer_intersect(self, other):
        return Intersection(self, other, flavor = "left-outer")
    def right_outer_intersect(self, other):
        return Intersection(self, other, flavor = "right-outer")
    def group_by(self, *args):
        return GroupBy(self, *args)

class SortedRandomBed3(IntervalBase):
    def __init__(self, count, min_len = 100, max_len = 100):
        super().__init__()
        self._min_len = min_len
        self._max_len = max_len
        self._count = count
    def emit_eval_code(self) -> IRBase:
        return SortedRandomInterval(self._count, self._min_len, self._max_len)

class InlineRustIntervalIterator(IntervalBase):
    def __init__(self, env, code, sorted = False):
        super().__init__()
        self._env = env
        self._code = code
        self._sorted = sorted
    def emit_eval_code(self) -> IRBase:
        _env_ir = {}
        for key, expr in self._env.items():
            _env_ir[key] = expr.lower_to_ir()
        return InlineRust(_env_ir, self._code)

class AssumeSorted(IntervalBase):
    def __init__(self, inner: IntervalBase):
        super().__init__()
        self._inner = inner
        self._sorted = True
    def emit_eval_code(self) -> IRBase:
        code = self._inner.lower_to_ir()
        return AssumeSortedIR(
            inner = code
        )
class GroupBy(IntervalBase):
    def __init__(self, inner : IntervalBase, *args):
        super().__init__()
        self._inner = inner
        self._key_func = args
    def emit_eval_code(self) -> IRBase:
        code = self._inner.lower_to_ir()
        return GroupByIR(
            inner = code,
            key_func = [key_comp.lower_to_ir() for key_comp in self._key_func]
        )
    
class FormatedInterval(RecordCollectionBase):
    def __init__(self, inner : IntervalBase, fmt_str : str, **kwargs):
        super().__init__()
        self._inner = inner
        self._fmt_str = fmt_str
        self._values = {} 
        for key in kwargs:
            self._values[key] = make_field_expression(kwargs[key])
    def emit_eval_code(self) -> IRBase:
        values = dict()
        for key in self._values:
            values[key] = self._values[key].lower_to_ir()
        return Format(
            inner = self._inner.lower_to_ir(),
            fmt_str = self._fmt_str,
            values = values
        )
class AlteredInterval(IntervalBase):
    def __init__(self, base : IntervalBase, **kwargs):
        super().__init__()
        self._alters = {}
        self._base = base
        self._sorted = False
        for key, value in kwargs.items():
            self._alters[key] = make_field_expression(value)
    def emit_eval_code(self) -> IRBase:
        code = self._base.lower_to_ir()
        for key, value in self._alters.items():
            code = Alter(
                base = code,
                target_field = key,
                value_expr = value.lower_to_ir()
            )
        return code

class FilteredInterval(IntervalBase):
    def __init__(self, base : IntervalBase, cond : FieldExpr, *args):
        super().__init__()
        self._base = base
        self._conds = [cond] + [make_field_expression(cond) for cond in args]
        self._sorted = self._base._sorted
    def emit_eval_code(self) -> IRBase:
        def fuse_conds(left, right):
            if right - left == 1:
                return self._conds[left].lower_to_ir()
            else:
                return And(
                    lhs = self._conds[left].lower_to_ir(),
                    rhs = fuse_conds(left + 1, right)
                )
        return FilterIR(
            base = self._base.lower_to_ir(),
            cond = fuse_conds(0, len(self._conds))
        )

class MergedInterval(IntervalBase):
    def __init__(self, base : IntervalBase):
        super().__init__()
        self._base = base
        self._sorted = base._sorted
    def emit_eval_code(self) -> IRBase:
        return Merge(
            inner = self._base.lower_to_ir(),
            sorted = self._sorted
        )

class Intersection(IntervalBase):
    def __init__(self, left : IntervalBase, right : IntervalBase, flavor : str = "inner"):
        super().__init__()
        self._flavor = flavor
        self._left = left
        self._right = right
        self._sorted = left._sorted and right._sorted
    def emit_eval_code(self) -> IRBase:
        left_ref = self._left.lower_to_ir()
        right_ref = self._right.lower_to_ir()
        return IntersectionIR(
            lhs = left_ref,
            rhs = right_ref,
            flavor = self._flavor,
            sorted = self._sorted
        )

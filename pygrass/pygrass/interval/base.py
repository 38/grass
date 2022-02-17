from pygrass.interval.field_expr import make_field_expression
from pygrass.record_base import RecordCollectionBase

class IntervalBase(RecordCollectionBase):
    def __init__(self):
        super().__init__()
        self._sorted = False
    def format(self, fmt, **kwargs):
        return FormatedInterval(self, fmt, **kwargs)
    def alter(self, fmt, **kwargs):
        return AlteredInterval(self, **kwargs)        
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

class GroupBy(IntervalBase):
    def __init__(self, inner, *args):
        super().__init__()
        self._inner = inner
        self._key_func = args
    def emit_eval_code(self):
        code = self._inner.lower_to_ir()
        return "(group-by {inner} {expr})".format(
            inner = code, 
            expr = " ".join([key_comp.lower_to_ir() for key_comp in self._key_func])
        )
    
class FormatedInterval(RecordCollectionBase):
    def __init__(self, inner, fmt_str, **kwargs):
        super().__init__()
        self._inner = inner
        self._fmt_str = fmt_str
        self._values = {} 
        for key in kwargs:
            self._values[key] = make_field_expression(kwargs[key])
    def emit_eval_code(self):
        code = self._inner.lower_to_ir()
        arguments = []
        for key,val in self._values.items():
            arguments.append("({key} {value})".format(key = key, value = val.lower_to_ir()))
        return "(format-interval {what} {fmt} {args})".format(
            what = code, 
            fmt = repr(self._fmt_str), 
            args = " ".join(arguments)
        ) 
class AlteredInterval(IntervalBase):
    def __init__(self, base, **kwargs):
        super().__init__()
        self._alters = {}
        self._base = base
        self._sorted = self._base._sorted
        for key, value in kwargs.items():
            self._alters[key] = make_field_expression(value)
    def emit_eval_code(self):
        code = self._base.lower_to_ir()
        for key, value in self._alters.items():
            code = "(assign {} (field-ref {}) {})".format(code, key, value.lower_to_ir())
        return code

class FilteredInterval(IntervalBase):
    def __init__(self, base, cond, *args):
        super().__init__()
        self._base = base
        self._conds = [cond] + [make_field_expression(cond) for cond in args]
        self._sorted = self._base._sorted
    def emit_eval_code(self):
        def fuse_conds(left, right):
            if right - left == 1:
                return self._conds[left].lower_to_ir()
            else:
                return "(and {lhs} {rhs})".format(lhs = self._conds[left].lower_to_ir(), rhs = fuse_conds(left + 1, right))
        return "(filter {what} {how})".format(what = self._base.lower_to_ir(), how = fuse_conds(0, len(self._conds)))

class MergedInterval(IntervalBase):
    def __init__(self, base):
        super().__init__()
        self._base = base
        self._sorted = base._sorted
    def emit_eval_code(self):
        if self._sorted:
            return "(sorted-merge {what})".format(what = self._base.lower_to_ir())
        else:
            return "(unsorted-merge {what})".format(what = self._base.lower_to_ir())

class Intersection(IntervalBase):
    def __init__(self, left, right, flavor = "inner"):
        super().__init__()
        self._flavor = flavor
        self._left = left
        self._right = right
        self._sorted = left._sorted and right._sorted
    def emit_eval_code(self):
        left_ref = self._left.lower_to_ir()
        right_ref = self._right.lower_to_ir()
        prefix = "unsorted"
        if self._sorted:
            prefix = "sorted"
        return "({prefix}-{flavor}-intersect {left} {right})".format(
            prefix = prefix, 
            flavor = self._flavor, 
            left = left_ref, 
            right = right_ref
        )

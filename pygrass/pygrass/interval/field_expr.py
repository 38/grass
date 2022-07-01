from typing import Callable
from pygrass.ir import Add, And, ComponentFieldRef, Cond, Div, Eq, FieldRef, FullRecordRef, GreaterEqualThan, GreaterThan, IRBase, LeftShift, LessEqualThan, LessThan, Mod, Mul, Ne, Neg, Or, Not as NotIR, RecordRef, RightShift, StringRepr, Sub, Xor as XorIR, NumberOfComponents as NumberOfComponentsIR, ConstValue

class FieldExpr(object):
    """
    This is the base class for capturing the semantics of manipulatioin of a interval. 
    """
    def __getitem__(self, idx):
        return ComponentAccess(self, idx)
    def lower_to_ir(self, subs = None) -> IRBase:
        """
        Emit the underlying IR for the captured semantics.

        subs is the subscript that to be applied to the field expression.
        """
        pass
    def logic_and(self, other):
        return Operator(And, self, other)
    def logic_or(self, other):
        return Operator(Or, self, other)
    def logic_not(self, other):
        return Operator(NotIR, self)
    def logic_xor(self, other):
        return Operator(XorIR, self)
    def __and__(self, other):
        return self.logic_and(other)
    def __or__(self, other):
        return self.logic_and(other)
    def __xor__(self, other):
        return self.logic_and(other)
    def __add__(self, other):
        return Operator(Add, self, other)
    def __sub__(self, other):
        return Operator(Sub, self, other)
    def __mul__(self, other):
        return Operator(Mul, self, other)
    def __truediv__(self, other):
        return Operator(Div, self, other)
    def __mod__(self, other):
        return Operator(Mod, self, other)
    def __eq__(self, other):
        return Operator(Eq, self, other)
    def __ne__(self, other):
        return Operator(Ne, self, other)
    def __lt__(self, other):
        return Operator(LessThan, self, other)
    def __gt__(self, other):
        return Operator(GreaterThan, self, other)
    def __le__(self, other):
        return Operator(LessEqualThan, self, other)
    def __ge__(self, other):
        return Operator(GreaterEqualThan, self, other)
    def __rshift__(self, other):
        return Operator(RightShift, self, other)
    def __lshift__(self, other):
        return Operator(LeftShift, self, other)
    def __invert__(self):
        return Operator(Neg, self)


def make_field_expression(expr) -> FieldExpr:
    if not isinstance(expr, FieldExpr):
        return Constant(expr)
    return expr

class ComponentAccess(FieldExpr):
    def __init__(self, target : FieldExpr, idx : int):
        super().__init__()
        self._target = make_field_expression(target)
        self._idx = idx
        self.str_repr = Operator(StringRepr, self)
    def lower_to_ir(self, subs : int = None) -> IRBase:
        if subs != None:
            raise RuntimeError("Cannot set subscription twice")
        return self._target.lower_to_ir(self._idx)

class Operator(FieldExpr):
    def __init__(self, ir_builder : Callable, *args):
        super().__init__()
        self._ir_builder = ir_builder
        self._args = [make_field_expression(e) for e in args]
    def lower_to_ir(self, subs : int = None) -> IRBase:
        ir = [what.lower_to_ir(subs) for what in self._args]
        return self._ir_builder(*ir)

class FieldReference(FieldExpr):
    def __init__(self, name):
        super().__init__()
        self._name = name
    def lower_to_ir(self, subs : int = None) -> IRBase:
        if subs == None:
            return FieldRef(
                field_name = self._name
            )
        else:
            return ComponentFieldRef(
                target = subs,
                field_name = self._name
            )

class RecordReference(FieldExpr):
    def __init__(self):
        super().__init__()
    def lower_to_ir(self, subs : int = None) -> IRBase:
        if subs == None:
            return FullRecordRef()
        else:
            return RecordRef(subs)
    def count(self) -> FieldExpr:
        return NumOfComponents(self)

class NumOfComponents(FieldExpr):
    def __init__(self, inner : FieldExpr):
        super().__init__()
        self._inner = inner
    def lower_to_ir(self, subs : int = None) -> FieldExpr:
        if subs == None:
            return NumberOfComponentsIR()
        else:
            return ConstValue(1)

class Constant(FieldExpr):
    def __init__(self, value : FieldExpr):
        super().__init__()
        self._value = value
    def lower_to_ir(self, subs = None) -> IRBase:
        return ConstValue(self._value)

class If(FieldExpr):
    def __init__(self, cond : FieldExpr, then : FieldExpr, elze : FieldExpr):
        super().__init__()
        self._cond = make_field_expression(cond)
        self._then = make_field_expression(then)
        self._else = make_field_expression(elze)
    def lower_to_ir(self, subs : int = None) -> FieldExpr:
        return Cond(
            cond = self._cond.lower_to_ir(subs),
            then = self._then.lower_to_ir(subs),
            elze = self._else.lower_to_ir(subs),
        )

chr = FieldReference("chrom") 
start = FieldReference("start")
end = FieldReference("end")
name = FieldReference("name")
score = FieldReference("score")
strand = FieldReference("strand")
tag = FieldReference("tag_str")

length = end - start

item = RecordReference()

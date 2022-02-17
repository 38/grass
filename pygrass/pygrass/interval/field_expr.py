def make_field_expression(expr):
    if not isinstance(expr, FieldExpr):
        return Constant(expr)
    return expr

class FieldExpr(object):
    def __getitem__(self, idx):
        return ComponentAccess(self, idx)
    def lower_to_ir(self, subs = None):
        pass
    def logic_and(self, other):
        return Operator('and', self, other)
    def logic_or(self, other):
        return Operator('or', self, other)
    def logic_not(self, other):
        return Operator('not', self)
    def logic_xor(self, other):
        return Operator('xor', self)
    def __and__(self, other):
        return self.logic_and(other)
    def __or__(self, other):
        return self.logic_and(other)
    def __xor__(self, other):
        return self.logic_and(other)
    def __add__(self, other):
        return Operator('+', self, other)
    def __sub__(self, other):
        return Operator('-', self, other)
    def __mul__(self, other):
        return Operator('*', self, other)
    def __truediv__(self, other):
        return Operator('/', self, other)
    def __mod__(self, other):
        return Operator('%', self, other)
    def __eq__(self, other):
        return Operator('==', self, other)
    def __ne__(self, other):
        return Operator('!=', self, other)
    def __lt__(self, other):
        return Operator('<', self, other)
    def __gt__(self, other):
        return Operator('>', self, other)
    def __le__(self, other):
        return Operator('<=', self, other)
    def __ge__(self, other):
        return Operator('>=', self, other)
    def __ge__(self, other):
        return Operator('>=', self, other)
    def __rshift__(self, other):
        return Operator('shr', self, other)
    def __lshift__(self, other):
        return Operator('shl', self, other)
    def __invert__(self, other):
        return Operator('not', self, other)

class ComponentAccess(FieldExpr):
    def __init__(self, target, idx):
        super().__init__()
        self._target = make_field_expression(target)
        self._idx = idx
    def lower_to_ir(self, subs = None):
        if subs != None:
            raise RuntimeError("Cannot set subscription twice")
        return self._target.lower_to_ir(self._idx)

class Operator(FieldExpr):
    def __init__(self, opcode, *args):
        super().__init__()
        self._opcode = opcode
        self._args = [make_field_expression(e) for e in args]
    def lower_to_ir(self, subs = None):
        ir = [what.lower_to_ir(subs) for what in self._args]
        return "({} {})".format(self._opcode, " ".join(ir))

class FieldReference(FieldExpr):
    def __init__(self, name):
        super().__init__()
        self._name = name
    def lower_to_ir(self, subs = None):
        if subs == None:
            return "(field-ref {})".format(self._name)
        else:
            return "(get-component {id} {ref})".format(id = subs, ref = self._name)

class RecordReference(FieldExpr):
    def __init__(self):
        super().__init__()
    def lower_to_ir(self, subs=None):
        if subs == None:
            return "(full-record-ref)"
        else:
            return "(record-ref {id})".format(id = subs)
    def count(self):
        return NumOfComponents(self)

class NumOfComponents(FieldExpr):
    def __init__(self, inner):
        super().__init__()
        self._inner = inner
    def lower_to_ir(self, subs = None):
        if subs == None:
            return "(num-of-components)"
        else:
            return "(const 1)"

class Constant(FieldExpr):
    def __init__(self, value):
        super().__init__()
        self._value = value
    def lower_to_ir(self, subs = None):
        return "(const {})".format(self._value)

class If(FieldExpr):
    def __init__(self, cond, then, elze):
        super().__init__()
        self._cond = make_field_expression(cond)
        self._then = make_field_expression(then)
        self._else = make_field_expression(elze)
    def lower_to_ir(self, subs = None):
        cond = self._cond.lower_to_ir(subs)
        then_clause = self._then.lower_to_ir(subs)
        else_clause = self._then.lower_to_ir(subs)
        return "(cond {} {} {})".format(cond, then_clause, else_clause)

chr = FieldReference("chr")
start = FieldReference("start")
end = FieldReference("end")
name = FieldReference("name")
score = FieldReference("score")
strand = FieldReference("strand")

length = end - start

item = RecordReference()
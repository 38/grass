from abc import abstractclassmethod
from asyncio.tasks import _unregister_task
from functools import wraps
from json import dumps
from typing import Any

class IRBase(object):
    def __init__(self, opcode : str):
        self._opcode = opcode
    def to_dict(self, const_bag: list = None) -> dict[str]:
        ret = dict()
        ret["opcode"] = self._opcode
        return ret
    def to_json(self, indent = None, **kwargs) -> str:
        kwargs["indent"] = indent
        return dumps(self.to_dict(), **kwargs)
    def lift_const_and_jsonify(self, indent = None, **kwargs):
       bag = list()
       kwargs["indent"] = indent
       jsonified = dumps(self.to_dict(bag), **kwargs) 
       return (jsonified, bag)
    def defs(self) -> list[str] :
        ret = []
        for key in dir(self):
            value = self.__getattribute__(key)
            if isinstance(value, IRBase):
                ret.extend(value.defs())
        return ret
    def uses(self) -> list[str] :
        ret = []
        for key in dir(self):
            value = self.__getattribute__(key)
            if isinstance(value, IRBase):
                ret.extend(value.uses())
        return ret
    def imports(self) -> list[str]:
        defs = self.defs()
        uses = self.uses()
        ret = []
        for item in uses:
            if item not in defs:
                ret.append(item)
        return ret
    def exports(self) -> list[str]:
        defs = self.defs()
        uses = self.uses()
        ret = []
        for item in defs:
            if item not in uses:
                ret.append(item)
        return ret

def make_const_bag_ref(value, bag):
    key_id = len(bag)
    bag.append(value)
    return {
        "const_bag_key": key_id
    }
def lift_constant_to_env(dict_value: dict[str, Any], const_bag: list):
    if const_bag == None:
        return
    for key in dict_value.keys():
        if key == "opcode":
            continue
        key_type = type(dict_value[key])
        if key_type == int or key_type == float or key_type == str:
            dict_value[key] = make_const_bag_ref(dict_value[key], const_bag)
def try_lift_const(inner):
    @wraps(inner)
    def _to_dict_and_lift(self, bag : list = None):
        ret = inner(self, bag)
        lift_constant_to_env(ret, bag)
        return ret
    return _to_dict_and_lift

# Actual IR representations

## Random generated bed3
class SortedRandomInterval(IRBase):
    def __init__(self, count: int, min_len: int, max_len: int):
        super().__init__("SortedRandom")
        self._count = count
        self._min_len = min_len
        self._max_len = max_len
    @try_lift_const
    def to_dict(self, bag) -> dict[str]:
        ret = super().to_dict(bag)
        ret["min_length"] = self._min_len
        ret["max_length"] = self._max_len
        return ret

## Inline Rust Source Code
class InlineRust(IRBase):
    def __init__(self, env : dict[str, IRBase], src):
        super().__init__("InlineRust")
        self._env = env
        self._src = src
    def to_dict(self, bag) -> dict[str]:
        ret = super().to_dict(bag)
        ret["env"] = {}
        for key, val in self._env.items():
            if isinstance(val, IRBase):
                value = {"Iter": val.to_dict(bag)}
            elif bag == None:
                value = {"Const": val}
            else:
                value = {"Const": make_const_bag_ref(val, bag)}
            ret["env"][key] = value
        ret["src"] = self._src
        return ret
    def uses(self):
        ret = []
        for var in self._env.value():
            ret.extend(var.uses())
        return ret
    def defs(self):
        ret = []
        for var in self._env.value():
            ret.extend(var.defs())
        return ret

## Genome file manipulation
class LoadGenomeFile(IRBase):
    def __init__(self, path: str):
        super().__init__("LoadGenomeFile")
        self._path = path
    @try_lift_const
    def to_dict(self, bag) -> dict[str]:
        ret = super().to_dict(bag)
        ret["File"] = self._path
        return ret

## Label assignment
class LabelAssignmentBase(IRBase):
    pass

class Let(LabelAssignmentBase):
    def __init__(self, id : str, value : IRBase):
        super().__init__("Let")
        self._id = id
        self._value = value
    def defs(self) -> list[str]:
        return [self._id] + super().defs()
    def to_dict(self, bag = None) -> dict:
        ret = super().to_dict(bag)
        ret["id"] = self._id
        ret["value"] = self._value.to_dict(bag)
        return ret

class Ref(LabelAssignmentBase):
    def __init__(self, id : str):
        super().__init__("Ref")
        self._id = id
    def uses(self) -> list[str]:
        return [self._id] + super().uses()
    def to_dict(self, bag) -> dict[str]:
        ret = super().to_dict(bag)
        ret["id"] = self._id
        return ret

## The Data Sources

class BatchOperationBase(IRBase):
    pass

class OpenFile(BatchOperationBase):
    def __init__(self, target: dict, format : str, sorted : bool = False, ref : str = None, compression : bool = False, num_of_fields : int = 3):
        super().__init__("Open")
        self._target = target 
        self._format = format
        self._ref = ref
        self._compression = compression
        self._num_of_fields = num_of_fields
        self._sorted = sorted
    def to_dict(self, bag) -> dict[str]:
        ret = super().to_dict(bag)
        if "Path" in self._target and bag != None:
            ret["target"] = {
                "Path": make_const_bag_ref(self._target["Path"], bag)
            }
        else:
            ret["target"] = self._target
        ret["format"] = self._format
        ret["num_of_fields"] = self._num_of_fields
        ret["compression"] = self._compression
        ret["sorted"] = self._sorted
        return ret

## Record type casting
class CastToBed(BatchOperationBase):
    def __init__(self, inner : IRBase, num_of_fields: int, sorted: bool):
        super().__init__("CastToBed")
        self._inner = inner
        self._nof = num_of_fields
        self._sorted = sorted
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        ret["num_of_fields"] = self._nof
        ret["sorted"] = self._sorted
        return ret

## Record collection operators
class GroupBy(BatchOperationBase):
    def __init__(self, inner :IRBase, key_func : list[IRBase]):
        super().__init__("GroupBy")
        self._inner = inner
        self._key_func = key_func
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        ret["keys"] = [key.to_dict(bag) for key in self._key_func]
        return ret

class Format(BatchOperationBase):
    def __init__(self, inner : IRBase, fmt_str : str, values : dict[str, Any]):
        super().__init__("Format")
        self._inner = inner
        self._fmt_str = fmt_str
        self._values = values
    
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        ret["fmt_str"] = self._fmt_str
        ret["values"] = dict()
        for key in self._values:
            ret["values"][key] = self._values[key].to_dict(bag)
        return ret

class AssumeSortedIR(BatchOperationBase):
    def __init__(self, inner: IRBase):
        super().__init__("AssumeSorted")
        self._inner = inner
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        return ret

class InternalSort(BatchOperationBase):
    def __init__(self, base: IRBase):
        super().__init__("InternalSort")
        self._inner = base
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        return ret

class Nop(BatchOperationBase):
    def __init__(self, inner: IRBase):
        super().__init__("Nop")
        self._inner = inner
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        return ret

class Alter(BatchOperationBase):
    def __init__(self, base : IRBase, target_field : str, value_expr : IRBase, sorted: bool):
        super().__init__("Alter")
        self._inner = base
        self._target_field = target_field
        self._value_expr = value_expr
        self._sorted = sorted
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        ret["field"] = self._target_field
        ret["value"] = self._value_expr.to_dict(bag)
        ret["sorted"] = self._sorted
        return ret

class Filter(BatchOperationBase):
    def __init__(self, base : IRBase, cond : IRBase):
        super().__init__("Filter")
        self._inner = base
        self._cond = cond
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        ret["cond"] = self._cond.to_dict(bag)
        return ret

class Invert(BatchOperationBase):
    def __init__(self, inner: IRBase):
        super().__init__("Invert")
        self._inner = inner
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        return ret

class AssignTag(BatchOperationBase):
    def __init__(self, inner: IRBase, tag):
        super().__init__("AssignTag")
        self._inner = inner
        self._tag = tag
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        ret["tag"] = self._tag
        return ret

class TwoWayMerge(BatchOperationBase):
    def __init__(self, a: IRBase, b: IRBase):
        super().__init__("TwoWayMerge")
        self._a = a
        self._b = b
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["expr_1"] = self._a.to_dict(bag)
        ret["expr_2"] = self._b.to_dict(bag)
        return ret

class MergeOverlap(BatchOperationBase):
    def __init__(self, inner : IRBase):
        super().__init__("MergeOverlap")
        self._inner = inner
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["inner"] = self._inner.to_dict(bag)
        return ret

class Intersection(BatchOperationBase):
    def __init__(self, lhs : IRBase, rhs : IRBase, flavor : str, sorted : bool):
        super().__init__("Intersection")
        if flavor not in ["inner", "outer", "left-outer", "right-outer"]:
            raise RuntimeError("Unexpected intersection flavor")
        self._flavor = flavor
        self._lhs = lhs
        self._rhs = rhs
        self._sorted = sorted
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["flavor"] = self._flavor
        ret["lhs"] = self._lhs.to_dict(bag)
        ret["rhs"] = self._rhs.to_dict(bag)
        ret["sorted"] = self._sorted
        return ret


## Drain Functions
class WriteFile(BatchOperationBase):
    def __init__(self, target : Any, what : IRBase):
        super().__init__("WriteFile")
        self._what = what
        self._target = target
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["what"] = self._what.to_dict(bag)
        if type(self._target) == str and bag != None:
            ret["target"] = make_const_bag_ref(self._target, bag)
        else:
            ret["target"] = self._target
        return ret

class Count(BatchOperationBase):
    def __init__(self, what : IRBase):
        super().__init__("Count")
        self._what = what
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["what"] = self._what.to_dict(bag)
        return ret

class Limit(BatchOperationBase):
    def __init__(self, what: IRBase, count: int):
        super().__init__("Limit")
        self._what = what
        self._count = count
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["what"] = self._what.to_dict(bag)
        if bag != None:
            ret["count"] = make_const_bag_ref(self._count, bag) 
        else:
            ret["count"] = self._count
        return ret

## The Field Expression
class FieldExpressionBase(IRBase):
    pass

class RuntimeValueBase(FieldExpressionBase):
    def __init__(self, opcode : str):
        super().__init__(opcode)
    def to_dict(self, bag) -> dict[str]:
        return super().to_dict(bag)

class UnaryBase(FieldExpressionBase):
    def __init__(self, opcode : str, operand_key : str, operand : IRBase):
        super().__init__(opcode)
        self._dict = dict[str, IRBase]()
        self._dict[operand_key] = operand
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        for key in self._dict:
            if isinstance(self._dict[key], IRBase):
                ret[key] = self._dict[key].to_dict(bag)
            else:
                ret[key] = self._dict[key]
        return ret

class BinaryBase(FieldExpressionBase):
    def __init__(self, 
        opcode : str, 
        lhs : IRBase, 
        rhs : IRBase, 
        lhs_key : str = "lhs", 
        rhs_key : str = "rhs"
    ):
        super().__init__(opcode)
        self._dict = dict()
        self._dict[lhs_key] = lhs
        self._dict[rhs_key] = rhs
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        for key in self._dict:
            if isinstance(self._dict[key], IRBase):
                ret[key] = self._dict[key].to_dict(bag)
            else:
                ret[key] = self._dict[key]
        return ret

class Cond(FieldExpressionBase):
    def __init__(self, cond : IRBase, then : IRBase, elze : IRBase):
        super().__init__("Cond")
        self._cond = cond
        self._then = then
        self._else = elze
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["cond"] = self._cond.to_dict(bag)
        ret["then"] = self._then.to_dict(bag)
        ret["elze"] = self._else.to_dict(bag)
        return ret

class FieldRef(UnaryBase):
    def __init__(self, field_name : str):
        super().__init__("FieldRef", "field", field_name)

class NumberOfComponents(RuntimeValueBase):
    def __init__(self):
        super().__init__("NumberOfComponents")

class ComponentFieldRef(FieldExpressionBase):
    def __init__(self, target : int, field_name : str):
        super().__init__("ComponentFieldRef")
        self._target = target
        self._field_name = field_name
    def to_dict(self, bag = None) -> dict[str]:
        ret = super().to_dict(bag)
        ret["target"] = self._target
        ret["field_name"] = self._field_name
        return ret

class ConstValue(UnaryBase):
    def __init__(self, value : Any):
        super().__init__("ConstValue", "value", value)
    def to_dict(self, bag) -> dict[str]:
        ret = super().to_dict(bag)
        type_of_value = type(self._dict["value"])
        if bag != None and (type_of_value in [int, float, str]):
            ret["value"] = make_const_bag_ref(self._dict["value"], bag)
        return ret

class FullRecordRef(RuntimeValueBase):
    def __init__(self):
        super().__init__("FullRecordRef")

class RecordRef(IRBase):
    def __init__(self, id : int):
        super().__init__("RecordRef")
        self._id = id
    def to_dict(self, bag) -> dict[str]:
        ret = super().to_dict(bag)
        ret["id"] = self._id
        return ret

class StringRepr(UnaryBase):
    def __init__(self, operand: IRBase):
        super().__init__("StringRepr", "value", operand)

class And(BinaryBase):
    def __init__(self, lhs : IRBase, rhs : IRBase):
        super().__init__("And", lhs, rhs)

class Or(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Or", lhs, rhs)

class Xor(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Or", lhs, rhs)

class Not(UnaryBase):
    def __init__(self, operand : IRBase):
        super().__init__("Not", "operand", operand)

class Neg(UnaryBase):
    def __init__(self, operand : IRBase):
        super().__init__("Neg", "operand", operand)

class Add(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Add", lhs, rhs)

class Sub(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Sub", lhs, rhs)

class Mul(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Mul", lhs, rhs)

class Div(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Div", lhs, rhs)

class Mod(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Mod", lhs, rhs)

class Eq(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Eq", lhs, rhs)

class Ne(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Ne", lhs, rhs)

class LessThan(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("LessThan", lhs, rhs)

class GreaterThan(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("GreaterThan", lhs, rhs)

class LessEqualThan(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("LessEqualThan", lhs, rhs)

class GreaterEqualThan(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("GreaterEqualThan", lhs, rhs)


class RightShift(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("RightShift", lhs, rhs)

class LeftShift(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("LeftShift", lhs, rhs)

class Neg(BinaryBase):
    def __init__(self, lhs : IRBase, rhs: IRBase):
        super().__init__("Neg", lhs, rhs)

class Contains(BinaryBase):
    def __init__(self, lhs: IRBase, rhs: IRBase):
        super().__init__("Contains", lhs, rhs)
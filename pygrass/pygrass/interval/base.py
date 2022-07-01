from pygrass.interval.field_expr import FieldExpr, make_field_expression
from pygrass.record_base import RecordCollectionBase
from pygrass.ir import AssignTag, AssumeSortedIR, Alter, And, Filter as FilterIR, Format, GroupBy as GroupByIR, IRBase, InlineRust, Invert, Limit, MergeOverlap, Intersection as IntersectionIR, SortedRandomInterval, Nop, InternalSort, TwoWayMerge as TwoWayMergeIR

class IntervalBase(RecordCollectionBase):
    """The base class for PyGRASS runtime values which is an iterator of intervals"""
    def __init__(self):
        super().__init__()
        self._sorted = False
    def format(self, fmt : str, **kwargs):
        """Format the iterator, returns a new iterator that contains formatted results. 
        This methods gives a flexibility to what kinds of output PyGRASS can produce. 
        The basic syntax for the formatting is mostly consistent with Rust's formatting string.

        For example, to format an iterator of intervals to comma seperated chromosome, start and end:

        ```
            from pygrass import *
            interval.format("{chrom},{start},{end}", chrom = chrom, start = start, end = end)
        ```

        Each of the formatting argument is a field expression. 
        
        To learn more about field expression, 
        """
        return FormatedInterval(self, fmt, **kwargs)
    def alter(self, **kwargs):
        """ Modify the interval in the iterator.
        This method can be used to make many different interval manipulations. 
        For example:
        - Add 5 bases before and after the input interval (This is similar to `bedtools slop`):
        ```
            from pygrass import *
            
            interval.alter(start = start - 5, end = end - 5)
        ```

        - Change all the name of the interval if the interval size is larger than 1000bp:
        ```
            from pygrass import *

            input.alter(name = If(length > 1000, "LargeRegion", name)
        ```

        - Move all the region 50 bps after the original locus (Similar to `bedtools shift`):
        ```
            from pygrass import *
            
            interval.alter(start = start + 50, end = end + 50)
        ```

        NOTE: All the parameters are described with field expression. 
        To learn more about the field expression, read the documentation for `pygrass.interval.field_expr.FieldExpr`.
        """
        return AlteredInterval(self, **kwargs)
    def assume_sorted(self):
        """Force PyGRASS believe the iterator is a sorted iterator. 

        Normally, PyGRASS can do automatically reasoning about if the iterator is sorted. 
        For example, filtering a sorted iterator will result another sorted iterator; changing name doesn't make a sorted interator unordered. 
        However, it's not possible for PyGRASS to infer if an altered iterator with new coordinate is still sorted. 

        For example, `sorted_interal.alter(start = start - length)` is not ordered any more. 

        But there are still cases, even the coordinate has been changed but the output are still ordered. 
        For example shifting all intervals 50 bp after.

        Thus, we need to use this method to convince PyGRASS the result iterator is still sorted. For example:

        ```sorted_iter.alter(start = start + 1).assume_sorted()```

        NOTE: Calling this method on an actually unsorted iterator may result unexpected outputs or runtime error. 
        """
        return AssumeSorted(self)
    def filter(self, cond, *args):
        """Filter out the intervals doesn't meet the requirement.

        Example:

        - Only keeps the interval with positive strand and a length > 50.
        ```
            input.filter(strand == "+", length > 50)
        ```

        """
        return FilteredInterval(self, cond, *args)
    def sort(self):
        """
        Performe a internal sort on the interval iterator.

        Example:

        ```
            input.sort()
        ```

        Note: This method should be distinguished from `assume_sorted`.
              This method actually sorts the iterator, but `assume_sorted` make GRASS believe the iterator is sorted.
        """
        return SortInterval(self)
    def invert(self):
        """
        Make the complement of the given input. This only works for sorted interator.

        Example:

        - Filter intervals from file_b which is competely non-overlapping with intervals from file_a.
        ```
            file_a.invert().intersect(file_b).filter(length == length[1])
        ```
        """
        return InvertedInterval(self)
    def tagged(self, tag):
        """
        Assign a tag to the interator. 
        This tag is not a part of the output, but it can be use internally 
        to distinguish the source of the interval, especially from a merged iterator.

        Example:

        ```
            input.tagged("A").merge_with(input.tagged("B")).format("{tag}", tag = tag)
        ```
        """
        return TaggedInterval(self, tag)
    def merge_with(self, other):
        """
        Merge two sorted interval iterators.

        This method is similar to `bedtools merge`. 

        Example:

        - Intersect with two file B with file A.
        ```
            file_a.intersect(file_b1.merge_with(file_b2))
        ```
        """
        return TwoWayMerge(self, other)
    def limit(self, n: int):
        """
        Return the first n intervals.

        Example:
        Return top 10 intersections of two files.
        ```
            file_a.intersect(file_b).limit(10)
        ```
        """
        return LimitInterval(self, n)
    def merge_overlaps(self):
        """
        Merge the overlapping intervals in the iterator, this requires the iterator is sorted.

        Example:

        ```
            input.merge_overlaps()
        ```
        """
        return MergedInterval(self)
    def intersect(self, other):
        """
        Intersect two sorted interval iterators.
        This method is similar to `bedtools intersect`.
        This method performs the inner join of two sorted interval iterators.

        Example:

        ```
            file_a.intersect(file_b)
        ```

        """
        return Intersection(self, other, flavor = "inner")
    def outter_intersect(self, other):
        """
        Performe a full outer join of two sorted interval iterators.

        Example:

        ```
            file_a.outter_intersect(file_b)
        ```
        """
        return Intersection(self, other, flavor = "outter")
    def left_outer_intersect(self, other):
        """
        Performe a left outer join of two sorted interval iterators.

        Example:

        ```
            file_a.left_outer_intersect(file_b)
        ```
        """
        return Intersection(self, other, flavor = "left-outer")
    def right_outer_intersect(self, other):
        """
        Performe a right outer join of two sorted interval iterators.

        Example:

        ```
            file_a.right_outer_intersect(file_b)
        ```
        """
        return Intersection(self, other, flavor = "right-outer")
    def group_by(self, *args):
        """
        Group the intervals by the given field expression.

        Example:

        ```
            input.group_by(start)
        ```
        """
        return GroupBy(self, *args)

class SortedRandomBed3(IntervalBase):
    """
    The high level representation of a sorted random bed3 file.

    Some of the usage may need to randomly sample a subset of the whole file.
    This is the iterator that generates a set of sorted non-overlapping BED3 intervals,
    so that we can intersect with any other sorted interval iterator to get a random
    sample of the entire iterator.

    Example:

    ```
        input.intersect(SortRandomBed3(input, n = 100))
    ```

    """
    def __init__(self, count, length = None):
        """
        count: The number of intervals in the iterator.
        length: The length of each interval.
        """
        super().__init__()
        if length != None:
            if type(length) == range:
                self._min_len = length.start
                self._max_len = length.stop
            else:
                self._min_len = length
                self._max_len = length
        else:
            self._min_len = 100
            self._max_len = 100
        self._count = count
        self._sorted = True
    def emit_eval_code(self) -> IRBase:
        """
        Emit the lower level IR code for this iterator.
        """
        return SortedRandomInterval(self._count, self._min_len, self._max_len)

class InlineRustIntervalIterator(IntervalBase):
    """
    The high level representation of an iterator that is defined by a inlined Rust code.

    GRASS IR may not be expressive enough to support all features of genomics interval manipulation.
    So we provide a way to put abitrary Rust code as a supplement to the GRASS IR.

    See documentation of `RustEnv` for more details about usage of this iterator.

    Example:

    ```
        RustEnv(input = RandomInterval(100, 100)).iterator_processor(\"\"\"
            input.map(|interval| {
                interval.start += 100;
                interval.end += 100;
            })
        \"\"\")
    """
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


class InvertedInterval(RecordCollectionBase) :
    def __init__(self, inner: IntervalBase):
        super().__init__()
        self._inner = inner
        self._sorted = True
    def emit_eval_code(self) -> IRBase:
        inner_code = self._inner.lower_to_ir()
        return Invert(inner_code)
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
class TwoWayMerge(IntervalBase):
    def __init__(self, a: IntervalBase, b: IntervalBase):
        super().__init__()
        self._a = a
        self._b = b
        self._sorted = True
    def emit_eval_code(self) -> IRBase:
        code_a = self._a.lower_to_ir()
        code_b = self._b.lower_to_ir()
        return TwoWayMergeIR(code_a, code_b)
class TaggedInterval(IntervalBase):
    def __init__(self, base: IntervalBase, tag):
        super().__init__()
        self._base = base
        self._tag = tag
        self._sorted = self._base._sorted
    def emit_eval_code(self) -> IRBase:
        base = self._base.lower_to_ir()
        return AssignTag(base, self._tag)

class LimitInterval(IntervalBase):
    def __init__(self, what: IntervalBase, count: int):
        super().__init__()
        self._what = what
        self._count = count
        self._sorted = self._what._sorted
    def emit_eval_code(self) -> IRBase:
        return Limit(self._what.lower_to_ir(), self._count)

class AlteredInterval(IntervalBase):
    def __init__(self, base : IntervalBase, **kwargs):
        super().__init__()
        self._alters = {}
        self._base = base
        self._sorted = base._sorted
        for key, value in kwargs.items():
            self._alters[key] = make_field_expression(value)
            if key in ["chrom", "start", "end"]:
                self._sorted = False
    def emit_eval_code(self) -> IRBase:
        code = self._base.lower_to_ir()
        for key, value in self._alters.items():
            code = Alter(
                base = code,
                target_field = key,
                value_expr = value.lower_to_ir(),
                sorted = self._sorted
            )
        return code

class SortInterval(IntervalBase):
    def __init__(self, base: IntervalBase):
        super().__init__()
        self._base = base
        self._sorted = True
    def emit_eval_code(self) -> IRBase:
        if self._base._sorted:
            return Nop(self._base.lower_to_ir())
        else:
            return InternalSort(self._base.lower_to_ir())
    
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
        from pygrass import get_backend_session
        super().__init__()
        self._base = base
        self._sorted = base._sorted
        get_backend_session().add_dependency("genawaiter", features = ["futures03"])
        get_backend_session().add_dependency("futures")
    def emit_eval_code(self) -> IRBase:
        return MergeOverlap(inner = self._base.lower_to_ir())

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

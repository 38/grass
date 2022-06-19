from abc import abstractclassmethod
from functools import wraps

from pygrass.ir import IRBase, InlineRust, Let, Ref, WriteFile, Count, LoadGenomeFile

def _make_free_var_closure():
    nextid = 0
    def _free_var_impl():
        nonlocal nextid
        ret = "_grass_res_{}".format(nextid)
        nextid += 1
        return ret
    return _free_var_impl

_free_var = _make_free_var_closure()

def _send_to_backend(ir : IRBase):
    from pygrass import get_backend_session
    session = get_backend_session()
    session.register_ir(ir)

def _drain_method(origin_method):
    @wraps(origin_method)
    def modified_method(self, *args, **kwargs):
        ir = origin_method(self, *args, **kwargs)
        _send_to_backend(ir)
    return modified_method

def load_genome_file(path):
    _send_to_backend(LoadGenomeFile(path))

class IteratorBase(object):
    pass

class RustEnv(object):
    """introduce a PyGRASS value to the generated Rust code. 
    
    Once the RustEnv object is created, we are able to write inline Rust code in the Python source code. 

    Examples: 

    ```
    from pygrass import *
    RustEnv().inline_rust('println!("Hello world!");')
    ```

    This script will simply print the string "Hello world!" to the stdout

    """
    def __init__(self, **kwargs):
        self._env = kwargs
    def import_crate(self, name, version = None):
        from pygrass import get_backend_session
        get_backend_session().add_dependency(name, version)
        return self
    @_drain_method
    def inline_rust(self, code):
        """
        Inject inline Rust source code. 
        
        The PyGRASS value that introduced by RustEnv object will be avaiable inside the scope of the rust source code fragment. 

        RustEnv(input = IntervalFile("a.bed")).inline_rust('println!("The number of inlines in file is {}", input.count())')
        """
        _env_ir = {}
        for key, expr in self._env.items():
            _env_ir[key] = expr.lower_to_ir()
        return InlineRust(_env_ir, code)
    def iter_processor(self, code):
        """
        Inject inline Rust source code that transforms an iterator of interval records. 
        This is useful when the basic PyGRASS API doesn't have sufficient expression power to describe the operation. 

        For example, we can break each interval in an input bed3 file to two smaller intervals

        ```
            RustEnv(input = IntervalFile("a.bed")).iter_processor(r#"
                input.map(|interval|{
                    use grass_runtime::property::*;
                    use std::iter::once;
                    let chrom = interval.chrom();
                    let start = interval.start();
                    let end = interval.end();
                    let mid = (start + end) / 2;
                    let first_half = Bed3 {
                        chrom: chrom,
                        start: start,
                        end: mid
                    };
                    let second_half = Bed3 {
                        chrom: chrom,
                        start: mid,
                        end: end
                    };
                    once(first_half).chain(once(second_half))
                }).flatten()
            "#)
        ```
        """
        from pygrass.interval.base import InlineRustIntervalIterator
        return InlineRustIntervalIterator(self._env, code)

class RecordCollectionBase(IteratorBase):
    """The base class for any object that representing a PyGRASS value that represents a Rust iterator"""
    def __init__(self):
        self._symbol = None
    @_drain_method
    def print_to_stdout(self) -> IRBase:
        """Print the record collection object to standard output. 

        NOTE: This is a drain method, which will post the captured IR to backend session.
        """
        return WriteFile(1, self.lower_to_ir())
    @_drain_method
    def save_to_file(self, path: str) -> IRBase:
        """Save the record collection object to the file described by the path parameter. 

        NOTE: This is a drain method, which will post the captured IR to backend session.
        """
        return WriteFile(path, self.lower_to_ir())
    @_drain_method
    def count(self) -> IRBase:
        """Count and report the number of record has been seen in the report file.

        NOTE: This is a drain method, which will post the captured IR to backend session.
        """
        return Count(self.lower_to_ir())
    @abstractclassmethod
    def emit_eval_code(self) -> IRBase:
        """The abstract method that emits the actual IR to compute the value described by current object."""
        pass
    def lower_to_ir(self) -> IRBase:
        """The interface method used for PyGRASS value to IR lowering step. 
        When a backend wants to acquire the IR that is equivalent to the underlying PyGRASS object"""
        if self._symbol == None:
            self._symbol = _free_var()
            return Let(self._symbol, self.emit_eval_code()) 
        else:
            return Ref(self._symbol)

class PositionalValueBase(RecordCollectionBase):
    #TODO This is reserved for fasta file
    pass

import os

import json

from pygrass.backend.base import BackendBase
from pygrass.ir import IRBase

from pygrass.rust import expand_macro, execute_job, create_code_compilation_dir

def compose_job_file(ir_list : list[IRBase]):
    ret = dict()
    ret["ir"] = [ir.to_dict() for ir in ir_list]
    ret["working_dir"] = os.curdir
    ret["runtime_source"] = {"dep-kind": "Local", "value": "/home/haohou/source/grass-project/grass/grass-runtime"}
    ret["macro_source"] = {"dep-kind": "Local", "value": "/home/haohou/source/grass-project/grass/grass-macro"}
    #ret["runtime_source"] = {"dep-kind": "CratesIO", "value": None}
    #ret["macro_source"] = {"dep-kind": "CratesIO", "value": None}
    ret["build_flavor"] = "Release"
    return ret

class RustBackendBase(BackendBase):
    def __init__(self):
        super().__init__()
        self._ir_list = []
    def __del__(self):
        if len(self._ir_list) > 0:
            self.flush()
    def register_ir(self, ir: IRBase):
        self._ir_list.append(ir)
    def get_job_str(self):
        job_str = json.dumps(compose_job_file(self._ir_list))
        return job_str
    def _flush_impl(self):
        pass
    def flush(self):
        self._flush_impl()
        self._ir_list = []

class RustBackend(RustBackendBase):
    def _flush_impl(self):
        execute_job(self.get_job_str())

class DumpRustCode(RustBackendBase):
    def _flush_impl(self):
        expand_macro(self.get_job_str())

class CreateRustPackage(RustBackendBase):
    def _flush_impl(self):
        create_code_compilation_dir(self.get_job_str())

class PrintJobDesc(RustBackendBase):
    def _flush_impl(self):
        print(json.dumps(compose_job_file(self._ir_list), indent = 4))
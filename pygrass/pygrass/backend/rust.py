import os
import sys
import json

from shutil import copyfile

from pygrass.backend.base import BackendBase
from pygrass.ir import IRBase

from pygrass.rust import expand_macro, execute_job, create_code_compilation_dir, build_job_and_copy

def _compose_job_file(ir_list : list[IRBase], argv, build_flavor, const_bag = None):
    ret = dict()
    ret["ir"] = []
    for ir in ir_list:
        ret["ir"].append(ir.to_dict(const_bag))
    ret["working_dir"] = os.curdir
    ret["runtime_source"] = {"dep-kind": "Local", "value": "/home/haohou/source/grass-project/grass/grass-runtime"}
    ret["macro_source"] = {"dep-kind": "Local", "value": "/home/haohou/source/grass-project/grass/grass-macro"}
    #ret["runtime_source"] = {"dep-kind": "CratesIO", "value": None}
    #ret["macro_source"] = {"dep-kind": "CratesIO", "value": None}
    ret["build_flavor"] = build_flavor
    ret["cmdline_args"] = argv
    ret["const_bag_types"] = []
    if const_bag != None:
        for value in const_bag:
            ty = type(value)
            if ty == str:
                ret["const_bag_types"].append("str")
            elif ty == int:
                ret["const_bag_types"].append("i64")
            elif ty == float:
                ret["const_bag_types"].append("f64")
            else:
                raise RuntimeError("Unsupported constant bag type")
    # TODO: pass the constant bag and hookup with Rust code
    ret["env_vars"] = dict()
    return ret

class RustBackendBase(BackendBase):
    def __init__(self, build_flavor = None, debug = False, profiling = False):
        super().__init__()
        self._ir_list = []
        self._argv = sys.argv[1:]
        self._const_bag = None if os.environ.get("NO_PASS_CONST_WITH_ENV", "0") == "1" else list()
        if build_flavor == None and debug == False and profiling == False:
            self._build_flavor = os.environ["BUILD_FLAVOR"] if os.environ.get("BUILD_FLAVOR") in ["Debug", "Release", "ReleaseWithDebugInfo"] else "Release"
            if os.environ.get("DEBUG") == "1":
                self._build_flavor = "Debug"
            elif os.environ.get("PROF") == "1":
                self._build_flavor = "ReleaseWithDebugInfo"
        else:
            self._build_flavor = build_flavor if build_flavor in ["Debug", "Release", "ReleaseWithDebugInfo"] else "Release"
    def __del__(self):
        if len(self._ir_list) > 0:
            self.flush()
    def register_ir(self, ir: IRBase):
        self._ir_list.append(ir)
    def get_job_obj(self):
        return _compose_job_file(self._ir_list, self._argv, self._build_flavor, self._const_bag)
    def get_job_str(self):
        job = self.get_job_obj() 
        job_str = json.dumps(job)
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

class DumpJobDesc(RustBackendBase):
    def _flush_impl(self):
        job = self.get_job_obj()
        print(json.dumps(job, indent = 4))

class BuildBinary(RustBackendBase):
    def _flush_impl(self):
        output_path = os.environ.get("GRASS_BIN_OUTPUT", "grass_artifact")
        artifact_path = build_job_and_copy(self.get_job_str(), output_path)

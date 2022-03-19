import json
from pygrass.backend.base import BackendBase
from pygrass.ir import IRBase
from pygrass.rust import expand_macro, execute_job
from tempfile import NamedTemporaryFile
import subprocess
import os
def compose_job_file(ir : IRBase):
    ret = dict()
    ret["ir"] = ir.to_dict()
    ret["working_dir"] = os.curdir
    ret["runtime_source"] = {"dep-kind": "Local", "value": "/home/haohou/source/grass-project/grass/grass-runtime"}
    ret["macro_source"] = {"dep-kind": "Local", "value": "/home/haohou/source/grass-project/grass/grass-macro"}
    ret["build_flavor"] = "Release"
    return ret

class RustBackend(BackendBase):
    def __init__(self):
        super().__init__()
    def register_ir(self, ir: IRBase):
        execute_job(json.dumps(compose_job_file(ir)))

class DumpRustCode(BackendBase):
    def __init__(self):
        super().__init__()
    def register_ir(self, ir: IRBase):
        expand_macro(json.dumps(compose_job_file(ir)))
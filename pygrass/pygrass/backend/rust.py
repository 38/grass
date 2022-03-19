import json
from pygrass.backend.base import BackendBase
from pygrass.ir import IRBase
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
    def __init__(self, driver_executable = "../target/debug/grass-driver"):
        super().__init__()
        self._driver = driver_executable
    def register_ir(self, ir: IRBase):
        file = NamedTemporaryFile("w")
        job = compose_job_file(ir)
        json.dump(job, file)
        file.flush()
        ret = subprocess.Popen(
            args = [self._driver, "exec", file.name],
        ).wait()
        if ret != 0:
            raise RuntimeError("Rust backend returns a failure")

class DumpRustCode(BackendBase):
    def __init__(self, driver_executable = "../target/debug/grass-driver"):
        super().__init__()
        self._driver = driver_executable
    def register_ir(self, ir: IRBase):
        file = NamedTemporaryFile("w")
        job = compose_job_file(ir)
        json.dump(job, file)
        file.flush()
        ret = subprocess.Popen(
            args = [self._driver, "expand", file.name],
        ).wait()
        if ret != 0:
            raise RuntimeError("Rust backend returns a failure")
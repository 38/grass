from argparse import REMAINDER, ArgumentParser
import os
import sys
import json

from shutil import copyfile

from pygrass.backend.base import BackendBase
from pygrass.ir import IRBase

from pygrass.rust import expand_macro, execute_job, create_code_compilation_dir, build_job_and_copy

class RustBackendBase(BackendBase):
    def _compose_job_file(self):
        ret = dict()
        ret["ir"] = []
        for ir in self._ir_list:
            ret["ir"].append(ir.to_dict(self._const_bag))
        ret["working_dir"] = os.curdir
        ret["runtime_source"] = self._runtime_source
        ret["macro_source"] = self._macro_source
        ret["build_flavor"] = self._build_flavor
        ret["cmdline_args"] = self._argv
        ret["const_bag_types"] = []
        if self._const_bag != None:
            for value in self._const_bag:
                ty = type(value)
                if ty == str:
                    ret["const_bag_types"].append("str")
                elif ty == int:
                    ret["const_bag_types"].append("f64")
                elif ty == float:
                    ret["const_bag_types"].append("f64")
                else:
                    raise RuntimeError("Unsupported constant bag type")
        ret["env_vars"] = dict()
        if self._const_bag != None:
            ret["env_vars"]["__GRASS_CONST_BAG"] = ""
            for value in self._const_bag:
                ty = type(value)
                if ty == str:
                    value = value.replace('\\', '\\\\')
                    value = value.replace(';', '\;')
                else:
                    value = str(value)
                if ret["env_vars"]["__GRASS_CONST_BAG"] != "":
                    ret["env_vars"]["__GRASS_CONST_BAG"] += ";"
                ret["env_vars"]["__GRASS_CONST_BAG"] += value
        ret["deps"] = self._deps
        ret["use_cache"] = self._enable_cache
        ret["update_cache"] = self._update_cache
        ret["macro_features"] = self._macro_features
        ret["runtime_features"] = self._runtime_features
        if self._cache_root != None:
            ret["cache_root"] = self._cache_root
        return ret
    def _build_crate_source(self, source):
        if source == None:
            source = {"dep-kind": "CratesIO", "value": None }
        elif source.startswith(("http", "https", "git", "ssh")):
            source = {"dep-kind": "Git", "value": source }
        else:
            source = {"dep-kind": "Local", "value": source}
        return source
    def load_env_conf(self):
        self.enable_env_const_bag(os.environ.get("ENV_CONST_BAG", "1") == "1")
        self.set_build_flavor(os.environ.get("BUILD_FLAVOR", "Release"))
        self.set_args(sys.argv[1:])
        runtime_path = os.environ.get("GRASS_RUNTIME_PATH") 
        if runtime_path != None:
            self.set_runtime_source(runtime_path + "/grass-runtime")
            self.set_macro_source(runtime_path + "/grass-macro") 
    def set_build_flavor(self, flavor : str):
        flavor = flavor.upper()
        if flavor == "DEBUG":
            self._build_flavor = "Debug"
        elif flavor == "PROF":
            self._build_flavor = "ReleaseWithDebugInfo"
        else:
            self._build_flavor = "Release"
    def enable_env_const_bag(self, value):
        if value == True and self._const_bag == None:
            self._const_bag = list()
        elif value == False and self._const_bag != None:
            self._const_bag = None
    def add_dependency(self, crate_name, source: str = None, version = None, features = [], default_features = True):
        if crate_name in self._imported_crates:
            return
        source = self._build_crate_source(source)
        dep = {
            "name": crate_name,
            "source": source,
            "version": version,
            "features": features,
            "default_features": default_features,
        }
        self._deps.append(dep)
        self._imported_crates.add("crate_name")
    def enable_runtime_feature(self, feature_name):
        if feature_name not in self._runtime_features:
            self._runtime_features.append(feature_name)
    def enable_macro_feature(self, feature_name):
        if feature_name not in self._macro_features:
            self._macro_features.append(feature_name)
    def set_args(self, argv):
        self._argv = argv
    def add_env_vars(self, name, value):
        self._environ["name"] = value
    def set_runtime_source(self, source):
        self._runtime_source = self._build_crate_source(source)
    def set_macro_source(self, source):
        self._macro_source = self._build_crate_source(source)
    def enable_cache(self, value = True):
        self._enable_cache = value
    def update_cache(self, value = True):
        self._update_cache = value
    def cache_root(self, value):
        self._cache_root = value
    def __init__(self):
        super().__init__()
        self._imported_crates = set(["grass-runtime", "grass-macro"])
        self._enable_cache = True
        self._update_cache = True
        self._cache_root = None
        self._const_bag = None
        self._ir_list = []
        self._argv = sys.argv[1:]
        self._deps = list()
        self._environ = dict()
        self._runtime_features = []
        self._macro_features = []
        self._runtime_source = self._build_crate_source(None) 
        self._macro_source = self._build_crate_source(None)
        self.load_env_conf()
    def __del__(self):
        if len(self._ir_list) > 0:
            self.flush()
    def register_ir(self, ir: IRBase):
        self._ir_list.append(ir)
    def get_job_obj(self):
        return self._compose_job_file()
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

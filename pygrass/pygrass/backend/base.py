from argparse import REMAINDER, ArgumentParser
import os
import sys
from pygrass.ir import IRBase

def _execute_with_backend(backend, additional_env = {}):
    current_backend = os.environ.get("GRASS_BACKEND_CLASS", "pygrass.backend.RustBackend")
    if current_backend != backend:
        env = dict(os.environ)
        env["GRASS_BACKEND_CLASS"] = backend
        for key in additional_env:
            env[key] = additional_env[key]
        argv = [sys.executable] + sys.argv
        os.execve(sys.executable, argv, env)
        raise RuntimeError("exec error")

class BackendBase(object):
    def __init__(self):
        pass
    def register_ir(self, ir : IRBase):
        pass
    def join(self):
        pass
    def load_config_from_args(self):
        parser = ArgumentParser()
        parser.add_argument( "--release", 
            help = "Build the Rust artifact in release mode, enable all the compiler optimizations, strip all debug infomation",
            dest = "build_flavor",
            action = "store_const",
            const = "Release")
        parser.add_argument( "--debug", 
            help = "Build the Rust artifact in debug mode, disable most optimization and keep debug information",
            dest = "build_flavor",
            action = "store_const",
            const = "Debug")
        parser.add_argument( "--profiling", 
            help = "Build the Rust artifact in profiling mode, enable all optimization and keep debug information",
            dest = "build_flavor",
            action = "store_const",
            const = "Prof")
        parser.add_argument("--disable-env-const-bag",
            help = "Do not use environment variable for runtime constant passing, this will generate better optimized but less general binary artifact",
            dest = "no_use_const_bag",
            action = "store_const",
            const = True,
            default= False
        )
        parser.add_argument("--runtime",
            metavar = "runtime-path-or-url",
            help = "Use a user specified runtime crate instead of the one pulled from crates.io",
            type = str,
            dest = 'runtime'
        )
        parser.add_argument("--macro",
            metavar = "macro-path-or-url",
            help = "Use a user specified procedual macro crate instead of the one pulled from crates.io",
            type = str,
            dest = 'macro'
        )
        parser.add_argument("--dump-ir", 
            help = "Do not actually build and run the artifact, only print the GRASS IR",
            dest = "dump_ir",
            action = "store_const",
            default = False,
            const = True,
        )
        parser.add_argument("--dump-job", 
            help = "Do not actually build and run the artifact, only print the job description file",
            dest = "dump_job",
            action = "store_const",
            default = False,
            const = True,
        )
        parser.add_argument("--dump-rust-source", 
            help = "Do not actually build and run the artifact, only print the generated Rust code after macro expansion",
            dest = "dump_rust",
            action = "store_const",
            default = False,
            const = True,
        )
        parser.add_argument("--create-package", 
            help = "Do not actually build and run the artifact, only create the underlying Rust package",
            dest = "create_crate",
            action = "store_const",
            default = False,
            const = True,
        )
        parser.add_argument("--disable-cache", 
            help = "Do not use the binary cache, force GRASS build fresh artifact for each run",
            dest = "disable_cache",
            action = "store_const",
            default = False,
            const = True,
        )
        parser.add_argument("--force-update-cache",
            help = "Force GRASS build fresh Rust artifact and update the binary cache",
            dest = "force_update",
            action = "store_const",
            default = False,
            const = True,
        )
        parser.add_argument("--cache-root",
            help = "Force GRASS build fresh Rust artifact and update the binary cache",
            dest = "cache_root",
            metavar = "path"
        )
        default_bin = sys.argv[0][:-3] if sys.argv[0].endswith(".py") else "a.out"
        parser.add_argument("--build-bin",
            help = "Build artifact and copy it to specified location (default: " + default_bin + ")",
            dest = "bin_path",
            metavar = "output-path",
            type = str,
            action = "store",
            nargs = "?",
            default = None,
            const = default_bin,
        )
        parser.add_argument('remaining', nargs=REMAINDER)
        args = parser.parse_args() 
        current_backend = os.environ.get("GRASS_BACKEND_CLASS", "pygrass.backend.RustBackend")
        if args.dump_ir: 
            _execute_with_backend("pygrass.backend.DumpIR")
        if args.dump_job: 
            _execute_with_backend("pygrass.backend.DumpJobDesc")
        if args.dump_rust: 
            _execute_with_backend("pygrass.backend.DumpRustCode")
        if args.create_crate: 
            _execute_with_backend("pygrass.backend.CreateRustPackage")
        if args.bin_path != None: 
            _execute_with_backend("pygrass.backend.BuildBinary", {"GRASS_BIN_OUTPUT": args.bin_path})
        if args.build_flavor != None:
            self.set_build_flavor(args.build_flavor)
        self.enable_env_const_bag(not args.no_use_const_bag)
        if args.runtime != None:
            self.set_runtime_source(args.runtime)
        if args.macro != None:
            self.set_macro_source(args.macro)
        if args.disable_cache:
            self.enable_cache(False)
            self.update_cache(False)
        if args.force_update:
            self.enable_cache(False)
            self.update_cache(True)
        if args.cache_root != None:
            self.cache_root(args.cache_root)
        sys.argv = [sys.argv[0]]+ args.remaining
        self.set_args(sys.argv[1:])
        if current_backend == "pygrass.backend.BuildBinary":
            self.enable_env_const_bag(False)
    def set_build_flavor(self, flavor : str):
        pass
    def enable_env_const_bag(self, value):
        pass
    def add_dependency(self, crate_name, source: str = None, version = None, features = [], default_features = True):
        pass
    def set_args(self, argv):
        pass
    def add_env_vars(self, name, value):
        pass
    def set_runtime_source(self, source):
        pass
    def set_macro_source(self, source):
        pass
    def enable_cache(self, value = True):
        pass
    def update_cache(self, value = True):
        pass
    def cache_root(self, value):
        pass
    def enable_runtime_feature(self, feature_name):
        pass
    def enable_macro_feature(self, feature_name):
        pass
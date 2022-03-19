#!/usr/bin/env python3
import os
import sys
import pathlib

from setuptools import setup
from setuptools.command.test import test as TestCommand
from setuptools.command.sdist import sdist as SdistCommand

def install_required_package(package_name, module_name):
    import subprocess
    import importlib

    errno = subprocess.call([sys.executable, "-m", "pip", "install", package_name])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        return importlib.import_module(module_name)

try:
    from setuptools_rust import RustExtension
except ImportError:
    setuptools_rust = install_required_package("setuptools-rust", "setuptools_rust")
    RustExtension = setuptools_rust.RustExtension

try:
    import toml
except ModuleNotFoundError:
    toml = install_required_package("toml", "toml")

class CargoModifiedSdist(SdistCommand):
    def make_release_tree(self, base_dir, files):
        """Stages the files to be included in archives"""
        files.append("Cargo.toml")
        files += [str(f) for f in pathlib.Path("src").glob("**/*.rs") if f.is_file()]
        super().make_release_tree(base_dir, files)

def read_package_version():
    manifest = toml.load(open("Cargo.toml", "r"))
    return manifest["package"]["version"]


setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = ["numpy"]

setup(
    name="pygrass",
    version= read_package_version(),
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    long_description=open("README.md").read(),
    long_description_content_type='text/markdown',
    packages=["pygrass", "pygrass.backend", "pygrass.interval"],
    rust_extensions=[RustExtension("pygrass.rust", "Cargo.toml", debug="DEBUG" in os.environ)],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
    cmdclass={"sdist": CargoModifiedSdist},
)

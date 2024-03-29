#!/usr/bin/zsh
export GRASS_BACKEND_CLASS="pygrass.backend.DumpIR"

PROJECT_ROOT=$(readlink -f $(dirname $(readlink -f $0))/..)

cd ${PROJECT_ROOT}/pygrass
./setup.py build

export PYTHONPATH=${PROJECT_ROOT}/pygrass/build/lib

for example in ${PROJECT_ROOT}/pygrass/examples/*.py
do
    python3 ${example} a.bed b.bed c.bed d.bed e.bed >  ${PROJECT_ROOT}/data/ir/$(basename ${example}).json
done

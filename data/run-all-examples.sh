#!/usr/bin/zsh
export DEFAULT_GRASS_BACKEND="pygrass.backend.DumpIR"

PROJECT_ROOT=$(readlink -f $(dirname $(readlink -f $0))/..)

export PYTHONPATH=${PROJECT_ROOT}/pygrass

for example in ${PROJECT_ROOT}/pygrass/examples/*.py
do
    python3 ${example} a.bed b.bed c.bed d.bed e.bed >  ${PROJECT_ROOT}/data/ir/$(basename ${example}).json
done

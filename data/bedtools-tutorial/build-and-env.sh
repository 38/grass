REPO_ROOT=$(git rev-parse --show-toplevel)
pushd $REPO_ROOT/pygrass
python3 setup.py build
popd

export PYTHONPATH=${REPO_ROOT}/pygrass/build/lib
export GRASS_RUNTIME_PATH=${REPO_ROOT}

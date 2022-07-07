REPO_ROOT=$(git rev-parse --show-toplevel)
pushd $REPO_ROOT/pygrass
python3 setup.py build
popd

export PYTHONPATH=`echo ${REPO_ROOT}/pygrass/build/lib* | head -n 1`
export GRASS_RUNTIME_PATH=${REPO_ROOT}

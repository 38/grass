pushd $(dirname $0)
REPO_ROOT=$(git rev-parse --show-toplevel)
popd
pushd $REPO_ROOT/pygrass
python3 setup.py build
popd

export PYTHONPATH=$(echo ${REPO_ROOT}/pygrass/build/lib* | head -n 1):${REPO_ROOT}/data/bedtools-tutorial/lib
export GRASS_RUNTIME_PATH=${REPO_ROOT}

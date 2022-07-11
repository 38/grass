def _load_rust_src(path):
    from pathlib import Path
    with Path(__file__).parent.joinpath(path).open() as f:
        return f.read()

class import_rust(object):
    def __init__(self, name):
        self._src = _load_rust_src(name)
    def __call__(self, function):
        def wrapper(*args, **kwargs):
            return function(self._src, *args, **kwargs)
        return wrapper

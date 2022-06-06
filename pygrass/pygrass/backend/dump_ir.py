from pygrass.backend.base import BackendBase
from pygrass.ir import IRBase

import os

class DumpIR(BackendBase):
    def __init__(self):
        super().__init__()
    def register_ir(self, ir: IRBase):
        if os.environ.get("NO_PASS_CONST_WITH_ENV", "0") == "1":
            print(ir.to_json(indent = 4))
        else:
            ir, bag = ir.lift_const_and_jsonify(indent = 4)
            print(ir)
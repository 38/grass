from pygrass.backend.base import BackendBase
from pygrass.ir import IRBase

class DumpIR(BackendBase):
    def __init__(self):
        super().__init__()
    def register_ir(self, ir: IRBase):
        print(ir.to_json(indent = 4))
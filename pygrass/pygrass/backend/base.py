from pygrass.ir import IRBase


class BackendBase(object):
    def __init__(self):
        pass
    def register_ir(self, ir : IRBase):
        pass
    def join(self):
        pass

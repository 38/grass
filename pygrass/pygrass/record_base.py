from abc import abstractclassmethod

from pygrass.ir import IRBase, Let, Ref, WriteFile, Count

def _make_free_var_closure():
	nextid = 0
	def _free_var_impl():
		nonlocal nextid
		ret = "_grass_res_{}".format(nextid)
		nextid += 1
		return ret
	return _free_var_impl

_free_var = _make_free_var_closure()

def send_to_backend(ir : IRBase):
	from pygrass import get_backend_session
	session = get_backend_session()
	session.register_ir(ir)

def drain_method(origin_method):
	def modified_method(self, *args, **kwargs):
		ir = origin_method(self, *args, **kwargs)
		send_to_backend(ir)
	return modified_method

class IteratorBase(object):
	pass

class RecordCollectionBase(IteratorBase):
	def __init__(self):
		self._symbol = None
	@drain_method
	def print_to_stdout(self) -> IRBase:
		return WriteFile(1, self.lower_to_ir())
	@drain_method
	def save_to_file(self, path: str) -> IRBase:
		return WriteFile(path, self.lower_to_ir())
	@drain_method
	def count(self) -> IRBase:
		return Count(self.lower_to_ir())
	@abstractclassmethod
	def emit_eval_code(self) -> IRBase:
		pass
	def lower_to_ir(self) -> IRBase:
		if self._symbol == None:
			self._symbol = _free_var()
			return Let(self._symbol, self.emit_eval_code()) 
		else:
			return Ref(self._symbol)

class PositionalValueBase(RecordCollectionBase):
	#TODO This is reserved for fasta file
	pass

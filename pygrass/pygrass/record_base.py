from abc import abstractclassmethod

def _make_free_var_closure():
	nextid = 0
	def _free_var_impl():
		nonlocal nextid
		ret = "_grass_res_{}".format(nextid)
		nextid += 1
		return ret
	return _free_var_impl

_free_var = _make_free_var_closure()

def send_to_backend(ir):
	# TODO
	print(ir)

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
	def print_to_stdout(self):
		return "(print-to-stdout {ir})".format(ir = self.lower_to_ir())
	@drain_method
	def save_to_file(self, path):
		return "(save-to-file {ir} \"{path}\")".format(ir = self.lower_to_ir(), path = path)
	@drain_method
	def count(self):
		return "(count {ir})".format(ir = self.lower_to_ir())
	@abstractclassmethod
	def emit_eval_code(self):
		pass
	def lower_to_ir(self):
		if self._symbol == None:
			self._symbol = _free_var()
			return "(label {symbol} {expr})".format(symbol = self._symbol, expr = self.emit_eval_code())
		else:
			return "(ref {symbol})".format(symbol = self._symbol)

class PositionalValueBase(RecordCollectionBase):
	pass

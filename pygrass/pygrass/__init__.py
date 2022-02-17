from abc import abstractclassmethod

from pygrass.interval import IntervalBase, IntervalFile
from pygrass.interval.formats import BedFile, BamFile, Bed3File
from pygrass.interval.cast import Bed3
from pygrass.interval.field_expr import length, start, end, length, name, chr, strand, item
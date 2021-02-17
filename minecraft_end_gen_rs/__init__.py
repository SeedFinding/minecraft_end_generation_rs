from . import _native
from ctypes import *
from enum import IntEnum

class CtypesEnum(IntEnum):
    """A ctypes-compatible IntEnum superclass."""
    @classmethod
    def from_param(cls, obj):
        return int(obj)


class EndBiomes(CtypesEnum):
    Default = 0,
    TheEnd = 9,
    SmallEndIslands = 40,
    EndMidlands = 41,
    EndHighlands = 42,
    EndBarrens = 43,


class EndGen(Structure):
    _pack_ = 8
    _fields_ = [("seed", c_uint64),
                ("_noise", c_void_p)]


def create_new_end(seed) -> POINTER(EndGen):
    return _native.lib.create_new_end(seed)

def get_biome_2d(end_gen: POINTER(EndGen), x: c_int32, z: c_int32) -> EndBiomes:
    return _native.lib.get_biome_2d(end_gen, x, y, z)

def get_biome(end_gen: POINTER(EndGen), x: c_int32, y: c_int32, z: c_int32) -> EndBiomes:
    return _native.lib.get_biome(end_gen, x, y, z)

def delete(end_gen: POINTER(EndGen)):
    return _native.lib.delete(end_gen)
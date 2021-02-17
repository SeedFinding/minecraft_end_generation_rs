# python3 setup.py install --user

from minecraft_end_gen_rs import EndGen,create_new_end,get_biome,get_biome_2d,EndBiomes,delete
from ctypes import *

end_gen:POINTER(EndGen)=create_new_end(1551515151585454)
assert get_biome(end_gen,10000,251,10000)==EndBiomes.SmallEndIslands
print("That's a win again")
delete(end_gen)
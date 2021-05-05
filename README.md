# Minecraft End Generation in Rust

Warning by default you need rust 1.47.0+ since I use const fn but if you want to use another version just pass 
`--no-default-features` on your build/test/run command or as a dependency do `default-features = false`.
The MSRV is 1.40.0 if you disable default-features because of the use non exhaustive memchr #45394.

You can run this in C, C++, Rust and Python.

First thing: Get rust: https://rustup.rs

Second thing: Run Cargo: `cargo build --release`

Third thing: Install python bindings (optional): `python3 setup.py install --user`
You can then use it in python as usual:
```python
from minecraft_end_gen_rs import EndGen,create_new_end,get_biome,get_biome_2d,EndBiomes
from ctypes import *
end_gen:POINTER(EndGen)=create_new_end(1551515151585454)
assert get_biome(end_gen,10000,251,10000)==EndBiomes.SmallEndIslands
```

Fourth thing: Use Rust:
```rust
let mut gen: EndGen = EndGen::new(seed);
let biome=gen.get_final_biome(x, 251, z);
```

Fifth thing: Use C/C++: You have a shared library (.so/.dll) in target/release and a header file in target, you have three functions, just use them like any other functions ;) see example.c
```c
struct EndGen *create_new_end(uint64_t seed);

EndBiomes get_biome_2d(struct EndGen *end_gen, int32_t x, int32_t z);

EndBiomes get_biome(struct EndGen *end_gen, int32_t x, int32_t y, int32_t z);
```

Six thing (you can also run that in go, ruby and much more, rust/cbindings are awesome !)



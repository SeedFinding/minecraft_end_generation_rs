use minecraft_end_generation::EndGen;

fn main() {
    let seed: u64 = 1551515151585454;
    let offset_x: i32 = 10000;
    let offset_z: i32 = 10000;
    let mut gen: EndGen = EndGen::new(seed);
    let mut som:i32 =0;
    for x in 0..100 {
        for z in 0..100 {
            som=som.wrapping_add((gen.get_final_biome_2d(offset_x + x, offset_z + z) as i32));
        }
    }
    println!("{}",som);
}
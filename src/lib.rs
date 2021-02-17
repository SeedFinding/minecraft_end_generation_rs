#![allow(dead_code)]

use core::fmt;

use intmap::IntMap;
use java_random::{LCG, Random};
use noise_rs::math;
use noise_rs::simplex_noise::SimplexNoise;
use noise_rs::voronoi::Voronoi;

pub const END_LCG: LCG = LCG::combine_java(17292);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EndBiomes {
    Default = 0,
    TheEnd = 9,
    SmallEndIslands = 40,
    EndMidlands = 41,
    EndHighlands = 42,
    EndBarrens = 43,
}

impl fmt::Display for EndBiomes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen1() {
        let seed: u64 = 1551515151585454u64;
        let x: i32 = 10000;
        let z: i32 = 10000;
        let mut gen: EndGen = EndGen::new(seed);
        assert_eq!(gen.get_final_biome(x, 251, z).to_string(), "SmallEndIslands");
    }

    #[test]
    fn gen_column() {
        let seed: u64 = 1551515151585454u64;
        let x: i32 = 10000;
        let z: i32 = 10000;
        let mut gen: EndGen = EndGen::new(seed);
        let mut sum: i32 = 0;
        for y in 0..256 {
            let biome: EndBiomes = gen.get_final_biome(x, y, z);
            sum = sum.wrapping_add(biome as i32);
            println!("{} {}", y, biome.to_string());
        }
        assert_eq!(sum, 10689);
    }


    #[test]
    fn gen1million() {
        let seed: u64 = 1551515151585454;
        let offset_x: i32 = 10000;
        let offset_z: i32 = 10000;
        let mut gen: EndGen = EndGen::new(seed);
        let mut som: i32 = 0;
        for x in 0..1000 {
            for z in 0..1000 {
                som = som.wrapping_add(gen.get_final_biome_2d(offset_x + x, offset_z + z) as i32);
            }
        }
        assert_eq!(som, 41033489);
    }
}

/// <div rustbindgen hide></div>
#[derive(Clone)]
struct Noise {
    noise: SimplexNoise,
    voronoi: Voronoi,
    cache: IntMap<EndBiomes>,
}

#[repr(C)]
#[derive(Clone)]
pub struct EndGen {
    seed: u64,
    _noise: Box<Noise>,
}

#[no_mangle]
pub extern "C" fn create_new_end(seed: u64) -> Box<EndGen> {
    Box::new(EndGen::new(seed))
}

#[no_mangle]
pub unsafe extern "C" fn delete(end_gen: &mut EndGen) -> () {
    std::mem::drop(Box::from_raw(end_gen));
}

#[no_mangle]
pub extern "C" fn get_biome_2d(end_gen: &mut EndGen, x: i32, z: i32) -> EndBiomes {
    end_gen.get_final_biome_2d(x, z)
}

#[no_mangle]
pub unsafe extern "C" fn get_biome(end_gen: &mut EndGen, x: i32, y: i32, z: i32) -> EndBiomes {
    end_gen.get_final_biome(x, y, z)
}


impl EndGen {
    pub fn new(seed: u64) -> Self {
        let voronoi: Voronoi = Voronoi::new(math::sha2long(seed) as i64);
        let mut r: Random = Random::with_raw_seed_and_lcg(Random::default_scramble(seed), END_LCG);
        let seed: u64 = r.next_state().get_raw_seed();
        let noise: SimplexNoise = SimplexNoise::init(Random::with_raw_seed(seed));
        let cache: IntMap<EndBiomes> = IntMap::with_capacity(1024);
        let noise = Noise {
            noise,
            voronoi,
            cache,
        };
        EndGen { seed, _noise: Box::new(noise) }
    }
    pub fn get_final_biome_2d(&mut self, x: i32, z: i32) -> EndBiomes {
        let (xx, _, zz): (i32, i32, i32) = self._noise.voronoi.get_fuzzy_positions(x, 0, z);
        return self.get_biome(xx >> 2, zz >> 2);
    }
    pub fn get_final_biome(&mut self, x: i32, y: i32, z: i32) -> EndBiomes {
        let (xx, _, zz): (i32, i32, i32) = self._noise.voronoi.get_fuzzy_positions(x, y, z);
        return self.get_biome(xx >> 2, zz >> 2);
    }
    pub fn get_biome(&mut self, chunk_x: i32, chunk_z: i32) -> EndBiomes {
        let key: u64 = (((chunk_x as u32) as u64) << 32 | ((chunk_z as u32) as u64)) as u64;
        let value: EndBiomes = *self._noise.cache.get(key).unwrap_or(&EndBiomes::Default);
        if value != EndBiomes::Default {
            return value;
        }
        let value: EndBiomes = self._get_biome(chunk_x, chunk_z);
        self._noise.cache.insert(key, value);
        return value;
    }
    fn _get_biome(&mut self, chunk_x: i32, chunk_z: i32) -> EndBiomes {
        if chunk_x as i64 * chunk_x as i64 + chunk_z as i64 * chunk_z as i64 <= 4096i64 {
            return EndBiomes::TheEnd;
        }
        let height: f32 = self.get_height(chunk_x * 2 + 1, chunk_z * 2 + 1);
        if height > 40.0f32 {
            return EndBiomes::EndHighlands;
        }
        if height >= 0.0f32 {
            return EndBiomes::EndMidlands;
        }
        if height < -20.0f32 {
            return EndBiomes::SmallEndIslands;
        }
        return EndBiomes::EndBarrens;
    }
    fn get_height(&mut self, x: i32, z: i32) -> f32 {
        let scaled_x: i32 = x / 2;
        let scaled_z: i32 = z / 2;
        let odd_x: i32 = x % 2;
        let odd_z: i32 = z % 2;
        let mut height: f32 = math::clamp(100.0f32 - math::sqrt((x * x + z * z) as f32) * 8.0f32, -100.0f32, 80.0f32);
        for rx in -12..=12 {
            for rz in -12..=12 {
                let shifted_x: i64 = (scaled_x + rx) as i64;
                let shifted_z: i64 = (scaled_z + rz) as i64;
                if shifted_x * shifted_x + shifted_z * shifted_z <= 4096i64 || !(self._noise.noise.get_value_2d(shifted_x as f64, shifted_z as f64) < -0.8999999761581421) {
                    continue;
                }
                let elevation: f32 = (math::abs(shifted_x as f32) * 3439.0f32 + math::abs(shifted_z as f32) * 147.0f32) % 13.0f32 + 9.0f32;
                let smooth_x: f32 = (odd_x - rx * 2) as f32;
                let smooth_z: f32 = (odd_z - rz * 2) as f32;
                height = math::max(height, math::clamp(100.0f32 - math::sqrt(smooth_x * smooth_x + smooth_z * smooth_z) * elevation, -100.0f32, 80.0f32));
            }
        }
        return height;
    }
    pub fn set_seed(&mut self, seed: u64) {
        let mut random: Random = Random::with_seed_and_lcg(Random::default_scramble(seed), END_LCG);
        self.seed = random.next_state().get_raw_seed()
    }
}
mod math;

use crate::simplex_noise::SimplexNoise;
use crate::voronoi::get_fuzzy_positions;
use java_random::{Random, END_LCG};
mod voronoi;
mod simplex_noise;



#[derive(Copy, Clone)]
pub struct EndGen {
    seed: u64,
    x: i32,
    z: i32,
    noise: SimplexNoise,
}

pub enum EndBiomes {
    TheEnd,
    EndHighlands,
    EndMidlands,
    SmallEndIslands,
    EndBarrens,
}

impl EndGen {
    pub fn new(seed: u64, x: i32, z: i32) -> Self {
        let seed: u64 = Random::with_seed_and_lcg(seed, END_LCG).next_state().get_raw_seed();
        let noise: SimplexNoise = SimplexNoise::init(Random::with_raw_seed(seed));
        EndGen { seed, x, z, noise }
    }

    pub fn get_final_biome(&mut self, x: i32, z: i32) -> EndBiomes {
        let (xx, _, zz): (i32, i32, i32) = get_fuzzy_positions(self.seed as i64, x, 0, z);
        return self.get_biome(xx, zz);
    }
    pub fn get_biome(&mut self, x: i32, z: i32) -> EndBiomes {
        let chunk_x: i32 = x >> 2;
        let chunk_z: i32 = z >> 2;
        if chunk_x as i64 * chunk_x as i64 + chunk_z as i64 * chunk_z as i64 <= 4096i64 {
            return EndBiomes::TheEnd;
        }
        let height: f32 = Self::get_height(self.noise, chunk_x * 2 + 1, chunk_z * 2 + 1);
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
    pub fn get_height(noise: SimplexNoise, x: i32, z: i32) -> f32 {
        let scaled_x: i32 = x / 2;
        let scaled_z: i32 = z / 2;
        let odd_x: i32 = x % 2;
        let odd_z: i32 = z % 2;
        let mut height: f32 = math::clamp(100.0f32 - math::sqrt((x * x + z * z) as f32) * 8.0f32, -100.0f32, 80.0f32);
        for rx in -12..=12 {
            for rz in -12..=12 {
                let shifted_x: i64 = (scaled_x + rx) as i64;
                let shifted_z: i64 = (scaled_z + rz) as i64;
                if shifted_x * shifted_x + shifted_z * shifted_z <= 4096i64 || !(noise.get_value_2d(shifted_x as f64, shifted_z as f64) < -0.8999999761581421) {
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
        let mut random: Random = Random::with_seed_and_lcg(seed, END_LCG);
        self.seed = random.next_state().get_raw_seed()
    }

    pub fn set_position(&mut self, x: i32, z: i32) {
        self.x = x;
        self.z = z;
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
}
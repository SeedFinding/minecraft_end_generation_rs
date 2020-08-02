use crate::math;

pub fn next(world_seed: i64, salt: i64) -> i64 {
    return world_seed * (world_seed * 6364136223846793005i64 + 1442695040888963407i64) + salt;
}

pub fn get_fuzzy_positions(world_seed: i64, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
    let moved_x: i32 = x - 2;
    let moved_y: i32 = y - 2;
    let moved_z: i32 = z - 2;
    let reduced_x: i32 = moved_x >> 2;
    let reduced_y: i32 = moved_y >> 2;
    let reduced_z: i32 = moved_z >> 2;
    let x_scaled: f64 = (moved_x & 3) as f64 / 4.0f64;
    let y_scaled: f64 = (moved_y & 3) as f64 / 4.0f64;
    let z_scaled: f64 = (moved_z & 3) as f64 / 4.0f64;
    let mut arrd: [f64; 8] = [0.0f64; 8];
    for cell in 0usize..8 {
        let high4: bool = (cell & 4) == 0;
        let high2: bool = (cell & 2) == 0;
        let high1: bool = (cell & 1) == 0;
        let xx: i32 = if high4 { reduced_x } else { reduced_x + 1 };
        let yy: i32 = if high2 { reduced_y } else { reduced_y + 1 };
        let zz: i32 = if high1 { reduced_z } else { reduced_z + 1 };
        let xx_scaled: f64 = if high4 { x_scaled } else { x_scaled - 1.0 };
        let yy_scaled: f64 = if high2 { y_scaled } else { y_scaled - 1.0 };
        let zz_scaled: f64 = if high1 { z_scaled } else { z_scaled - 1.0 };
        arrd[cell] = get_fiddled_distance(world_seed, xx, yy, zz, xx_scaled, yy_scaled, zz_scaled);
    }
    let mut max_index: i32 = 0;
    let mut max: f64 = arrd[0];
    for cell in 1usize..8 {
        if !(max > arrd[cell]) { continue; }
        max_index = cell as i32;
        max = arrd[cell];
    }
    return (if (max_index & 4) == 0 { reduced_x } else { reduced_x + 1 }, if (max_index & 2) == 0 { reduced_y } else { reduced_y + 1 }, if (max_index & 1) == 0 { reduced_z } else { reduced_z + 1 });
}

pub fn get_fiddled_distance(l: i64, x: i32, y: i32, z: i32, x_scaled: f64, y_scaled: f64, z_scaled: f64) -> f64 {
    let mut l2: i64 = l;
    l2 = next(l2, x as i64);
    l2 = next(l2, y as i64);
    l2 = next(l2, z as i64);
    l2 = next(l2, x as i64);
    l2 = next(l2, y as i64);
    l2 = next(l2, z as i64);
    let d4: f64 = get_fiddle(l2);
    l2 = next(l2, l);
    let d5: f64 = get_fiddle(l2);
    l2 = next(l2, l);
    let d6: f64 = get_fiddle(l2);
    return sqr(z_scaled + d6) + sqr(y_scaled + d5) + sqr(x_scaled + d4);
}

pub fn get_fiddle(l: i64) -> f64 {
    return ((((math::floor_mod(l >> 24, 1024i64)) as i32) as f64 / 1024.0f64) - 0.5) * 0.9;
}

pub fn sqr(d: f64) -> f64 {
    d * d
}

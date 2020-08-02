pub fn abs(a: f32) -> f32 {
    if a <= 0.0f32 { -a } else { a }
}

pub fn clamp(f1: f32, f2: f32, f3: f32) -> f32 {
    if f1 < f2 {
        return f2;
    }
    if f1 > f3 {
        return f3;
    }
    return f1;
}

pub fn floor_mod(x: i64, y: i64) -> i64 {
    let mut modulo: i64 = x % y;
    // if the signs are different and modulo not zero, adjust result
    if (x ^ y) < 0 && modulo != 0 {
        modulo += y;
    }
    modulo
}

pub fn sqrt(f: f32) -> f32 {
    return f.sqrt();
}

pub fn max(a: f32, b: f32) -> f32 {
    if a >= b { a } else { b }
}


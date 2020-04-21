use rand::prelude::*;

pub const INFINITY: f32 = std::f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

pub fn float_min(a: f32, b: f32) -> f32 {
    if a <= b {
        a
    } else {
        b
    }
}

pub fn float_max(a: f32, b: f32) -> f32 {
    if a >= b {
        a
    } else {
        b
    }
}

pub fn random_float() -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen()
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

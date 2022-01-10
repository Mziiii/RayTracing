use rand::{thread_rng, Rng};

use super::Color;

pub const INF: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double(min: Option<f32>, max: Option<f32>) -> f32 {
    let mut rng = thread_rng();
    let n: f32 = rng.gen_range(min.unwrap_or(0.0), max.unwrap_or(1.0));
    n
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn random_f() -> f32 {
    thread_rng().gen_range(0.0, 1.0) //thread_rng().gen::<f32>()
}

pub fn random_u() -> u16 {
    thread_rng().gen::<u16>()
}

pub fn random_u_m(min: u16, max: u16) -> u16 {
    thread_rng().gen_range(min, max)
}

pub fn random_f_m(min: f32, max: f32) -> f32 {
    thread_rng().gen_range(min, max)
}

pub fn random_cosine_direction() -> Color {
    let r1 = random_f();
    let r2 = random_f();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (r2.sqrt());
    let y = phi.sin() * (r2.sqrt());

    Color::new(x, y, z)
}

pub fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        return a;
    }
    b
}

pub fn fmax(a: f32, b: f32) -> f32 {
    if a > b {
        return a;
    }
    b
}

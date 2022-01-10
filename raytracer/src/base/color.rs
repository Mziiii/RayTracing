use super::{rtweekend::clamp, vec3::Color};
pub fn write_color(pixel_color: &mut Color, samples_per_pixel: u16, pixel: &mut image::Rgb<u8>) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    if r != r {
        r = 0.0;
    }
    if g != g {
        g = 0.0;
    }
    if b != b {
        b = 0.0;
    }

    let scale = 1.0 / samples_per_pixel as f32;
    r = 256.0 * clamp((r * scale).sqrt(), 0.0, 0.999);
    g = 256.0 * clamp((g * scale).sqrt(), 0.0, 0.999);
    b = 256.0 * clamp((b * scale).sqrt(), 0.0, 0.999);

    *pixel = image::Rgb([r as u8, g as u8, b as u8]);
    // unsafe {
    //     format!(
    //         "{} {} {}\n",
    //         (255.999 * clamp(r, 0.0, 0.999)).to_int_unchecked::<u16>(),
    //         (255.999 * clamp(g, 0.0, 0.999)).to_int_unchecked::<u16>(),
    //         (255.999 * clamp(b, 0.0, 0.999)).to_int_unchecked::<u16>()
    //     )
    // }
}

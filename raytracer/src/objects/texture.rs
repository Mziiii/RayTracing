use std::{path::Path, sync::Arc};

use crate::{
    base::{
        rtweekend::*,
        vec3::{Color, Point3},
    },
    perlin::Perlin,
};
use image::{open, DynamicImage, GenericImageView};

pub trait Texture: Sync + Send {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

#[derive(Clone)]
pub struct SolidColor {
    pub color_value: Color,
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.color_value
    }
}

impl SolidColor {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color_value: Color {
                x: red,
                y: green,
                z: blue,
            },
        }
    }
    pub fn new_with_color(c: Color) -> Self {
        Self { color_value: c }
    }
}

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, U: Texture> {
    pub odd: T,
    pub even: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(o: T, e: U) -> Self {
        Self { odd: o, even: e }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let sines = ((10.0 * p.x).sin()) * ((10.0 * p.y).sin()) * ((10.0 * p.z).sin());
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        }
        self.even.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32,
}

impl NoiseTexture {
    pub fn new(s: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: s,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        Color::ones() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    pub image: DynamicImage,
    pub width: u16,
    pub height: u16,
}

impl ImageTexture {
    pub fn new(file_path: &str) -> ImageTexture {
        let _image = open(Path::new(file_path)).unwrap();
        ImageTexture {
            image: _image.clone(),
            width: _image.dimensions().0 as u16,
            height: _image.dimensions().1 as u16,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f32) as u16;
        let mut j = (v * self.height as f32) as u16;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i as u32, j as u32);

        Color {
            x: pixel[0] as f32 * color_scale,
            y: pixel[1] as f32 * color_scale,
            z: pixel[2] as f32 * color_scale,
        }
    }
}

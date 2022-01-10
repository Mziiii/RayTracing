#![allow(clippy::float_cmp)]
#![allow(unknown_lints)]
#![allow(clippy::module_inception)]
#![allow(clippy::eq_op)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::redundant_clone)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::clippy::redundant_clone)]
#![allow(clippy::new_without_default)]
#![allow(clippy::suspicious_operation_groupings)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_range_loop)]
#![allow(unused)]

pub mod base;
pub mod bvh;
pub mod hit;
pub mod objects;
pub mod pdf;
mod scene;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use image::GenericImage;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use threadpool::ThreadPool;

use crate::base::*;
use crate::hit::hittable_list::HittableList;
use crate::hit::*;
use crate::objects::arrect::XZRect;
use crate::objects::material::Empty;
use crate::objects::material::Lambertian;
use crate::objects::material::Material;
use crate::objects::sphere::Sphere;
use crate::objects::texture::SolidColor;
use crate::objects::*;
use crate::scene::*;

const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: f32 = 1.0;
const SAMPLES_PER_PIXEL: u16 = 20;
const MAX_DEPTH: u16 = 20;

fn main() {
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    let world;

    let mut background = Color::zero();

    let lookfrom;
    let lookat;
    let mut vfov = 40.0;
    let vup = Color::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let mut aperture = 0.0;
    match 0 {
        1 => {
            world = random_scene();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::zero();
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_checker_spheres();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::zero();
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::zero();
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::zero();
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            background = Color::zero();
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = cornell_box();
            background = Color::zero();
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            background = Color::new(0.70, 0.80, 1.00);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        8 => {
            world = final_scene();
            lookfrom = Point3::new(478.0, 278.0, -600.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0
        }
        _ => {
            world = pic();
            background = Color::new(0.90, 0.90, 0.97);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    }

    //cornell_box
    let mut lights = HittableList::new();
    lights.add(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Lambertian::new(SolidColor::new_with_color(Color::zero())),
    )));
    // lights.add(Arc::new(Sphere::new(
    //     &Point3::new(190.0, 90.0, 190.0),
    //     90.0,
    //     Lambertian::new(SolidColor::new_with_color(Color::zero())),
    // )));

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    //多线程
    let (tx, rx) = mpsc::channel();
    let num_threads: usize = 8;
    let num_jobs: u32 = 32;
    let thread_pool = ThreadPool::new(num_threads);

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let bar = ProgressBar::new(num_jobs as u64);

    for i in 0..num_jobs {
        let tx_ = tx.clone();
        let world_ptr = world.clone();
        let lights_ptr = lights.clone();
        let cam_ptr = cam.clone(); //when Camera doesn't implement Copy trait
        let start_height = IMAGE_HEIGHT * i / num_jobs;
        let finish_height = IMAGE_HEIGHT * (i + 1) / num_jobs;

        thread_pool.execute(move || {
            let delta_height = finish_height - start_height;
            let mut _img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, delta_height);
            for x in 0..IMAGE_WIDTH {
                for (img_y, y) in (start_height..finish_height).enumerate() {
                    let mut color = Color::zero();
                    for s in 0..SAMPLES_PER_PIXEL {
                        let u = (x as f32 + random_f()) / (IMAGE_WIDTH - 1) as f32;
                        let v = (IMAGE_HEIGHT as f32 - y as f32 + random_f())
                            / (IMAGE_HEIGHT - 1) as f32;
                        let r = cam_ptr.get_ray(u, v);
                        color += ray_color(
                            &r,
                            &background,
                            &world_ptr,
                            &Arc::new(lights_ptr.clone()),
                            MAX_DEPTH,
                        );
                    }
                    let pixel_color = _img.get_pixel_mut(x, img_y as u32);
                    write_color(&mut color, SAMPLES_PER_PIXEL, pixel_color);
                }
            }

            tx_.send((start_height..finish_height, _img))
                .expect("FAILED IN SENDING");
        });
    }

    for (rows, data) in rx.iter().take(num_jobs as usize) {
        for (idx, row) in rows.enumerate() {
            for col in 0..IMAGE_WIDTH {
                *img.get_pixel_mut(col, row) = *data.get_pixel(col, idx as u32);
            }
        }
        bar.inc(1);
    }

    img.save("output/pic.png").unwrap();
    bar.finish();
}

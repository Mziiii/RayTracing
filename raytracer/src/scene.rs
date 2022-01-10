use crate::base::{ray::*, rtweekend::*, vec3::*};
use crate::bvh::bvh::*;
use crate::hit::{hittable::*, hittable_list::*};
use crate::objects::{
    arrect::*, constant_medium::ConstantMedium, material::*, moving_sphere::*, sphere::*,
    texture::*,
};
use crate::pdf::cosine_pdf::{self, CosinePdf};
use crate::pdf::hittable_pdf::HittablePdf;
use crate::pdf::mixture_pdf::MixturePdf;
use crate::pdf::Pdf;
use std::clone;
use std::ops::Deref;
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = CheckerTexture::new(
        SolidColor::new_with_color(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new_with_color(Color::new(0.9, 0.9, 0.9)),
    );

    let ground_material = Lambertian { albedo: checker };
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f();
            let center = Color::new(
                a as f32 + 0.9 * random_f(),
                0.2,
                b as f32 + 0.9 * random_f(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Color = Color::elemul(Color::random_f(), Color::random_f());
                    let sphere_material = Lambertian::new(SolidColor::new_with_color(albedo));
                    let center2 = center + Color::new(0.0, random_f_m(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        &center,
                        &center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = random_f_m(0.0, 0.5);
                    let sphere_material = Metal { albedo, fuzz };
                    world.add(Arc::new(Sphere::new(&center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Dielectric { ir: 1.5 };
                    world.add(Arc::new(Sphere::new(&center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Dielectric { ir: 1.5 };
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Lambertian::new(SolidColor::new_with_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Arc::new(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

pub fn ray_color(
    ray: &Ray,
    background: &Color,
    world: &HittableList,
    lights: &Arc<HittableList>,
    depth: u16,
) -> Color {
    if depth == 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(ray, 0.001, INF) {
        let emitted = rec.mat_ptr.emitted(ray, &rec.clone(), rec.u, rec.v, &rec.p);
        if let Some(mut srec) = rec.mat_ptr.scatter(ray, &rec) {
            if srec.is_specular {
                return Vec3::elemul(
                    srec.attenuation,
                    ray_color(&srec.specular_ray, background, world, lights, depth - 1),
                );
            }

            let light_ptr = HittablePdf::new(lights.deref().clone(), rec.p);
            let p = MixturePdf::new(light_ptr, srec.pdf_ptr);
            let scattered = Ray::new(&rec.p, &p.generate(), ray.tm);
            let pdf_val = p.value(&scattered.dir);

            let multiply = ray_color(&scattered, background, world, lights, depth - 1)
                * rec.mat_ptr.scattering_pdf(ray, &rec.clone(), &scattered);
            let add = Color::elemul(srec.attenuation, multiply) / pdf_val;
            return emitted + add;
        }
        emitted
    } else {
        background.clone()
    }
}

pub fn two_checker_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = CheckerTexture::new(
        SolidColor::new_with_color(Color::new(0.2, 0.3, 0.1)),
        SolidColor::new_with_color(Color::new(0.9, 0.9, 0.9)),
    );

    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    )));

    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = NoiseTexture::new(4.0);

    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(pertext.clone()),
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(pertext.clone()),
    )));

    objects
}

pub fn earth() -> HittableList {
    let mut objects = HittableList::new();
    let earth_texture = ImageTexture::new("mars.jpg");
    let earth_surface = Lambertian::new(earth_texture);
    let globe = Arc::new(Sphere::new(&Point3::zero(), 2.0, earth_surface));

    objects.add(globe);
    objects
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = NoiseTexture::new(4.0);
    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(pertext.clone()),
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(pertext.clone()),
    )));

    let difflight = DiffuseLight::new(SolidColor::new_with_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Lambertian::new(SolidColor::new_with_color(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new_with_color(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new_with_color(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new_with_color(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    objects.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    )))));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let aluminum = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);
    let box1 = Box::new(Point3::zero(), Point3::new(165.0, 330.0, 165.0), aluminum);

    let box1 = RotateY::new(box1, 15.0);
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let glass = Dielectric::new(1.5);
    objects.add(Arc::new(Sphere::new(
        &Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    objects
}

pub fn pic() -> HittableList {
    let mut objects = HittableList::new();
    let glass = Dielectric::new(1.5);
    let white = Lambertian::new(SolidColor::new(0.82, 0.71, 0.84));
    let ground = Metal::new(Color::new(0.27, 0.04, 0.83), 1.0);
    // let background = Lambertian::new(ImageTexture::new("background.jpg"));

    // let background = Lambertian::new(ImageTexture::new("color4.jpg"));
    // let dark = Lambertian::new(SolidColor::new_with_color(Color::new(0.34, 0.33, 0.64)));
    // let purple = Lambertian::new(SolidColor::new_with_color(Color::new(0.54, 0.33, 0.63)));
    let pink = Lambertian::new(SolidColor::new_with_color(Color::new(0.99, 0.125, 0.47)));
    let sky_blue = Lambertian::new(SolidColor::new_with_color(Color::new(0.255, 0.41, 0.99)));
    let light_blue = Lambertian::new(SolidColor::new_with_color(Color::new(0.27, 0.04, 0.83)));
    let light = DiffuseLight::new(SolidColor::new_with_color(Color::new(15.0, 15.0, 15.0)));

    let sun = Lambertian::new(ImageTexture::new("sun.jpg"));
    // let mercury = Lambertian::new(ImageTexture::new("mercury.jpg"));
    let venus = Lambertian::new(ImageTexture::new("venus.jpg"));
    let earth = Lambertian::new(ImageTexture::new("earthmap.jpg"));
    let mars = Lambertian::new(ImageTexture::new("mars.jpg"));
    let jupiter = Lambertian::new(ImageTexture::new("jupiter.jpg"));
    let saturn = Lambertian::new(ImageTexture::new("saturn.jpg"));
    // let uranus = Lambertian::new(ImageTexture::new("uranus.jpg"));
    // let neptune = Lambertian::new(ImageTexture::new("neptune.jpg"));

    let back = Lambertian::new(ImageTexture::new("back.jpg"));
    let pink_ = Metal::new(Color::new(0.99, 0.125, 0.47), 0.5);
    let blue_ = Metal::new(Color::new(0.27, 0.04, 0.83), 0.5);
    objects.add(Arc::new(YZRect::new(
        //left
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        back.clone(),
    )));

    // objects.add(Arc::new(YZRect::new(
    //     //right
    //     0.0,
    //     555.0,
    //     0.0,
    //     555.0,
    //     0.0,
    //     blue_.clone(),
    // )));
    objects.add(Arc::new(YZRect::new(
        //right
        0.0,
        555.0,
        0.0,
        555.0,
        2.0,
        back.clone(),
    )));
    // let lamp = Sphere::new(&Point3::new(278.0, 610.0, 280.0), 80.0, light.clone());
    // objects.add(Arc::new(lamp));
    // let lamp = Sphere::new(&Point3::new(128.0, 590.0, 340.0), 50.0, light.clone());
    // objects.add(Arc::new(lamp));
    // let lamp = Sphere::new(&Point3::new(428.0, 590.0, 340.0), 50.0, light.clone());
    // objects.add(Arc::new(lamp));
    objects.add(Arc::new(XZRect::new(
        //lower
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        back.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        //upper
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        back.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        back.clone(),
    )));

    objects.add(Arc::new(Sphere::new(
        &Point3::new(320.0, 200.0, 320.0),
        133.3,
        sun,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(260.0, 200.0, 240.0),
        6.67,
        pink,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(200.0, 200.0, 320.0),
        16.0,
        venus,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(425.65, 200.0, 215.4),
        20.0,
        earth,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(320.0, 200.0, 160.0),
        12.5,
        mars,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(88.0, 200.0, 186.0),
        60.0,
        jupiter,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(560.0, 200.0, 500.0),
        53.0,
        saturn,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(572.0, 200.0, 68.0),
        33.3,
        sky_blue,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 200.0, 80.0),
        32.0,
        light_blue,
    )));

    let mut boxes = HittableList::new();
    let white = Lambertian::new(SolidColor::new_with_color(Color::new(0.99, 0.99, 0.99)));
    let radius = 100.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius + 320.0;
        let z = (i as f32 * unit_theta).sin() * radius + 320.0;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 200.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 120.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius + 320.0;
        let z = (i as f32 * unit_theta).sin() * radius + 320.0;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 200.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 148.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius + 320.0;
        let z = (i as f32 * unit_theta).sin() * radius + 320.0;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 200.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 160.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius + 320.0;
        let z = (i as f32 * unit_theta).sin() * radius + 320.0;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 200.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 268.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius + 320.0;
        let z = (i as f32 * unit_theta).sin() * radius + 320.0;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 200.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 300.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius + 320.0;
        let z = (i as f32 * unit_theta).sin() * radius + 320.0;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 200.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 360.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius + 320.0;
        let z = (i as f32 * unit_theta).sin() * radius + 320.0;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 200.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 400.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius + 320.0;
        let z = (i as f32 * unit_theta).sin() * radius + 320.0;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 200.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    // for i in 0..20 {
    //     let shadow = Sphere::new(
    //         &Point3::new(278.0, 590.0 - i as f32 * 30.0, 280.0),
    //         60.0 + i as f32 * 2.0,
    //         light.clone(),
    //     );
    //     // objects.add(Arc::new(ConstantMedium::new(
    //     //     shadow.clone(),
    //     //     0.0014,
    //     //     // DiffuseLight::new(SolidColor::new(1.0, 1.0, 1.0)),
    //     //     Metal::new(Color::new(0.54,0.83,0.92), 1.5),
    //     // )));
    //     objects.add(Arc::new(ConstantMedium::new(
    //         shadow.clone(),
    //         0.0013,
    //         DiffuseLight::new(SolidColor::new(1.0, 1.0, 1.0)),
    //         // Metal::new(Color::new(0.54,0.83,0.92), 0.5),
    //     )));
    // }

    // for i in 0..20 {
    //     let shadow = Sphere::new(
    //         &Point3::new(128.0 - i as f32 * 1.0, 590.0 - i as f32 * 30.0, 340.0),
    //         40.0 + i as f32 * 1.0,
    //         light.clone(),
    //     );
    //     objects.add(Arc::new(ConstantMedium::new(
    //         shadow,
    //         0.0013,
    //         DiffuseLight::new(SolidColor::new(1.0, 1.0, 1.0)),
    //         // Metal::new(Color::new(0.54,0.83,0.92), 0.5),
    //     )));
    // }

    // for i in 0..20 {
    //     let shadow = Sphere::new(
    //         &Point3::new(428.0 + i as f32 * 1.0, 590.0 - i as f32 * 30.0, 340.0),
    //         40.0 + i as f32 * 1.0,
    //         light.clone(),
    //     );
    //     objects.add(Arc::new(ConstantMedium::new(
    //         shadow,
    //         0.001,
    //         DiffuseLight::new(SolidColor::new(1.0, 1.0, 1.0)),
    //         // Metal::new(Color::new(0.54,0.83,0.92), 0.5),
    //     )));
    // }

    // let mut boxes1 = HittableList::new();
    // const BOXES_PER_SIDE: u16 = 10;
    // for i in 0..BOXES_PER_SIDE {
    //     for j in 0..BOXES_PER_SIDE {
    //         let w = 55.0;
    //         let y0 = 0.0 + i as f32 * w;
    //         let z0 = 0.0 + j as f32 * w;
    //         let x0 = 0.0;
    //         let y1 = y0 + w;
    //         let x1 = random_f_m(11.0, 51.0);
    //         let z1 = z0 + w;

    //         let box_: Arc<dyn Hittable> = Arc::new(Box::new(
    //             Point3::new(x0, y0, z0),
    //             Point3::new(x1, y1, z1),
    //             blue_.clone(),
    //         ));

    //         boxes1.add(box_);
    //     }
    // }
    // objects.add(Arc::new(boxes1));

    // let mut boxes1 = HittableList::new();
    // for i in 0..BOXES_PER_SIDE {
    //     for j in 0..BOXES_PER_SIDE {
    //         let w = 55.0;
    //         let y0 = 0.0 + i as f32 * w;
    //         let z0 = 0.0 + j as f32 * w;
    //         let x1 = 554.0;
    //         let y1 = y0 + w;
    //         let x0 = x1 - random_f_m(11.0, 51.0);
    //         let z1 = z0 + w;

    //         let box_: Arc<dyn Hittable> = Arc::new(Box::new(
    //             Point3::new(x0, y0, z0),
    //             Point3::new(x1, y1, z1),
    //             pink_.clone(),
    //         ));

    //         boxes1.add(box_);
    //     }
    // }
    // objects.add(Arc::new(boxes1));

    // objects.add(Arc::new(MovingSphere::new(
    //     &Point3::new(
    //         200.0 + 30.0,
    //         100.0 + random_f_m(-1.0, 1.0) * 50.0,
    //         200.0 + 30.0,
    //     ),
    //     &Point3::new(
    //         200.0 + 30.0,
    //         100.0 + random_f_m(-1.0, 1.0) * 20.0,
    //         200.0 + 30.0,
    //     ),
    //     0.0,
    //     1.0,
    //     60.0,
    //     // glass.clone(),
    //     Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    // )));
    // for i in 0..20 {
    //     for j in 0..20 {
    //         objects.add(Arc::new(MovingSphere::new(
    //             &Point3::new(
    //                 10.0 + 30.0 * i as f32,
    //                 90.0 + random_f_m(-1.0, 1.0),
    //                 10.0 + 30.0 * j as f32,
    //             ),
    //             &Point3::new(
    //                 10.0 + 30.0 * i as f32,
    //                 90.0 + random_f_m(-1.0, 1.0) * 20.0,
    //                 10.0 + 30.0 * j as f32,
    //             ),
    //             0.0,
    //             1.0,
    //             20.0,
    //             glass.clone(),
    //         )));
    //     }
    // }
    let mut ret_objects = HittableList::new();
    ret_objects.add(Arc::new(BvhNode::new_with_list(&mut objects, 0., 0.)));
    ret_objects
}

pub fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();
    let red = Lambertian::new(SolidColor::new_with_color(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColor::new_with_color(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColor::new_with_color(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColor::new_with_color(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Box::new(
        Point3::zero(),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));

    let box2 = Box::new(
        Point3::zero(),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));

    objects.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        Isotropic::new(SolidColor::new_with_color(Color::zero())),
    )));
    objects.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        Isotropic::new(SolidColor::new_with_color(Color::ones())),
    )));

    objects
}

pub fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new(SolidColor::new_with_color(Color::new(0.48, 0.83, 0.53)));

    const BOXES_PER_SIDE: u16 = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f_m(1.0, 101.0);
            let z1 = z0 + w;

            let box_: Arc<dyn Hittable> = Arc::new(Box::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));

            boxes1.add(box_);
        }
    }
    let mut objects = HittableList::new();

    objects.add(Arc::new(BvhNode::new_with_list(&mut boxes1, 0.0, 1.0)));

    let light = DiffuseLight::new(SolidColor::new_with_color(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        light.clone(),
    )));

    let cen0 = Point3::new(400.0, 400.0, 200.0);
    let cen1 = cen0.clone() + Color::new(30.0, 0.0, 0.0);
    let moving_sphere_material =
        Lambertian::new(SolidColor::new_with_color(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        &cen0,
        &cen1,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary = Sphere::new(
        &Point3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5),
    );
    objects.add(Arc::new(boundary.clone()));
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        Isotropic::new(SolidColor::new_with_color(Color::new(0.2, 0.4, 0.9))),
    )));
    let boundary = Sphere::new(&Point3::zero(), 5000.0, Dielectric::new(1.5));
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Isotropic::new(SolidColor::new_with_color(Color::ones())),
    )));

    let emat = Lambertian::new(ImageTexture::new("earthmap.jpg"));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = NoiseTexture::new(0.1);
    objects.add(Arc::new(Sphere::new(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(pertext),
    )));

    let mut boxes2 = HittableList::new();
    let white = Lambertian::new(SolidColor::new_with_color(Color::new(0.73, 0.73, 0.73)));
    const NS: usize = 1000;
    for j in 0..NS {
        let sphere: Arc<dyn Hittable> = Arc::new(Sphere::new(
            &Point3::random(0.0, 165.0),
            10.0,
            white.clone(),
        ));
        boxes2.add(sphere);
    }

    let p = BvhNode::new_with_list(&mut boxes2, 0.0, 1.0);
    let ptr = RotateY::new(p, 15.0);
    objects.add(Arc::new(Translate::new(
        ptr,
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut ret_objects = HittableList::new();
    ret_objects.add(Arc::new(BvhNode::new_with_list(&mut objects, 0., 1.)));
    ret_objects
}

pub fn solar_system() -> HittableList {
    let mut objects = HittableList::new();
    let background = Lambertian::new(ImageTexture::new("background.jpg"));
    let sun = Lambertian::new(ImageTexture::new("sun.jpg"));
    let mercury = Lambertian::new(ImageTexture::new("mercury.jpg"));
    let venus = Lambertian::new(ImageTexture::new("venus.jpg"));
    let earth = Lambertian::new(ImageTexture::new("earthmap.jpg"));
    let mars = Lambertian::new(ImageTexture::new("mars.jpg"));
    let jupiter = Lambertian::new(ImageTexture::new("jupiter.jpg"));
    let saturn = Lambertian::new(ImageTexture::new("saturn.jpg"));
    let uranus = Lambertian::new(ImageTexture::new("uranus.jpg"));
    let neptune = Lambertian::new(ImageTexture::new("neptune.jpg"));

    objects.add(Arc::new(XYRect::new(
        -600.0,
        600.0,
        -600.0,
        600.0,
        600.0,
        background.clone(),
    )));

    objects.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 200.0, 0.0),
        70.0,
        sun,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(-14.0, 200.0, 14.0),
        5.0,
        mercury,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(40.0, 200.0, 0.0),
        19.0,
        venus,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(42.0, 200.0, 42.0),
        20.0,
        earth,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(75.0, 200.0, 75.0),
        12.5,
        mars,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(-140.0, 200.0, -140.0),
        40.0,
        jupiter,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(42.0, 200.0, 42.0),
        33.0,
        saturn,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(315.0, 400.0, -315.0),
        25.0,
        uranus,
    )));
    objects.add(Arc::new(Sphere::new(
        &Point3::new(-500.0, 200.0, 0.0),
        24.0,
        neptune,
    )));

    let mut boxes = HittableList::new();
    let white = Lambertian::new(SolidColor::new_with_color(Color::new(0.99, 0.99, 0.99)));
    let radius = 20.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius;
        let z = (i as f32 * unit_theta).sin() * radius;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 400.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 40.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius;
        let z = (i as f32 * unit_theta).sin() * radius;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 400.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 60.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius;
        let z = (i as f32 * unit_theta).sin() * radius;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 400.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 75.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius;
        let z = (i as f32 * unit_theta).sin() * radius;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 400.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 200.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius;
        let z = (i as f32 * unit_theta).sin() * radius;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 400.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 300.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius;
        let z = (i as f32 * unit_theta).sin() * radius;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 400.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 450.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius;
        let z = (i as f32 * unit_theta).sin() * radius;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 400.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    let radius = 500.0;
    let num = radius as usize * 4;
    let unit_theta = 2.0 * PI / num as f32;
    for i in 0..num {
        let x = (i as f32 * unit_theta).cos() * radius;
        let z = (i as f32 * unit_theta).sin() * radius;
        let sphere: Arc<dyn Hittable> =
            Arc::new(Sphere::new(&Point3::new(x, 400.0, z), 1.0, white.clone()));
        objects.add(sphere);
    }

    // let p = BvhNode::new_with_list(&mut boxes, 0., 0.);
    // objects.add(Arc::new(p));

    // let mut boxes2 = HittableList::new();
    // let ring1 = Lambertian::new(ImageTexture::new("ring.jpeg"));
    // let ring2 = Lambertian::new(ImageTexture::new("saturn.jpg"));
    // const NS: usize = 10000;
    // for j in 0..NS {
    //     let sphere: Arc<dyn Hittable> = Arc::new(Sphere::new(
    //         &(Point3::new(0.0, 400.0, 600.0)
    //             + random_in_unit_disk().unit() * 110.0 * random_f_m(0.8, 1.)),
    //         1.0,
    //         ring1.clone(),
    //     ));
    //     objects.add(sphere);
    //     let sphere: Arc<dyn Hittable> = Arc::new(Sphere::new(
    //         &(Point3::new(0.0, 400.0, 600.0)
    //             + random_in_unit_disk().unit() * 100.0 * random_f_m(0.7, 1.)),
    //         1.0,
    //         ring2.clone(),
    //     ));
    //     objects.add(sphere);
    //     let sphere: Arc<dyn Hittable> = Arc::new(Sphere::new(
    //         &(Point3::new(0.0, 400.0, 600.0)
    //             + random_in_unit_disk().unit() * 90.0 * random_f_m(0.8, 1.)),
    //         1.0,
    //         ring1.clone(),
    //     ));
    //     objects.add(sphere);
    // }
    // objects.add(Arc::new(BvhNode::new_with_list(&mut boxes2, 0., 0.)));

    let mut ret_objects = HittableList::new();
    ret_objects.add(Arc::new(BvhNode::new_with_list(&mut objects, 0., 0.)));
    ret_objects
}

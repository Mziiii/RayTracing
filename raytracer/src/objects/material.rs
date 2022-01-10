use super::texture::*;
use crate::base::onb::Onb;
use crate::base::rtweekend::{random_cosine_direction, PI};
use crate::base::{ray::*, rtweekend::random_f, vec3::*};
use crate::hit::hittable::*;
use crate::pdf::cosine_pdf::CosinePdf;
use crate::pdf::Pdf;
use std::collections::hash_map::RandomState;
use std::sync::Arc;

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf_ptr: CosinePdf,
}

impl ScatterRecord {
    pub fn new() -> ScatterRecord {
        ScatterRecord {
            attenuation: Color::zero(),
            specular_ray: Ray::new_default(&Point3::zero(), &Vec3::zero()),
            pdf_ptr: CosinePdf::new(&Color::ones()),
            is_specular: false,
        }
    }
}

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        0.0
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f32, v: f32, p: &Point3) -> Color {
        Color::zero()
    }
}
pub struct Empty {}

impl Empty {
    pub fn new() -> Empty {
        Empty {}
    }
}

impl Material for Empty {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(a: T) -> Lambertian<T> {
        Lambertian { albedo: a }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some(ScatterRecord {
            attenuation,
            specular_ray: Ray::new_default(&Point3::zero(), &Vec3::zero()),
            pdf_ptr: CosinePdf::new(&rec.normal),
            is_specular: false,
        })
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = Color::dot(rec.normal, scattered.dir.unit());
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(a: Color, f: f32) -> Metal {
        Metal {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.dir.unit(), rec.normal) + self.fuzz * random_in_unit_sphere();

        let scattered = Ray {
            orig: rec.p,
            dir: reflected,
            tm: 0.0, //r_in.tm,
        };
        let attenuation = self.albedo;
        Some(ScatterRecord {
            attenuation,
            specular_ray: scattered,
            pdf_ptr: CosinePdf::new(&Color::ones()),
            is_specular: true,
        })
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r_square = r * r;
        r_square + (1.0 - r_square) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::ones();
        let refraction_ratio = {
            match rec.front_face {
                true => 1.0 / self.ir,
                false => self.ir,
            }
        };

        let unit_direction = r_in.dir.unit();
        let flag = Color::dot(-unit_direction, rec.normal) < 1.0;
        let cos_theta = {
            match flag {
                true => Color::dot(-unit_direction, rec.normal),
                false => 1.0,
            }
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = (refraction_ratio * sin_theta > 1.0)
            || (Dielectric::reflectance(cos_theta, refraction_ratio) > random_f());
        let direction = {
            match cannot_refract {
                true => reflect(unit_direction, rec.normal),
                false => refract(unit_direction, rec.normal, refraction_ratio),
            }
        };

        let scattered = Ray {
            orig: rec.p,
            dir: direction,
            tm: r_in.tm,
        };
        Some(ScatterRecord {
            attenuation,
            specular_ray: scattered,
            pdf_ptr: CosinePdf::new(&Color::ones()),
            is_specular: true,
        })
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f32, v: f32, p: &Point3) -> Color {
        if rec.front_face {
            return self.emit.value(u, v, p);
        }
        Color::zero()
    }
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(a: T) -> DiffuseLight<T> {
        DiffuseLight { emit: a }
    }
}

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(a: T) -> Isotropic<T> {
        Isotropic { albedo: a }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scattered = Ray::new(&rec.p, &random_in_unit_sphere(), r_in.tm);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some(ScatterRecord {
            attenuation,
            specular_ray: scattered,
            pdf_ptr: CosinePdf::new(&Color::ones()),
            is_specular: true,
        })
    }
}

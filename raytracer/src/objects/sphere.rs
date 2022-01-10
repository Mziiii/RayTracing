use super::material::Material;
use crate::base::onb::Onb;
use crate::base::rtweekend::random_f;
use crate::base::{ray::Ray, rtweekend::PI, vec3::*};
use crate::hit::hittable::*;
use crate::{base::rtweekend::INF, bvh::aabb::AABB};
use std::mem::ManuallyDrop;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere<T: Material> {
    pub center: Point3,
    pub radius: f32,
    pub mat_ptr: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(cen: &Point3, radius: f32, mat_ptr: T) -> Sphere<T> {
        Sphere {
            center: Point3::new(cen.x, cen.y, cen.z),
            radius,
            mat_ptr,
        }
    }

    fn get_sphere_uv(&self, p: &Point3) -> (f32, f32) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl<T: 'static + Clone + Material + Sync + Send> Hittable for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Color = ray.orig - self.center; //源心向量
        let a = ray.dir.squared_length();
        let b_half = Color::dot(oc, ray.dir);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;

        let mut rec = HitRecord {
            p: Point3::zero(),
            normal: Vec3::zero(),
            mat_ptr: Arc::new(self.mat_ptr.clone()),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };
        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();
            let root = (-b_half - sqrtd) / a;
            if root <= t_max && root >= t_min {
                rec.t = root;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                let (u, v) = self.get_sphere_uv(&outward_normal);
                rec.u = u;
                rec.v = v;
                rec.mat_ptr = Arc::new(self.mat_ptr.clone());
                return Some(rec);
            }
            let root = (-b_half + sqrtd) / a;
            if root <= t_max && root >= t_min {
                rec.t = root;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                let (u, v) = self.get_sphere_uv(&outward_normal);
                rec.u = u;
                rec.v = v;
                rec.mat_ptr = Arc::new(self.mat_ptr.clone());
                return Some(rec);
            }
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let outout_box = AABB::new(
            &(self.center - Color::new(self.radius, self.radius, self.radius)),
            &(self.center + Color::new(self.radius, self.radius, self.radius)),
        );

        Some(outout_box)
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f32 {
        if let Some(rec) = self.hit(&Ray::new_default(o, v), 0.001, INF) {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - *o).squared_length()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            return 1.0 / solid_angle;
        }
        0.0
    }

    fn random(&self, o: &Point3) -> Vec3 {
        let direction = self.center - *o;
        let squared_distance = direction.squared_length();
        let uvw = Onb::build_from_w(&direction);
        uvw.local(&random_to_sphere(self.radius, squared_distance))
    }
}

fn random_to_sphere(radius: f32, squared_distance: f32) -> Vec3 {
    let r1 = random_f();
    let r2 = random_f();
    let z = 1.0 + r2 * ((1.0 - radius * radius / squared_distance).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * ((1.0 - z * z).sqrt());
    let y = phi.sin() * ((1.0 - z * z).sqrt());

    Vec3::new(x, y, z)
}

use super::material::Material;
use crate::base::{ray::Ray, vec3::*};
use crate::bvh::aabb::*;
use crate::hit::hittable::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct MovingSphere<T: Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub mat_ptr: T,
}

impl<T: Material> MovingSphere<T> {
    pub fn new(
        cen0: &Point3,
        cen1: &Point3,
        time0: f32,
        time1: f32,
        radius: f32,
        mat_ptr: T,
    ) -> Self {
        Self {
            center0: Point3::new(cen0.x, cen0.y, cen0.z),
            center1: Point3::new(cen1.x, cen1.y, cen1.z),
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0)
    }
}

impl<T: 'static + Clone + Material + Sync + Send> Hittable for MovingSphere<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.orig - self.center(ray.tm);
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
                let outward_normal = (rec.p - self.center(ray.tm)) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                rec.mat_ptr = Arc::new(self.mat_ptr.clone());
                return Some(rec);
            }
            let root = (-b_half + sqrtd) / a;
            if root <= t_max && root >= t_min {
                rec.t = root;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center(ray.tm)) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                rec.mat_ptr = Arc::new(self.mat_ptr.clone());
                return Some(rec);
            }
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            &(self.center(time0) - Color::new(self.radius, self.radius, self.radius)),
            &(self.center(time0) + Color::new(self.radius, self.radius, self.radius)),
        );
        let box1 = AABB::new(
            &(self.center(time1) - Color::new(self.radius, self.radius, self.radius)),
            &(self.center(time1) + Color::new(self.radius, self.radius, self.radius)),
        );
        Some(surrounding_box(&box0, &box1))
    }
}

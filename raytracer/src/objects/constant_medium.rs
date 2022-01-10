use super::{material::*, texture::*};
use crate::base::{ray::Ray, rtweekend::*, vec3::*};
use crate::bvh::aabb::AABB;
use crate::hit::hittable::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConstantMedium<T: Hittable, U: Material> {
    pub boundary: T,
    pub phase_function: U,
    pub neg_inv_density: f32,
}

impl<T: Hittable, U: Material> ConstantMedium<T, U> {
    pub fn new(boundary: T, d: f32, a: U) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / d,
            phase_function: a,
        }
    }
}

impl<T: Clone + Hittable, U: 'static + Clone + Material + Sync + Send> Hittable
    for ConstantMedium<T, U>
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, -INF, INF) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, INF) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }

                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = ray.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_f().ln();
                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let rec = HitRecord {
                    p: ray.at(rec1.t + hit_distance / ray_length),
                    normal: Color::new(1.0, 0.0, 0.0),
                    mat_ptr: Arc::new(self.phase_function.clone()),
                    t: rec1.t + hit_distance / ray_length,
                    u: 0.0,
                    v: 0.0,
                    front_face: true,
                };

                return Some(rec);
            }
        }

        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if let Some(a) = self.boundary.bounding_box(time0, time1) {
            let _a = a.clone();
            return Some(_a);
        }
        None
    }
}

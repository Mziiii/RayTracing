use std::mem::swap;

use crate::base::{ray::Ray, rtweekend::*, vec3::Point3};

#[derive(Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(a: &Point3, b: &Point3) -> AABB {
        AABB {
            minimum: Point3::new(a.x, a.y, a.z),
            maximum: Point3::new(b.x, b.y, b.z),
        }
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, ray: &Ray, t_mini: f32, t_maxi: f32) -> bool {
        let mut t_min = t_mini;
        let mut t_max = t_maxi;
        for i in 0..3 {
            let inv_d = 1.0 / ray.dir[i];
            let mut t0 = (self.minimum[i] - ray.orig[i]) * inv_d;
            let mut t1 = (self.maximum[i] - ray.orig[i]) * inv_d;
            if inv_d.is_sign_negative() {
                swap(&mut t0, &mut t1);
            }
            t_min = fmax(t0, t_min);
            t_max = fmin(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    AABB {
        minimum: Point3 {
            x: fmin(box0.minimum.x, box1.minimum.x),
            y: fmin(box0.minimum.y, box1.minimum.y),
            z: fmin(box0.minimum.z, box1.minimum.z),
        },
        maximum: Point3 {
            x: fmax(box0.maximum.x, box1.maximum.x),
            y: fmax(box0.maximum.y, box1.maximum.y),
            z: fmax(box0.maximum.z, box1.maximum.z),
        },
    }
}

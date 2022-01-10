use crate::base::rtweekend::random_u_m;
use crate::base::{ray::*, vec3::Point3};
use crate::base::{Color, Vec3};
use crate::bvh::aabb::{surrounding_box, AABB};
use crate::hit::hittable::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        (*self).objects.truncate(0);
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        (*self).objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Option::Some(_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = _rec.t;
                temp_rec = Option::Some(_rec.clone());
            }
        }
        temp_rec
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let first_box = true;
        let mut output_box = AABB {
            //useless initialize but to assure syntax legal
            maximum: Point3::ones(),
            minimum: Point3::zero(),
        };
        for object in &self.objects {
            if let Some(_aabb) = (*object).bounding_box(time0, time1) {
                if first_box {
                    output_box = _aabb;
                } else {
                    output_box = surrounding_box(&output_box, &_aabb)
                }
            } else {
                return None;
            }
        }
        Some(output_box)
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f32 {
        let weight = 1.0 / self.objects.len() as f32;
        let mut sum = 0.0;
        for item in self.objects.iter() {
            sum += item.pdf_value(o, v) * weight;
        }
        sum
    }

    fn random(&self, o: &Point3) -> Vec3 {
        let index = random_u_m(0, self.objects.len() as u16);
        self.objects[index as usize].random(o)
    }
}

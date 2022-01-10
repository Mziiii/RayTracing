use crate::base::{ray::Ray, rtweekend::*, vec3::*};
use crate::bvh::aabb::AABB;
use crate::objects::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f32 {
        0.0
    }

    fn random(&self, o: &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(ray.dir, *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

#[derive(Clone)]
pub struct Translate<T: Hittable> {
    ptr: T,
    offset: Vec3,
}

impl<T: Clone + Hittable> Translate<T> {
    pub fn new(p: T, offset: Vec3) -> Self {
        Self {
            ptr: p.clone(),
            offset,
        }
    }
}

impl<T: Clone + Hittable> Hittable for Translate<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(&(ray.orig - self.offset), &ray.dir, ray.tm);
        if let Some(rec) = self.ptr.hit(&moved_ray, t_min, t_max) {
            let mut _rec = rec.clone();

            _rec.p += self.offset;
            _rec.set_face_normal(&moved_ray, &rec.normal);

            return Some(_rec);
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if let Some(output_box) = self.ptr.bounding_box(time0, time1) {
            return Some(AABB::new(
                &(output_box.minimum + self.offset),
                &(output_box.maximum + self.offset),
            ));
        }
        None
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f32 {
        self.ptr.pdf_value(&(*o - self.offset), v)
    }

    fn random(&self, o: &Point3) -> Vec3 {
        self.ptr.random(&(*o - self.offset))
    }
}

#[derive(Clone)]
pub struct RotateY<T: Hittable> {
    pub ptr: T,
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub hasbox: bool,
    pub bbox: AABB,
}

impl<T: Clone + Hittable> RotateY<T> {
    pub fn new(p: T, angle: f32) -> RotateY<T> {
        let radians = degrees_to_radians(angle);
        let mut min = Point3::new(INF, INF, INF);
        let mut max = Point3::new(-INF, -INF, -INF);
        let mut hasbox = false;
        let mut tmp_bbox = AABB::new(&Color::zero(), &Color::zero());
        if let Some(_bbox) = p.clone().bounding_box(0.0, 1.0) {
            hasbox = true;
            tmp_bbox = _bbox;
        }
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * tmp_bbox.maximum.x + (1.0 - i as f32) * tmp_bbox.minimum.x;
                    let y = j as f32 * tmp_bbox.maximum.y + (1.0 - j as f32) * tmp_bbox.minimum.y;
                    let z = k as f32 * tmp_bbox.maximum.z + (1.0 - k as f32) * tmp_bbox.minimum.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Color::new(new_x, y, new_z);

                    min = Point3::new(
                        fmin(min.x, tester.x),
                        fmin(min.y, tester.y),
                        fmin(min.z, tester.z),
                    );
                    max = Point3::new(
                        fmax(max.x, tester.x),
                        fmax(max.y, tester.y),
                        fmax(max.z, tester.z),
                    );
                }
            }
        }
        Self {
            ptr: p.clone(),
            sin_theta,
            cos_theta,
            hasbox,
            bbox: AABB::new(&min, &max),
        }
    }
}

impl<T: Clone + Hittable> Hittable for RotateY<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.orig.clone();
        let mut direction = ray.dir.clone();

        origin.x = self.cos_theta * ray.orig.x - self.sin_theta * ray.orig.z;
        origin.z = self.sin_theta * ray.orig.x + self.cos_theta * ray.orig.z;

        direction.x = self.cos_theta * ray.dir.x - self.sin_theta * ray.dir.z;
        direction.z = self.sin_theta * ray.dir.x + self.cos_theta * ray.dir.z;

        let rotated_ray = Ray::new(&origin, &direction, ray.tm);

        if let Some(mut rec) = self.ptr.hit(&rotated_ray, t_min, t_max) {
            let mut p = rec.p.clone();
            let mut normal = rec.normal.clone();

            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

            rec.p = p;
            rec.set_face_normal(&rotated_ray, &normal);
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.hasbox {
            return Some(self.bbox.clone());
        }
        None
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f32 {
        self.ptr.pdf_value(
            &Point3::new(
                self.cos_theta * o.x - self.sin_theta * o.z,
                o.y,
                self.sin_theta * o.x + self.cos_theta * o.z,
            ),
            &Vec3::new(
                self.cos_theta * v.x - self.sin_theta * v.z,
                v.y,
                self.sin_theta * v.x + self.cos_theta * v.z,
            ),
        )
    }

    fn random(&self, o: &Point3) -> Vec3 {
        let p = Point3::new(
            self.cos_theta * o.x - self.sin_theta * o.z,
            o.y,
            self.sin_theta * o.x + self.cos_theta * o.z,
        );
        let v = self.random(&p);
        Vec3::new(
            self.cos_theta * v.x + self.sin_theta * v.z,
            v.y,
            -self.sin_theta * v.x + self.cos_theta * v.z,
        )
    }
}

pub struct FlipFace {
    pub ptr: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(p: Arc<dyn Hittable>) -> FlipFace {
        FlipFace { ptr: p }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec) = self.ptr.hit(ray, t_min, t_max) {
            rec.front_face = !rec.front_face;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.ptr.bounding_box(time0, time1)
    }
}

use std::sync::Arc;

use crate::base::{
    ray::*,
    rtweekend::{random_f, random_f_m, INF},
    vec3::{Color, Point3},
    Vec3,
};
use crate::bvh::aabb::AABB;
use crate::hit::{hittable::*, hittable_list::HittableList};
use crate::objects::material::Material;

#[derive(Clone)]
pub struct XYRect<T: Material> {
    mp: T,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl<T: Material> XYRect<T> {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mp: T) -> XYRect<T> {
        XYRect {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl<T: 'static + Clone + Material + Sync + Send> Hittable for XYRect<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.orig.z) / ray.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.orig.x + t * ray.dir.x;
        let y = ray.orig.y + t * ray.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let mut rec = HitRecord {
            p: ray.at(t),
            normal: Vec3::zero(),
            mat_ptr: Arc::new(self.mp.clone()),
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            front_face: false,
        };
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(
            &Point3::new(self.x0, self.y0, self.k - 0.0001),
            &Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

#[derive(Clone)]
pub struct XZRect<T: Material> {
    mp: T,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl<T: Material> XZRect<T> {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, mp: T) -> XZRect<T> {
        XZRect {
            mp,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl<T: 'static + Clone + Material + Sync + Send> Hittable for XZRect<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.orig.y) / ray.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.orig.x + t * ray.dir.x;
        let z = ray.orig.z + t * ray.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let mut rec = HitRecord {
            p: ray.at(t),
            normal: Vec3::zero(),
            mat_ptr: Arc::new(self.mp.clone()),
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false,
        };
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(
            &Point3::new(self.x0, self.k - 0.0001, self.z0),
            &Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f32 {
        if let Some(rec) = self.hit(&Ray::new(o, v, 0.0), 0.001, INF) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let squared_distance = rec.t * rec.t * v.squared_length();
            let cosine = (Vec3::dot(v.clone(), rec.normal) / v.length()).abs();

            return squared_distance / (cosine * area);
        }
        0.0
    }

    fn random(&self, o: &Point3) -> Vec3 {
        let random_point = Point3::new(
            random_f_m(self.x0, self.x1),
            self.k,
            random_f_m(self.z0, self.z1),
        );
        random_point - o.clone()
    }
}

#[derive(Clone)]
pub struct YZRect<T: Material> {
    mp: T,
    z0: f32,
    z1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl<T: Material> YZRect<T> {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mp: T) -> YZRect<T> {
        YZRect {
            mp,
            z0,
            z1,
            y0,
            y1,
            k,
        }
    }
}

impl<T: 'static + Clone + Material + Sync + Send> Hittable for YZRect<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.orig.x) / ray.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let z = ray.orig.z + t * ray.dir.z;
        let y = ray.orig.y + t * ray.dir.y;
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let mut rec = HitRecord {
            p: ray.at(t),
            normal: Vec3::zero(),
            mat_ptr: Arc::new(self.mp.clone()),
            t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false,
        };
        rec.set_face_normal(ray, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(
            &Point3::new(self.k - 0.0001, self.y0, self.z0),
            &Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

#[derive(Clone)]
pub struct Box {
    pub box_min: Point3,
    pub box_max: Point3,
    pub slides: HittableList,
}

impl Box {
    pub fn new<T: 'static + Clone + Material + Sync + Send>(p0: Point3, p1: Point3, ptr: T) -> Box {
        let mut box0 = Box {
            box_min: p0,
            box_max: p1,
            slides: HittableList::new(),
        };
        box0.slides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        box0.slides.add(Arc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));
        box0.slides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        box0.slides.add(Arc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));
        box0.slides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        box0.slides.add(Arc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone(),
        )));

        box0
    }
}

impl Hittable for Box {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.slides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(&self.box_min, &self.box_max))
    }
}

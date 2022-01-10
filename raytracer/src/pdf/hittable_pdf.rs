use std::sync::Arc;

use crate::{
    base::{Color, Point3, Vec3},
    hit::{hittable::Hittable, hittable_list::HittableList},
};

use super::Pdf;

#[derive(Clone)]
pub struct HittablePdf<T: Hittable> {
    pub orig: Point3,
    pub ptr: T,
}

impl<T: Hittable> HittablePdf<T> {
    pub fn new(p: T, origin: Point3) -> HittablePdf<T> {
        HittablePdf {
            orig: origin,
            ptr: p,
        }
    }
}

impl<T: Hittable> Pdf for HittablePdf<T> {
    fn value(&self, direction: &Vec3) -> f32 {
        self.ptr.pdf_value(&self.orig, &direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.orig)
    }
}

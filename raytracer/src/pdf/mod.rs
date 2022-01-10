pub mod cosine_pdf;
pub mod hittable_pdf;
pub mod mixture_pdf;

use crate::base::Vec3;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f32;

    fn generate(&self) -> Vec3;
}

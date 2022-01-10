use std::sync::Arc;

use crate::base::{rtweekend::random_f, Vec3};

use super::Pdf;

#[derive(Clone)]
pub struct MixturePdf<T: Pdf, U: Pdf> {
    p0: T,
    p1: U,
}

impl<T: Pdf, U: Pdf> MixturePdf<T, U> {
    pub fn new(p0: T, p1: U) -> MixturePdf<T, U> {
        MixturePdf { p0, p1 }
    }
}

impl<T: Pdf, U: Pdf> Pdf for MixturePdf<T, U> {
    fn value(&self, direction: &Vec3) -> f32 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_f() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}

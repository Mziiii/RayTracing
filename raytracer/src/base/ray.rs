use super::vec3::Color;
use super::vec3::Point3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Color,
    pub tm: f32,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Color, time: f32) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
            tm: time,
        }
    }

    pub fn new_default(origin: &Point3, direction: &Color) -> Self {
        Self {
            orig: origin.clone(),
            dir: direction.clone(),
            tm: 0.0,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Color {
        self.dir
    }

    pub fn time(&self) -> f32 {
        self.tm
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}

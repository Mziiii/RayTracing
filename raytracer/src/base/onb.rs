use super::Color;

#[derive(Clone)]
pub struct Onb {
    pub axis: [Color; 3],
}

impl Default for Onb {
    fn default() -> Self {
        Self::new()
    }
}

impl Onb {
    pub fn new() -> Onb {
        Onb {
            axis: [Color::zero(); 3],
        }
    }

    pub fn u(&self) -> Color {
        self.axis[0]
    }

    pub fn v(&self) -> Color {
        self.axis[1]
    }

    pub fn w(&self) -> Color {
        self.axis[2]
    }

    pub fn local(&self, a: &Color) -> Color {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }

    pub fn local_single(&self, a: f32, b: f32, c: f32) -> Color {
        a * self.u() + b * self.v() + c * self.w()
    }

    pub fn build_from_w(n: &Color) -> Self {
        let mut uvw = Onb::new();
        uvw.axis[2] = n.unit();
        let flag = uvw.w().x.abs() > 0.9;
        let a = match flag {
            true => Color::new(0.0, 1.0, 0.0),
            false => Color::new(1.0, 0.0, 0.0),
        };
        uvw.axis[1] = Color::cross(uvw.w(), a).unit();
        uvw.axis[0] = Color::cross(uvw.w(), uvw.v());

        uvw
    }
}

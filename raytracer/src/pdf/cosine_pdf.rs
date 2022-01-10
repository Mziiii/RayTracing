use crate::base::{
    onb::Onb,
    rtweekend::{random_f, PI},
    Color, Vec3,
};

use super::Pdf;

#[derive(Clone)]
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Color) -> CosinePdf {
        let uvw = Onb::build_from_w(w);
        CosinePdf { uvw }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f32 {
        let cosine = Vec3::dot(direction.unit(), self.uvw.w());
        if cosine <= 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(&random_cosine_direction())
    }
}

fn random_cosine_direction() -> Color {
    let r1 = random_f();
    let r2 = random_f();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (r2.sqrt());
    let y = phi.sin() * (r2.sqrt());

    Color::new(x, y, z)
}

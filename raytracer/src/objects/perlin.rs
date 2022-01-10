use crate::{
    rtweekend::*,
    vec3::{Color, Point3},
};

#[derive(Clone)]
pub struct Perlin {
    ranvec: Vec<Color>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const POINT_COUNT: i32 = 256;

    pub fn new() -> Perlin {
        let mut random_vector: Vec<Color> = vec![];
        for i in 0..Perlin::POINT_COUNT {
            random_vector.push(Color::random(-1.0, 1.0));
        }
        Perlin {
            ranvec: random_vector,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (1..n).rev() {
            let target = random_u_m(0, i as u16);
            p.swap(i as usize, target as usize);
        }
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut rani: Vec<i32> = vec![];
        for i in 0..Perlin::POINT_COUNT {
            rani.push(i);
        }
        Perlin::permute(&mut rani, Perlin::POINT_COUNT);
        rani
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Color::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[(i as usize + di) & 255]
                        ^ self.perm_y[(j as usize + dj) & 255]
                        ^ self.perm_z[(k as usize + dk) & 255])
                        as usize];
                }
            }
        }
        self.perlin_interp(&c, u, v, w)
    }

    pub fn perlin_interp(&self, c: &[[[Color; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Color::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + ((1.0 - k as f32) * (1.0 - ww)))
                        * Color::dot(c[i][j][k], weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Point3, depth: u16) -> f32 {
        let mut accum: f32 = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

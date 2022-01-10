use super::ray::*;
use super::rtweekend::*;
use super::vec3::*;

#[derive(Clone, Copy)]
pub struct Camera {
    pub orig: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Color,
    pub vertical: Color,
    pub u: Color,
    pub v: Color,
    pub w: Color,
    pub lens_radius: f32,
    pub time0: f32,
    pub time1: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Color,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = Color::cross(vup, w).unit();
        let v = Color::cross(w, u);
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            orig: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn new_default(
        lookfrom: Point3,
        lookat: Point3,
        vup: Color,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = Color::cross(vup, w).unit();
        let v = Color::cross(w, u);
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            orig: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0: 0.0,
            time1: 0.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            orig: self.orig + offset,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.orig
                - offset,
            tm: random_f_m(self.time0, self.time1),
        }
    }
}

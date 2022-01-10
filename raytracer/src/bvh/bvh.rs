use super::aabb::*;
use crate::base::{ray::*, rtweekend::*, vec3::*};
use crate::hit::{hittable::*, hittable_list::*};
use crate::objects::texture::SolidColor;
use crate::objects::{material::Lambertian, sphere::Sphere};
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone)]
pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub boox: AABB,
}
impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.boox.hit(ray, t_min, t_max) {
            return None;
        }

        let mut opt = None;
        let mut t = t_max;

        if let Some(hitleft) = self.left.hit(ray, t_min, t_max) {
            t = hitleft.t;
            opt = Some(hitleft);
        }
        if let Some(hitright) = self.right.hit(ray, t_min, t) {
            opt = Some(hitright);
        }
        opt
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(self.boox.clone())
    }
}

impl BvhNode {
    pub fn new_with_list(list: &mut HittableList, time0: f32, time1: f32) -> BvhNode {
        let len = list.objects.len();
        BvhNode::new(&mut list.objects, 0, len, time0, time1)
    }

    pub fn new(
        src_objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f32,
        time1: f32,
    ) -> BvhNode {
        let mut bvh_node = BvhNode {
            left: Arc::new(Sphere::new(
                &Point3::zero(),
                0.0,
                Lambertian::new(SolidColor::new_with_color(Color::zero())),
            )),
            right: Arc::new(Sphere::new(
                &Point3::zero(),
                0.0,
                Lambertian::new(SolidColor::new_with_color(Color::zero())),
            )),
            boox: AABB::new(&Color::zero(), &Color::zero()),
        };

        let axis = random_u_m(0, 3);
        let object_span = end - start;
        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            2 => BvhNode::box_z_compare,
            _ => panic!("AXIS ERROR"),
        };
        match object_span {
            1 => {
                bvh_node.left = src_objects[start].clone();
                bvh_node.right = src_objects[start].clone();
            }
            2 => {
                if comparator(&src_objects[start], &src_objects[start + 1]) == Ordering::Less {
                    bvh_node.right = src_objects[start + 1].clone();
                    bvh_node.left = src_objects[start].clone();
                } else {
                    bvh_node.left = src_objects[start + 1].clone();
                    bvh_node.right = src_objects[start].clone();
                }
            }
            _ => {
                let objects = &mut src_objects[start..end];
                objects.sort_by(|a, b| comparator(a, b));

                let mid = (start + end) / 2;
                bvh_node.left = Arc::new(BvhNode::new(src_objects, start, mid, time0, time1));
                bvh_node.right = Arc::new(BvhNode::new(src_objects, mid, end, time0, time1));
            }
        }
        if let Some(box0) = bvh_node.left.bounding_box(time0, time1) {
            if let Some(box1) = bvh_node.right.bounding_box(time0, time1) {
                bvh_node.boox = surrounding_box(&box0, &box1);
                return bvh_node;
            }
        }

        panic!("NO BOUNDING BOX IN BVHNODE::NEW");
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: u16) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);

        match axis {
            0 => {
                if let Some(rec1) = box_a {
                    if let Some(rec2) = box_b {
                        if let Some(cmp) = rec1.minimum.x.partial_cmp(&rec2.minimum.x) {
                            return cmp;
                        }
                    }
                }
            }
            1 => {
                if let Some(rec1) = box_a {
                    if let Some(rec2) = box_b {
                        if let Some(cmp) = rec1.minimum.y.partial_cmp(&rec2.minimum.y) {
                            return cmp;
                        }
                    }
                }
            }
            2 => {
                if let Some(rec1) = box_a {
                    if let Some(rec2) = box_b {
                        if let Some(cmp) = rec1.minimum.z.partial_cmp(&rec2.minimum.z) {
                            return cmp;
                        }
                    }
                }
            }
            _ => {}
        }

        panic!("No bounding box in bvh_node constructor.")
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 2)
    }
}

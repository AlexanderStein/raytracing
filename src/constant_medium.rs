use crate::{hitable::*, material::*, texture::*};
use cgmath::{InnerSpace, Vector3};
use rand::prelude::*;

pub struct ConstantMedium<H: Hittable> {
    density: f64,
    boundary: H,
    phase_function: Box<dyn Material>,
}

impl<H: Hittable> ConstantMedium<H> {
    pub fn new<T: Texture + 'static>(boundary: H, density: f64, texture: T) -> Self {
        Self {
            density,
            boundary,
            phase_function: Box::new(Isotropic::new(Box::new(texture))),
        }
    }
}

impl<H: Hittable> Hittable for ConstantMedium<H> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = thread_rng();
        if let Some(mut rec1) = self.boundary.hit(ray, f64::MIN, f64::MAX) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, f64::MAX) {
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);
                if rec1.t < rec2.t {
                    rec1.t = rec1.t.max(0.0);
                    let ray_length = ray.direction().magnitude();
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                    let hit_distance =
                        -(1.0 / self.density) * (rng.gen_range(0.0..1.0) as f64).ln();
                    if hit_distance <= distance_inside_boundary {
                        let t = rec1.t + hit_distance / ray_length;
                        return Some(HitRecord {
                            p: ray.at(t),
                            normal: Vector3::new(1.0, 0.0, 0.0),
                            material: self.phase_function.as_ref(),
                            t,
                            u: 0.0,
                            v: 0.0,
                            front_face: true,
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}

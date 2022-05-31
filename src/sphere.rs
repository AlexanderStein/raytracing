use crate::{hitable::*, material::MaterialTrait};
use cgmath::*;
use std::option::Option;

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material: Box<dyn MaterialTrait>,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Box<dyn MaterialTrait>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().magnitude2();
        let half_b = oc.dot(ray.direction());
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut record = HitRecord {
            p,
            normal: outward_normal,
            material: self.material.as_ref(),
            t: root,
            front_face: false,
        };
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }
}

use crate::{hitable::*, vec3::*, material::MaterialTrait};
use std::option::Option;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn MaterialTrait>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn MaterialTrait>) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut record = HitRecord {
            p: ray.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: self.material,
            t: root,
            front_face: false,
        };
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.material = self.material;
        Some(record)
    }
}

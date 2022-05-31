use crate::{hitable::*, material::Material};
use cgmath::*;
use std::option::Option;

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Box<dyn Material>) -> Self {
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

pub struct MovingSphere {
    center0: Point3<f64>,
    center1: Point3<f64>,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(center0: Point3<f64>, center1: Point3<f64>, time0: f64, time1: f64, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3<f64> {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
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
        let outward_normal = (p - self.center(ray.time())) / self.radius;
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

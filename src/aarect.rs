use crate::{aabb::AABB, hitable::*, material::Material};
use cgmath::{Point3, Vector3};

pub struct XYRect {
    pub material: Box<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl Hittable for XYRect {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x + t * ray.direction().x;
        let y = ray.origin().y + t * ray.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vector3::new(0.0, 0.0, 1.0);
        let mut record = HitRecord {
            p: ray.at(t),
            normal: outward_normal,
            material: self.material.as_ref(),
            t: t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            front_face: false,
        };
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        let minimum = Point3::new(self.x0, self.y0, self.k - 0.0001);
        let maximum = Point3::new(self.x1, self.y1, self.k + 0.0001);
        Some(AABB::new(minimum, maximum))
    }
}

pub struct XZRect {
    pub material: Box<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl Hittable for XZRect {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().y) / ray.direction().y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vector3::new(0.0, 1.0, 0.0);
        let mut record = HitRecord {
            p: ray.at(t),
            normal: outward_normal,
            material: self.material.as_ref(),
            t: t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false,
        };
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        let minimum = Point3::new(self.x0, self.k - 0.0001, self.z0);
        let maximum = Point3::new(self.x1, self.k + 0.0001, self.z1);
        Some(AABB::new(minimum, maximum))
    }
}

pub struct YZRect {
    pub material: Box<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl Hittable for YZRect {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().x) / ray.direction().x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin().y + t * ray.direction().y;
        let z = ray.origin().z + t * ray.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1{
            return None;
        }

        let outward_normal = Vector3::new(1.0, 0.0, 0.0);
        let mut record = HitRecord {
            p: ray.at(t),
            normal: outward_normal,
            material: self.material.as_ref(),
            t: t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false,
        };
        record.set_face_normal(ray, outward_normal);
        Some(record)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        let minimum = Point3::new(self.k - 0.0001, self.y0, self.z0);
        let maximum = Point3::new(self.k + 0.0001, self.y1, self.z1);
        Some(AABB::new(minimum, maximum))
    }
}

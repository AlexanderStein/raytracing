use crate::{aabb::Aabb, material::Material, ray::*};
use cgmath::{InnerSpace, Point3, Vector3};
use std::option::Option;

pub struct HitRecord<'a> {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

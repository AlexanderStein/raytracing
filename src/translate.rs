use crate::{aabb::AABB, hitable::Hittable, ray::Ray};
use cgmath::Vector3;

pub struct Translate<H: Hittable> {
    offset: Vector3<f64>,
    hitable: H,
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hitable::HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        self.hitable.hit(ray, t_min, t_max).map(|mut record| {
            record.p += self.offset;
            record.set_face_normal(&moved_ray, record.normal);
            record
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        self.hitable.bounding_box(time0, time1).map(|bbox| {
            AABB::new(bbox.min(), bbox.max())
        })
    }
}

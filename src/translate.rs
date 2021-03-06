use crate::{aabb::Aabb, hitable::*, ray::Ray};
use cgmath::Vector3;

pub struct Translate<H: Hittable> {
    offset: Vector3<f64>,
    hitable: H,
}

impl<H: Hittable> Translate<H> {
    pub fn new(offset: Vector3<f64>, hitable: H) -> Self {
        Self { offset, hitable }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        self.hitable.hit(&moved_ray, t_min, t_max).map(|mut record| {
            record.p += self.offset;
            record.set_face_normal(&moved_ray, record.normal);
            record
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.hitable
            .bounding_box(time0, time1)
            .map(|bbox| Aabb::new(bbox.min(), bbox.max()))
    }
}

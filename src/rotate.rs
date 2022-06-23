use crate::{aabb::Aabb, hitable::*, ray::Ray};
use cgmath::Point3;

pub struct RotateY<H: Hittable> {
    hitable: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(hitable: H, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hitable.bounding_box(0.0, 1.0).map(|mut bbox| {
            let mut min = Point3 {
                x: f64::MAX,
                y: f64::MAX,
                z: f64::MAX,
            };
            let mut max = Point3 {
                x: f64::MIN,
                y: f64::MIN,
                z: f64::MIN,
            };
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bbox.max().x + (1 - i) as f64 * bbox.min().x;
                        let y = j as f64 * bbox.max().y + (1 - j) as f64 * bbox.min().y;
                        let z = k as f64 * bbox.max().z + (1 - k) as f64 * bbox.min().z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        min.x = f64::min(min.x, new_x);
                        min.y = f64::min(min.y, y);
                        min.z = f64::min(min.z, new_z);

                        max.x = f64::max(max.x, new_x);
                        max.y = f64::max(max.y, y);
                        max.z = f64::max(max.z, new_z);
                    }
                }
            }
            bbox.set_min(min);
            bbox.set_max(max);
            bbox
        });

        Self {
            hitable,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin.x = self.cos_theta * ray.origin().x - self.sin_theta * ray.origin().z;
        origin.z = self.sin_theta * ray.origin().x + self.cos_theta * ray.origin().z;

        direction.x = self.cos_theta * ray.direction().x - self.sin_theta * ray.direction().z;
        direction.z = self.sin_theta * ray.direction().x + self.cos_theta * ray.direction().z;

        let rotated_ray = Ray::new(origin, direction, ray.time());
        self.hitable
            .hit(&rotated_ray, t_min, t_max)
            .map(|mut record| {
                let mut p = record.p;
                let mut normal = record.normal;

                p.x = self.cos_theta * record.p.x + self.sin_theta * record.p.z;
                p.z = -self.sin_theta * record.p.x + self.cos_theta * record.p.z;

                normal.x = self.cos_theta * record.normal.x + self.sin_theta * record.normal.z;
                normal.z = -self.sin_theta * record.normal.x + self.cos_theta * record.normal.z;

                record.p = p;
                record.set_face_normal(&rotated_ray, normal);
                record
            })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}

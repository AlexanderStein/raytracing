use crate::{
    hitable::{HitRecord, Hittable, HitableList},
    vec3::*,
    Color,
};
use rand::RngCore;

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &HitableList, rng: &mut dyn RngCore) -> Color {
        let mut record = HitRecord::new();
        if world.hit(self, 0.0, f64::MAX, &mut record) {
            let target = record.p + record.normal + Vec3::random_in_unit_sphere(rng);
            let ray = Ray::new(record.p, target - record.p);
            return 0.5 * ray.color(world, rng);
        }
        let unit_direction = &self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = self.origin() - *center;
        let a = self.direction().length_squared();
        let half_b = oc.dot(&self.direction());
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-half_b - discriminant.sqrt()) / a;
        }
    }
}

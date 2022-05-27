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

    pub fn color(&self, world: &HitableList, depth: usize, rng: &mut dyn RngCore) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0,0.0,0.0);
        }

        if let Some(record) = world.hit(self, 0.001, f64::MAX) {
            let target = record.p + record.normal + Vec3::random_in_hemisphere(&record.normal, rng);
            let ray = Ray::new(record.p, target - record.p);
            return 0.5 * ray.color(world, depth - 1, rng);
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

use crate::{
    hitable::{HitableList, Hittable},
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
            return Color::zero();
        }

        if let Some(record) = world.hit(self, 0.001, f64::MAX) {
            if let Some((attenuation, scattered)) = record.material.scatter(self, &record, rng) {
                return attenuation * scattered.color(world, depth - 1, rng);
            }
            return Color::zero();
        }
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

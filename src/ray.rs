use crate::{
    color::{self, Color},
    hitable::Hittable,
    hitable_list::HitableList,
};
use cgmath::{InnerSpace, Point3, Vector3};
use rand::RngCore;

pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3<f64> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &HitableList, depth: usize, rng: &mut dyn RngCore) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return color::black();
        }

        if let Some(record) = world.hit(self, 0.001, f64::MAX) {
            if let Some((attenuation, scattered)) = record.material.scatter(self, &record, rng) {
                return attenuation.zip(scattered.color(world, depth - 1, rng), |l, r| l * r);
            }
            return color::black();
        }
        let unit_direction = self.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

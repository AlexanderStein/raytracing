use crate::{
    color::{self, Color},
    hitable::Hittable,
    hitable_list::HitableList,
};
use cgmath::{Point3, Vector3};
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

    pub fn color(
        &self,
        background: &Color,
        world: &HitableList,
        depth: usize,
        rng: &mut dyn RngCore,
    ) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return color::black();
        }

        // If the ray hits nothing, return the background color
        match world.hit(self, 0.001, f64::MAX) {
            None => *background,
            Some(record) => {
                let emitted = record.material.emitted(record.u, record.v, &record.p);
                match record.material.scatter(self, &record, rng) {
                    None => emitted,
                    Some((attenuation, scattered)) => attenuation.zip(
                        scattered.color(background, world, depth - 1, rng),
                        |l, r| l * r,
                    ),
                }
            }
        }
    }
}

use crate::{hitable::HitRecord, vec3::*, color::Color, ray::Ray};
use rand::RngCore;

#[derive(Clone)]
pub struct Material;

pub trait MaterialTrait {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray, rng: &mut dyn RngCore) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new (color: &Color) -> Self {
        Self {albedo: *color}
    }
}

impl MaterialTrait for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray, rng: &mut dyn RngCore) -> bool {
        let scatter_direction = record.normal + Vec3::random_unit_vector(rng);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

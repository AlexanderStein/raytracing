use crate::{color::Color, hitable::HitRecord, ray::Ray, vec3::*};
use rand::RngCore;

pub trait MaterialTrait {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: &Color) -> Self {
        Self { albedo: *color }
    }
}

impl MaterialTrait for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        let scatter_direction = record.normal + Vec3::random_unit_vector(rng);

        // Catch degenerate scatter direction
        let scatter_direction = if scatter_direction.near_zero() {
            record.normal
        } else {
            scatter_direction
        };

        let scattered = Ray::new(record.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: &Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo: *color,
            fuzz,
        }
    }
}

impl MaterialTrait for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().unit_vector().reflect(&record.normal);
        let scattered = Ray::new(record.p, reflected + self.fuzz * Vec3::random_in_unit_sphere(rng));
        if scattered.direction().dot(&record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

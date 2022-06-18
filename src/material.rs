use crate::{color::Color, hitable::HitRecord, ray::Ray, texture::*};
use cgmath::{InnerSpace, Vector3};
use rand::{Rng, RngCore};
use raytracer::{random_in_unit_sphere, random_unit_vector};

// Return true if the vector is close to zero in all dimensions.
fn near_zero(v: Vector3<f64>) -> bool {
    const EPSILON: f64 = 1e-8;
    v.x.abs() < EPSILON && v.y.abs() < EPSILON && v.z.abs() < EPSILON
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-uv).dot(n).min(1.0);

    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude2()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord, rng: &mut dyn RngCore)
        -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(color: &Color) -> Self {
        let albedo = Box::new(SolidColor::new(color));
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        let scatter_direction = record.normal + random_unit_vector(rng);

        // Catch degenerate scatter direction
        let scatter_direction = if near_zero(scatter_direction) {
            record.normal
        } else {
            scatter_direction
        };

        let scattered = Ray::new(record.p, scatter_direction, ray.time());
        Some((self.albedo.value(), scattered))
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

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        let reflected = reflect(ray.direction().normalize(), record.normal);
        let scattered = Ray::new(
            record.p,
            reflected + (self.fuzz * random_in_unit_sphere(rng)),
            ray.time(),
        );
        if scattered.direction().dot(record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, reflectance_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - reflectance_index) / (1.0 + reflectance_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        rng: &mut dyn RngCore,
    ) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            reflect(unit_direction, record.normal)
        } else {
            refract(unit_direction, record.normal, refraction_ratio)
        };
        let scattered = Ray::new(record.p, direction, ray.time());
        Some((attenuation, scattered))
    }
}

use crate::{hitable::HitRecord, color::Color, ray::Ray};

pub trait Material {
    fn scatter(ray_in: &Ray, record: &HitRecord, attenuation: &Color, scattered: &mut Ray) -> bool;
}

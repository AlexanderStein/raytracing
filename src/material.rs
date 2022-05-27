use crate::{hitable::HitRecord, color::Color, ray::Ray};

#[derive(Clone)]
pub struct Material;

pub trait MaterialTrait {
    fn scatter(ray_in: &Ray, record: &HitRecord, attenuation: &Color, scattered: &mut Ray) -> bool;
}

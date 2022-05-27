use crate::{hitable::HitRecord, color::Color, ray::Ray};
use rand::RngCore;

#[derive(Clone)]
pub struct Material;

pub trait MaterialTrait {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray, rng: &mut dyn RngCore) -> bool;
}

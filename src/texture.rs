use crate::perlin::Perlin;
use cgmath::Point3;
use rand::RngCore;

use crate::color::{self, Color};

pub trait Texture: Send + Sync {
    fn value(&self, p: &Point3<f64>) -> Color;
    fn box_clone(&self) -> Box<dyn Texture>;
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: &Color) -> Self {
        Self { color: *color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: &Point3<f64>) -> Color {
        self.color
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, p: &Point3<f64>) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.value(p)
        } else {
            self.even.value(p)
        }
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64, rng: &mut dyn RngCore) -> Self {
        Self {
            scale,
            noise: Perlin::new(rng),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, p: &Point3<f64>) -> Color {
        color::white() * self.noise.turb(&(self.scale * *p), 7)
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}

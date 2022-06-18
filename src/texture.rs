use cgmath::Point3;

use crate::color::Color;

pub trait Texture: Send + Sync {
    fn value(&self, p: &Point3<f64>) -> Color;
}

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
}

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
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(p)
        } else {
            self.even.value(p)
        }
    }
}

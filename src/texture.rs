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

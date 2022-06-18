use crate::color::Color;

pub trait Texture: Send + Sync {
    fn value(&self) -> Color;
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
    fn value(&self) -> Color {
        self.color
    }
}

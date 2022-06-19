use crate::perlin::Perlin;
use cgmath::Point3;
use rand::RngCore;

use crate::color::{self, Color};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3<f64>) -> Color;
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
    fn value(&self, _: f64, _: f64, _: &Point3<f64>) -> Color {
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
    fn value(&self, u: f64, v: f64, p: &Point3<f64>) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
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
    fn value(&self, _: f64, _: f64, p: &Point3<f64>) -> Color {
        color::white()
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(&(self.scale * *p), 7)))
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}

const BYTES_PER_PIXEL: usize = 3;

#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3<f64>) -> Color {
        // Clamp input to texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let i = (u * self.width as f64) as usize;
        let j = (v * self.height as f64) as usize;

        // // Clamp integer mapping, since actual coordinates shoule be less than 1.0
        let i = i.min(self.width - 1);
        let j = j.min(self.height - 1);

        let bytes_per_scanline = BYTES_PER_PIXEL * self.width;
        let index = j * bytes_per_scanline + i * BYTES_PER_PIXEL;
        let pixel = self.data.as_slice().get(index..index+BYTES_PER_PIXEL).unwrap();
        let r = pixel[0] as f64 / 255.0;
        let g = pixel[1] as f64 / 255.0;
        let b = pixel[2] as f64 / 255.0;

        Color::new(r, g, b)
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}

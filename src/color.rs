use cgmath::Vector3;
use rand::{Rng, RngCore};

pub type Color = Vector3<f64>;

// Write the translated [0,255] value of each color component.
pub fn pnm_color(color: Color, samples_per_pixel: usize) -> String {
    let scale = 1.0 / samples_per_pixel as f64;
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();

    format!(
        "{} {} {}\n",
        (256.0 * r.clamp(0.0, 0.999)) as u8,
        (256.0 * g.clamp(0.0, 0.999)) as u8,
        (256.0 * b.clamp(0.0, 0.999)) as u8
    )
}

pub fn random_color(rng: &mut dyn RngCore) -> Color {
    let x = rng.gen_range(0.0..1.0);
    let y = rng.gen_range(0.0..1.0);
    let z = rng.gen_range(0.0..1.0);
    Color { x, y, z }
}

pub fn black() -> Color {
    Color { x: 0.0, y: 0.0, z: 0.0 }
}

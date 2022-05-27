use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    // Write the translated [0,255] value of each color component.
    pub fn pnm_color(&self, samples_per_pixel: usize) -> String {
        let scale = 1.0 / samples_per_pixel as f64;
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let r = (self.x() * scale).sqrt();
        let g = (self.y() * scale).sqrt();
        let b = (self.z() * scale).sqrt();

        format!(
            "{} {} {}\n",
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8
        )
    }
}

use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    // Write the translated [0,255] value of each color component.
    pub fn pnm_color(&self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8
        )
    }
}

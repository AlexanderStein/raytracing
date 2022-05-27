use crate::{color::*, ray::*, vec3::*};

mod color;
mod hitable;
mod ray;
mod vec3;
mod sphere;

use image::{load_from_memory_with_format, ImageFormat};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1280;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    let mut pnm_data = String::new();
    pnm_data += &format!("P3\n{} {}\n255\n\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!(
            "\rScanlines remaining: {} ({})%",
            y,
            ((1.0 - (y as f32 / IMAGE_HEIGHT as f32)) * 100.0) as u16
        );
        for x in 0..IMAGE_WIDTH {
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray.ray_color();
            pnm_data += &pixel_color.pnm_color();
        }
    }
    eprintln!("");
    // print!("{}", pnm_data);

    match load_from_memory_with_format(&pnm_data.into_bytes(), ImageFormat::Pnm) {
        Ok(img) => img.save("raytracer.png").unwrap(),
        Err(err) => println!("{}", err),
    }
}

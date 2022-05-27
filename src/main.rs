use crate::{color::*, hitable::HitableList, ray::*, sphere::*, vec3::*, camera::Camera};
use std::rc::Rc;

mod camera;
mod color;
mod hitable;
mod ray;
mod sphere;
mod vec3;

use image::{load_from_memory_with_format, ImageFormat};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 1280;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // World
    let mut world = HitableList { objects: vec![] };
    world.objects.push(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.objects.push(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

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
            let ray = camera.get_ray(u, v);
            let pixel_color = ray.color(&world);
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

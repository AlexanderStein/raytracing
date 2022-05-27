use crate::{color::*, hitable::HitableList, ray::*, sphere::*, vec3::*, camera::Camera};
use std::rc::Rc;
use rand::prelude::*;

mod camera;
mod color;
mod hitable;
mod ray;
mod sphere;
mod vec3;

use image::{load_from_memory_with_format, ImageFormat};

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    let mut rng = thread_rng();

    // Image
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;

    // World
    let mut world = HitableList { objects: vec![] };
    world.objects.push(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.objects.push(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    let mut pnm_data = String::new();
    pnm_data += &format!("P3\n{} {}\n255\n\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // #TODO Use rayon for parallel iteration
    // Speed up should be noticable with 1280 px image width
    for y in (0..IMAGE_HEIGHT).rev() {
        eprint!(
            "\rScanlines remaining: {} ({})%",
            y,
            ((1.0 - (y as f32 / IMAGE_HEIGHT as f32)) * 100.0) as u16
        );
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (y as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray.color(&world);
            }
            pnm_data += &pixel_color.pnm_color(SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("");
    // print!("{}", pnm_data);

    match load_from_memory_with_format(&pnm_data.into_bytes(), ImageFormat::Pnm) {
        Ok(img) => img.save("raytracer.png").unwrap(),
        Err(err) => println!("{}", err),
    }
}

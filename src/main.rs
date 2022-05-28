#[macro_use]
extern crate approx;

use crate::{camera::Camera, color::*, vec3::*, world::random_scene};
use image::{load_from_memory_with_format, ImageFormat};
use rand::prelude::*;

mod camera;
mod color;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

fn main() {
    let mut rng = thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 10;
    const MAX_DEPTH: usize = 50;

    // World
    let world = random_scene(&mut rng);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
                let ray = camera.get_ray(u, v, &mut rng);
                pixel_color += ray.color(&world, MAX_DEPTH, &mut rng);
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

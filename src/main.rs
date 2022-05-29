#[cfg(test)]
#[macro_use]
extern crate approx;

use crate::{camera::Camera, color::*, vec3::*, world::random_scene};
use clap::Parser;
use image::{load_from_memory_with_format, ImageFormat};
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

mod camera;
mod color;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

#[derive(Parser, Default, Debug)]
#[clap(author="Alexander Stein", version=env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT"), about)]
/// Ray Tracing in One Weekend
struct Arguments {
    #[clap(default_value_t = 0, short, long)]
    /// maximum threads to be used in parallel. 0 = all logical CPUs
    threads: usize,
    /// per pixel over-sampling. 500 for good results. Beware: time-consuming
    #[clap(default_value_t = 100, short, long)]
    samples_per_pixel: usize,
    /// image width in pixel
    #[clap(default_value_t = 480, long)]
    image_width: usize,
    /// image height in pixel
    #[clap(default_value_t = 320, long)]
    image_height: usize,
}

fn main() {
    let args = Arguments::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();

    let mut rng = thread_rng();

    // Image
    let image_width = args.image_width;
    let image_height = args.image_height;
    let ascpect_ratio = args.image_width as f64 / args.image_height as f64;
    const MAX_DEPTH: usize = 50;

    // World
    let world = random_scene(&mut rng);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::zero();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        ascpect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    let camera_ref = &camera;
    let world_ref = &world;
    let scanlines = Arc::new(AtomicUsize::new(image_height));
    let image: Vec<Color> = (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            let scanlines_left = scanlines.fetch_sub(1, Ordering::SeqCst);
            eprint!(
                "\rScanlines remaining: {} ({})%",
                scanlines_left,
                ((1.0 - (scanlines_left as f32 / image_height as f32)) * 100.0) as u16
            );
            (0..image_width).into_par_iter().map(move |x| {
                let sampled_pixel = (0..args.samples_per_pixel)
                    .map(move |_| {
                        let mut rng = thread_rng();
                        let u = (x as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                        let v = (y as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                        let ray = camera_ref.get_ray(u, v, &mut rng);
                        ray.color(world_ref, MAX_DEPTH, &mut rng)
                    })
                    .sum();
                sampled_pixel
            })
        })
        .collect();

    // Serialize to PNM
    let mut pnm_data = format!("P3\n{} {}\n255\n\n", image_width, image_height);
    for pixel in image {
        pnm_data += &pixel.pnm_color(args.samples_per_pixel);
    }
    eprintln!("");

    match load_from_memory_with_format(&pnm_data.into_bytes(), ImageFormat::Pnm) {
        Ok(img) => img.save("raytracer.png").unwrap(),
        Err(err) => println!("{}", err),
    }
}

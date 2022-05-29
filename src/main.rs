#[cfg(test)]
#[macro_use]
extern crate approx;

use crate::{camera::Camera, color::*, vec3::*, world::random_scene};
use clap::Parser;
use image::{load_from_memory_with_format, ImageFormat};
use rand::prelude::*;
use rayon::prelude::*;

mod camera;
mod color;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

#[derive(Parser,Default,Debug)]
#[clap(author="Alexander Stein", version=env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT"), about)]
/// Ray Tracing in One Weekend
struct Arguments {
    #[clap(default_value_t=0, short, long)]
    /// maximum threads to be used in parallel. 0 = all logical CPUs
    threads: usize,
    /// per pixel over-sampling. 500 for good results. Beware: time-consuming
    #[clap(default_value_t=100, short, long)]
    samples_per_pixel: usize,
}

fn main() {
    let args = Arguments::parse();
    rayon::ThreadPoolBuilder::new().num_threads(args.threads).build_global().unwrap();

    let mut rng = thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
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
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    //TODO: Create vector dynamically, so width & height can be runtime values
    let mut pixel_colors =  vec![[Color::new(0.0, 0.0, 0.0); IMAGE_WIDTH]; IMAGE_HEIGHT];
    pixel_colors
        .par_iter_mut()
        .enumerate()
        .for_each(|(y, column)| {
            // eprint!(
            //     "\rScanlines remaining: {} ({})%",
            //     y,
            //     ((1.0 - (y as f32 / IMAGE_HEIGHT as f32)) * 100.0) as u16
            // );
            column.par_iter_mut().enumerate().for_each(|(x, color)| {
                let mut rng = thread_rng();
                for _ in 0..args.samples_per_pixel {
                    let u = (x as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f64;
                    let v = (y as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f64;
                    let ray = camera.get_ray(u, v, &mut rng);
                    *color += ray.color(&world, MAX_DEPTH, &mut rng);
                }
            })
        });

    // Serialize to PNM
    let mut pnm_data = format!("P3\n{} {}\n255\n\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for rows in pixel_colors.iter().rev() {
        for pixel_color in rows {
            pnm_data += &pixel_color.pnm_color(args.samples_per_pixel);
        }
    }
    eprintln!("");

    match load_from_memory_with_format(&pnm_data.into_bytes(), ImageFormat::Pnm) {
        Ok(img) => img.save("raytracer.png").unwrap(),
        Err(err) => println!("{}", err),
    }
}

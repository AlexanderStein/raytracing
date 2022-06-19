#[cfg(test)]
#[macro_use]
extern crate approx;

use crate::{camera::Camera, color::*, world::*};
use cgmath::{Point3, Vector3};
use clap::{arg, command};
use image::{load_from_memory_with_format, ImageFormat};
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

mod aabb;
mod bvh;
mod camera;
mod color;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod texture;
mod world;

fn main() {
    let matches = command!()
        .version(env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT"))
        .author("Alexander Stein <alexander.stein@mailbox.org>")
        .about("Ray Tracing in One Weekend")
        .arg(
            arg!(
                -x --"image-width" <IMAGE_WIDTH> "image width in pixel"
            )
            .required(false)
            .default_value("480")
            .validator(|s| s.parse::<usize>())
        )
        .arg(
            arg!(
                -y --"image-height" <IMAGE_HEIGHT> "image height in pixel"
            )
            .required(false)
            .default_value("360")
            .validator(|s| s.parse::<usize>())
        )
        .arg(
            arg!(
                -s --"samples-per-pixel" <SAMPLES_PER_PIXEL> "per pixel over-sampling. 500 for good results. Beware: time-consuming"
            )
            .required(false)
            .default_value("100")
            .validator(|s| s.parse::<usize>())
        )
        .arg(
            arg!(
                -t --threads <THREADS> "maximum threads to be used in parallel. Default: all physical CPUs"
            )
            .required(false)
            .default_value(&format!("{}", num_cpus::get_physical()))
            .validator(|s| s.parse::<usize>())
        )
        .arg(
            arg!(
                -w --world <world> "Select world"
            )
            .required(false)
            .default_value("0")
            .validator(|s| s.parse::<usize>())
        )
        .get_matches();

    let threads: usize = matches.value_of_t("threads").unwrap();
    let samples_per_pixel: usize = matches.value_of_t("samples-per-pixel").unwrap();

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let mut rng = thread_rng();

    // Image
    let image_width: usize = matches.value_of_t("image-width").unwrap();
    let image_height: usize = matches.value_of_t("image-height").unwrap();
    let ascpect_ratio = image_width as f64 / image_height as f64;
    const MAX_DEPTH: usize = 50;

    // World
    let (world, lookfrom, lookat, vfov, aperture) = match matches.value_of_t("world").unwrap() {
        1 => {
            let lookfrom = Point3 {
                x: 13.0,
                y: 2.0,
                z: 3.0,
            };
            let lookat = Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let vfov = 20.0;
            let aperture = 0.0;

            (two_spheres(), lookfrom, lookat, vfov, aperture)
        }
        _ => {
            let lookfrom = Point3 {
                x: 13.0,
                y: 2.0,
                z: 3.0,
            };
            let lookat = Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let vfov = 20.0;
            let aperture = 0.1;
            (random_scene(&mut rng), lookfrom, lookat, vfov, aperture)
        }
    };

    // Camera
    let vup = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ascpect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Render
    let camera_ref = &camera;
    let world_ref = &world;
    let bar = &Box::new(ProgressBar::new((image_width * image_height) as u64));
    bar.set_prefix("   Rendering");
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.white} [{elapsed_precise}] [{eta_precise}] {wide_bar} {pos:>7}/{len:7} ({per_sec}) ({percent}%)"),
    );
    bar.set_draw_rate(25);

    let image: Vec<Color> = (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            (0..image_width).into_par_iter().map(move |x| {
                let sampled_pixel = (0..samples_per_pixel)
                    .map(move |_| {
                        let mut rng = thread_rng();
                        let u = (x as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                        let v = (y as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                        let ray = camera_ref.get_ray(u, v, &mut rng);
                        ray.color(world_ref, MAX_DEPTH, &mut rng)
                    })
                    .sum();
                bar.inc(1);

                sampled_pixel
            })
        })
        .collect();

    bar.finish();

    // Serialize to PNM
    let mut pnm_data = format!("P3\n{} {}\n255\n\n", image_width, image_height);
    for pixel in image {
        pnm_data += &pnm_color(pixel, samples_per_pixel);
    }

    match load_from_memory_with_format(&pnm_data.into_bytes(), ImageFormat::Pnm) {
        Ok(img) => img.save("raytracer.png").unwrap(),
        Err(err) => println!("{}", err),
    }
}

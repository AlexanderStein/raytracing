use crate::{color::*, hitable_list::HitableList, material::*, sphere::*, texture::*};
use cgmath::{InnerSpace, Point3, Vector3};
use rand::{Rng, RngCore};

pub fn random_scene(rng: &mut dyn RngCore) -> HitableList {
    // TODO: Create Vec<Box<dyn Hittable>> first and pass this to world
    let mut world = HitableList::new();

    let checker_texture = Box::new(CheckerTexture::new(
        Box::new(SolidColor::new(&Color::new(0.2, 0.3, 0.1))),
        Box::new(SolidColor::new(&Color::new(0.9, 0.9, 0.9))),
    ));
    let ground_material = Box::new(Lambertian::new(checker_texture));
    world.push(Sphere::new(
        Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen_range(0.0..1.0);
            let center = Point3 {
                x: a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                y: 0.2,
                z: b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            };

            if (center
                - Point3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
            .magnitude()
                > 0.9
            {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Box::new(SolidColor::new(&random_color(rng)));
                    let center2 = center
                        + Vector3 {
                            x: 0.0,
                            y: rng.gen_range(0.0..0.5),
                            z: 0.0,
                        };
                    world.push(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Box::new(Lambertian::new(albedo)),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_color(rng);
                    let fuzz = rng.gen_range(0.0..1.0);
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(&albedo, fuzz)),
                    ));
                } else {
                    // glass
                    world.push(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5))));
                };
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.push(Sphere::new(
        Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        material1,
    ));

    let color2 = Box::new(SolidColor::new(&Color::new(0.4, 0.2, 0.1)));
    let material2 = Box::new(Lambertian::new(color2));
    world.push(Sphere::new(
        Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        material2,
    ));

    let material3 = Box::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(
        Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        material3,
    ));

    world
}

pub fn two_spheres(_rng: &mut dyn RngCore) -> HitableList {
    let mut world = HitableList::new();

    let checker = Box::new(CheckerTexture::new(
        Box::new(SolidColor::new(&Color::new(0.2, 0.3, 0.1))),
        Box::new(SolidColor::new(&Color::new(0.9, 0.9, 0.9))),
    ));
    world.push(Sphere::new(
        Point3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
        10.0,
        Box::new(Lambertian::new(checker.clone())),
    ));
    world.push(Sphere::new(
        Point3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
        10.0,
        Box::new(Lambertian::new(checker)),
    ));

    world
}

pub fn two_perlin_spheres(rng: &mut dyn RngCore) -> HitableList {
    let mut world = HitableList::new();

    let noise = Box::new(NoiseTexture::new(4.0, rng));
    world.push(Sphere::new(
        Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        1000.0,
        Box::new(Lambertian::new(noise.clone())),
    ));
    world.push(Sphere::new(
        Point3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        2.0,
        Box::new(Lambertian::new(noise)),
    ));

    world
}

use crate::{
    aarect::*, color::*, cuboid::Cuboid, hitable_list::HitableList, material::*, rotate::RotateY,
    sphere::*, texture::*, translate::Translate,
};
use cgmath::{InnerSpace, Point3, Vector3};
use rand::{Rng, RngCore};

#[allow(dead_code)]
pub fn empty(_rng: &mut dyn RngCore) -> HitableList {
    HitableList::new()
}

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

pub fn earth(_rng: &mut dyn RngCore) -> HitableList {
    let mut world = HitableList::new();
    let image = image::open("earthmap.png")
        .expect("image not found")
        .to_rgb8();
    let (width, height) = image.dimensions();
    let data = image.into_raw();
    let earth_texture = Box::new(ImageTexture::new(data, width as usize, height as usize));
    world.push(Sphere::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        2.0,
        Box::new(Lambertian::new(earth_texture)),
    ));

    world
}

pub fn simple_light(rng: &mut dyn RngCore) -> HitableList {
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

    world.push(XYRect {
        material: DiffuseLight::with_color(&Color::new(4.0, 4.0, 4.0)),
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
    });

    world
}

pub fn cornell_box(_rng: &mut dyn RngCore) -> HitableList {
    let mut world = HitableList::new();

    let red = Lambertian::new(Box::new(SolidColor::new(&Color::new(0.65, 0.05, 0.05))));
    let white = Lambertian::new(Box::new(SolidColor::new(&Color::new(0.73, 0.73, 0.73))));
    let green = Lambertian::new(Box::new(SolidColor::new(&Color::new(0.12, 0.45, 0.15))));
    let light = DiffuseLight::with_color(&Color::new(15.0, 15.0, 15.0));

    world.push(YZRect {
        material: green,
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    });
    world.push(YZRect {
        material: red,
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    });
    world.push(XZRect {
        material: light,
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
    });
    world.push(XZRect {
        material: white.clone(),
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    });
    world.push(XZRect {
        material: white.clone(),
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    });
    world.push(XYRect {
        material: white.clone(),
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
    });

    let box1 = Cuboid::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 165.0,
            y: 330.0,
            z: 165.0,
        },
        white.clone(),
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(
        Vector3 {
            x: 265.0,
            y: 0.0,
            z: 295.0,
        },
        box1,
    );
    world.push(box1);
    let box2 = Cuboid::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Point3 {
            x: 165.0,
            y: 165.0,
            z: 165.0,
        },
        white,
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(
        Vector3 {
            x: 130.0,
            y: 0.0,
            z: 65.0,
        },
        box2,
    );
    world.push(box2);

    world
}

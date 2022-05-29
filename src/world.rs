use crate::{color::*, hitable::HitableList, material::*, sphere::*, vec3::*};
use rand::{Rng, RngCore};

pub fn random_scene(rng: &mut dyn RngCore) -> HitableList {
    // TODO: Create Vec<Box<dyn Hittable>> first and pass this to world
    let mut world = HitableList::new();

    let ground_material = Lambertian::new(&Color::new(0.5, 0.5, 0.5));
    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(rng, 0.0..1.0) * Color::random(rng, 0.0..1.0);
                    let sphere_material = Lambertian::new(&albedo);
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(rng, 0.5..1.0);
                    let fuzz = rng.gen_range(0.0..1.0);
                    let sphere_material = Metal::new(&albedo, fuzz);
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.push(Sphere::new(center, 0.2, sphere_material));
                };
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(&Color::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}

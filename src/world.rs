use crate::{color::*, hitable::HitableList, material::*, sphere::*, vec3::*};
use rand::{Rng, RngCore};
use std::sync::Arc;

pub fn random_scene(rng: &mut dyn RngCore) -> HitableList {
    // TODO: Create Vec<Arc<dyn Hittable>> first and pass this to world
    let mut world = HitableList { objects: vec![] };

    let ground_material = Arc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn MaterialTrait> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(rng, 0.0..1.0) * Color::random(rng, 0.0..1.0);
                    Arc::new(Lambertian::new(&albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(rng, 0.5..1.0);
                    let fuzz = rng.gen_range(0.0..1.0);
                    Arc::new(Metal::new(&albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                world
                    .objects
                    .push(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.objects.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));

    let material2 = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.objects.push(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));

    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.objects.push(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));

    world
}

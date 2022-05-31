use cgmath::*;
use rand::{Rng, RngCore};

pub fn random_in_unit_sphere(rng: &mut dyn RngCore) -> Vector3<f64> {
    loop {
        let point = Vector3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: rng.gen_range(-1.0..1.0),
        };
        if point.magnitude2() < 1.0 {
            return point;
        }
    }
}

pub fn random_unit_vector(rng: &mut dyn RngCore) -> Vector3<f64> {
    random_in_unit_sphere(rng).normalize()
}

pub fn random_in_unit_disk(rng: &mut dyn RngCore) -> Vector3<f64> {
    loop {
        let point = Vector3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: 0.0,
        };
        if point.magnitude2() < 1.0 {
            return point;
        }
    }
}

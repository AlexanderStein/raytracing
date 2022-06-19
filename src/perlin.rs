use cgmath::Point3;
use rand::{Rng, RngCore};

const POINT_COUNT: usize = 256;

fn generate_perm(rng: &mut dyn RngCore) -> Vec<usize> {
    let mut p: Vec<usize> = (0..POINT_COUNT).collect();

    for i in (0..p.len()).rev() {
        let target = rng.gen_range(0..=i);
        p.swap(i, target);
    }

    p
}

#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        let ranfloat = rng
            .sample_iter(rand::distributions::Standard)
            .take(POINT_COUNT)
            .collect();

        Self {
            ranfloat,
            perm_x: generate_perm(rng),
            perm_y: generate_perm(rng),
            perm_z: generate_perm(rng),
        }
    }

    pub fn noise(&self, p: &Point3<f64>) -> f64 {
        // Cast to usize _after_ doing & operation,
        // otherwise negative coordinates result in index 0
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

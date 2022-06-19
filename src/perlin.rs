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

fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    c.iter().enumerate().fold(0.0, |accum, (i, i_iter)| {
        i_iter.iter().enumerate().fold(accum, |accum, (j, j_iter)| {
            j_iter.iter().enumerate().fold(accum, |accum, (k, k_iter)| {
                accum
                    + (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * *k_iter
            })
        })
    })
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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[0.0; 2]; 2]; 2];
        c.iter_mut().enumerate().for_each(|(di, i_iter)| {
            i_iter.iter_mut().enumerate().for_each(|(dj, j_iter)| {
                j_iter.iter_mut().enumerate().for_each(|(dk, k_iter)| {
                    *k_iter = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                })
            })
        });

        return trilinear_interp(&c, u, v, w);
    }
}

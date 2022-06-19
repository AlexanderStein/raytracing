use cgmath::{InnerSpace, Point3, Vector3};
use rand::{Rng, RngCore};

const POINT_COUNT: usize = 256;

fn generate(rng: &mut dyn RngCore) -> Vec<Vector3<f64>> {
    (0..POINT_COUNT)
        .map(|_| Vector3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: rng.gen_range(-1.0..1.0),
        })
        .collect()
}

fn generate_perm(rng: &mut dyn RngCore) -> Vec<usize> {
    let mut p: Vec<usize> = (0..POINT_COUNT).collect();

    for i in (0..p.len()).rev() {
        let target = rng.gen_range(0..=i);
        p.swap(i, target);
    }

    p
}

fn trilinear_interp(c: &[[[Vector3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    c.iter().enumerate().fold(0.0, |accum, (i, i_iter)| {
        i_iter.iter().enumerate().fold(accum, |accum, (j, j_iter)| {
            j_iter.iter().enumerate().fold(accum, |accum, (k, k_iter)| {
                let weight_v = Vector3::new(u - i as f64, v - j as f64, w - k as f64);
                accum
                    + (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * k_iter.dot(weight_v)
            })
        })
    })
}
#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<Vector3<f64>>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        Self {
            ranfloat: generate(rng),
            perm_x: generate_perm(rng),
            perm_y: generate_perm(rng),
            perm_z: generate_perm(rng),
        }
    }

    pub fn noise(&self, p: &Point3<f64>) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vector3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        c.iter_mut().enumerate().for_each(|(di, i_iter)| {
            i_iter.iter_mut().enumerate().for_each(|(dj, j_iter)| {
                j_iter.iter_mut().enumerate().for_each(|(dk, k_iter)| {
                    *k_iter = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                })
            })
        });

        trilinear_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3<f64>, depth: usize) -> f64 {
        let mut weight = 1.0;
        let mut temp_p = *p;
        (0..depth).fold(0.0, | accum, _ | {
            let accum = accum + weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
            accum
        }).abs()
    }
}

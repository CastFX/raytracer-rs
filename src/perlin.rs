use rand::{prelude::SliceRandom, thread_rng, Rng};

use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let ranfloat =
            (0..Self::POINT_COUNT).map(|_| rng.gen::<f64>()).collect();
        let ranvec = (0..Self::POINT_COUNT)
            .map(|_| Vec3::random_in_unit_sphere().normalized())
            .collect();

        let mut gen_perm = || {
            let mut perm: Vec<usize> = (0..Self::POINT_COUNT).collect();
            perm.shuffle(&mut rng);
            perm
        };

        Perlin {
            ranfloat,
            ranvec,
            perm_x: gen_perm(),
            perm_y: gen_perm(),
            perm_z: gen_perm(),
        }
    }

    pub fn _noise(&self, p: &Point3) -> f64 {
        let i = (p.x()).rem_euclid(Self::POINT_COUNT as f64) as usize;
        let j = (p.y()).rem_euclid(Self::POINT_COUNT as f64) as usize;
        let k = (p.z()).rem_euclid(Self::POINT_COUNT as f64) as usize;
        let index = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];

        self.ranfloat[index]
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        fn split(x: f64) -> (usize, f64, f64) {
            let r = x.rem_euclid(Perlin::POINT_COUNT as f64);
            let u = r.rem_euclid(1.0);
            (r as usize, u, u * u * (3.0 - 2.0 * u))
        }
        let (i, u, uu) = split(p.x());
        let (j, v, vv) = split(p.y());
        let (k, w, ww) = split(p.z());

        let mut acc = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let b = self.ranvec[self.perm_x
                        [(i + di) % Self::POINT_COUNT]
                        ^ self.perm_y[(j + dj) % Self::POINT_COUNT]
                        ^ self.perm_z[(k + dk) % Self::POINT_COUNT]];
                    let f = (if di == 0 { 1.0 - uu } else { uu })
                        * (if dj == 0 { 1.0 - vv } else { vv })
                        * (if dk == 0 { 1.0 - ww } else { ww });

                    acc += f * b.dot(Vec3::new(
                        u - di as f64,
                        v - dj as f64,
                        w - dk as f64,
                    ));
                }
            }
        }
        acc
    }

    pub fn turbolence(&self, p: Point3, depth: i32) -> f64 {
        let (turb, _, _): (f64, Vec3, f64) =
            (0..depth).fold((0.0, p, 1.0), |(accum, p, weight), _| {
                (accum + weight * self.noise(&p), p * 2.0, weight * 0.5)
            });
        turb.abs()
    }
}

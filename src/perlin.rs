use crate::point::Point;
use rand::Rng;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut randfloat = [0.0; POINT_COUNT];
        for x in randfloat.iter_mut() {
            *x = rand::thread_rng().gen();
        }

        Perlin {
            randfloat,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let i = (((4.0 * p.0[0]) as i32) & 255) as usize;
        let j = (((4.0 * p.0[1]) as i32) & 255) as usize;
        let k = (((4.0 * p.0[2]) as i32) & 255) as usize;

        self.randfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }

    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut res = [0; POINT_COUNT];

        for (i, x) in res.iter_mut().enumerate() {
            *x = i as i32;
        }

        res = Perlin::permute(res, POINT_COUNT);
        res
    }

    fn permute(mut p: [i32; POINT_COUNT], n: usize) -> [i32; POINT_COUNT] {
        for i in (1..n).rev() {
            let target = rand::thread_rng().gen_range(0..i);
            p.swap(i, target);
        }
        p
    }
}

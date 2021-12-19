use crate::point::Point;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.e[0] * rhs[0] + self.e[1] * rhs[1] + self.e[2] * rhs[2]
    }

    pub fn cross(&self, rhs: &Self) -> Vec3 {
        Vec3::new(
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.e[0]) < s && f64::abs(self.e[1]) < s && f64::abs(self.e[2]) < s
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * self.dot(normal) * 2.0
    }

    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = f64::min(-self.dot(normal), 1.0);
        let r_out_perpendicular = etai_over_etat * (*self + *normal * cos_theta);
        let r_out_parallel =
            -f64::sqrt(f64::abs(1.0 - r_out_perpendicular.length_squared())) * *normal;
        r_out_perpendicular + r_out_parallel
    }

    pub fn random_in_unit_sphere() -> Point {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            if p.length_squared() < 1.0 {
                return Point(p);
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().0.unit_vector()
    }

    pub fn random_in_unit_disk() -> Point {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return Point(p);
            }
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
        let mut v1 = Vec3::new(3.0, 2.0, 1.0);
        v1 *= 2.0;
        assert_eq!(v1[0], 6.0);
    }
}

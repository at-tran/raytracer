use crate::vec3::Vec3;
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul};

#[derive(Copy, Clone)]
pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn to_rgb_array(self) -> [u8; 3] {
        [
            (256.0 * f64::clamp(self.0[0], 0.0, 0.999)) as u8,
            (256.0 * f64::clamp(self.0[1], 0.0, 0.999)) as u8,
            (256.0 * f64::clamp(self.0[2], 0.0, 0.999)) as u8,
        ]
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(rhs.0 * self)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}

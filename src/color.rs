use crate::vec3::Vec3;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn to_rgb_array(self) -> [u8; 3] {
        [
            (255.999 * self.0[0]) as u8,
            (255.999 * self.0[1]) as u8,
            (255.999 * self.0[2]) as u8,
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

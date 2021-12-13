use crate::vec3::Vec3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct Point(pub Vec3);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(Vec3::new(x, y, z))
    }
}

impl Add<Vec3> for Point {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point(self.0 + rhs)
    }
}

impl Sub<Vec3> for Point {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Point(self.0 - rhs)
    }
}

impl Sub for Point {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

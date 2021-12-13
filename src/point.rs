use crate::vec3::Vec3;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Point(pub Vec3);

impl Add<Vec3> for Point {
    type Output = Point;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point(self.0 + rhs)
    }
}

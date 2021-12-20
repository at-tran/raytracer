use crate::point::Point;
use crate::vec3::Vec3;

pub struct Ray {
    orig: Point,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3, time: f64) -> Ray {
        Ray { orig, dir, time }
    }

    pub fn origin(&self) -> &Point {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}

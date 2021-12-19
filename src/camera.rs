use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let h = f64::tan(theta / 2.0);
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner - self.origin + s * self.horizontal + t * self.vertical,
        )
    }
}

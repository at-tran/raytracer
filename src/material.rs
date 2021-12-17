use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Sphere::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        Some((Ray::new(rec.p, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Sphere::random_in_unit_sphere().0,
        );
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

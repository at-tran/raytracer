use crate::camera::Camera;
use crate::color::Color;
use crate::hit::{Hit, HitList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use rand::Rng;
use rayon::prelude::*;

mod camera;
mod color;
mod hit;
mod material;
mod point;
mod ray;
mod sphere;
mod vec3;

fn ray_color<T: Hit>(r: &Ray, world: &T, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
            return Color(attenuation.0 * ray_color(&scattered, world, depth - 1).0);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut world = HitList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Dielectric::new(1.5);
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.push(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.push(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.push(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.push(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, material_right));

    let cam = Camera::new();

    let mut img_buf = image::ImageBuffer::new(image_width, image_height);
    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j + 1);
        for i in 0..image_width {
            let pixel_color_sum: Color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let u =
                        (i as f64 + rand::thread_rng().gen::<f64>()) / (image_width as f64 - 1.0);
                    let v =
                        (j as f64 + rand::thread_rng().gen::<f64>()) / (image_height as f64 - 1.0);

                    let r = cam.get_ray(u, v);
                    ray_color(&r, &world, max_depth)
                })
                .sum();

            let pixel_color = Color(pixel_color_sum.0 / samples_per_pixel as f64);

            img_buf.put_pixel(
                i,
                image_height - j - 1,
                image::Rgb(pixel_color.to_rgb_array()),
            );
        }
    }
    img_buf.save("image.png").unwrap();
    println!("Done.");
}

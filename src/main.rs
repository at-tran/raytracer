use crate::camera::Camera;
use crate::color::Color;
use crate::hit::{Hit, HitList};
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::Rng;
use rayon::prelude::*;

mod camera;
mod color;
mod hit;
mod point;
mod ray;
mod sphere;
mod vec3;

fn ray_color<T: Hit>(r: &Ray, world: &T) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (Color(rec.normal) + Color::new(1.0, 1.0, 1.0));
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

    let mut world = HitList::new();
    world.push(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::new();

    let mut img_buf = image::ImageBuffer::new(image_width, image_height);
    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j + 1);
        for i in 0..image_width {
            let pixel_color_sum: Color = (0..samples_per_pixel).into_par_iter().map(|_| {
                let u = (i as f64 + rand::thread_rng().gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rand::thread_rng().gen::<f64>()) / (image_height as f64 - 1.0);

                let r = cam.get_ray(u, v);
                ray_color(&r, &world)
            }).sum();

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

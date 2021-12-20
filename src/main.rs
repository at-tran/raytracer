use crate::camera::Camera;
use crate::color::Color;
use crate::hit::{Hit, HitList};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
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

fn random_scene() -> HitList<'static> {
    let mut world = HitList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let center = Point::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            let choose_mat: f64 = rng.gen();
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color(
                        Color::new(rng.gen(), rng.gen(), rng.gen()).0
                            * Color::new(rng.gen(), rng.gen(), rng.gen()).0,
                    );
                    let material = Lambertian::new(albedo);
                    let center_end = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.push(Sphere::new_moving(center, center_end, 0.0, 1.0, 0.2, material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    );
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.push(Sphere::new(center, 0.2, material));
                } else {
                    // dielectric
                    let material = Dielectric::new(1.5);
                    world.push(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.push(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let world = random_scene();

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

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

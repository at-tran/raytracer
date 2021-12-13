use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod color;
mod point;
mod ray;
mod vec3;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height as f64 * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut img_buf = image::ImageBuffer::new(image_width, image_height);
    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j + 1);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);

            let r= Ray::new(origin, lower_left_corner - origin + u * horizontal + v * vertical);
            let c = ray_color(&r);

            img_buf.put_pixel(i, image_height - j - 1, image::Rgb(c.to_rgb_array()));
        }
    }
    img_buf.save("image.png").unwrap();
    println!("Done.");
}

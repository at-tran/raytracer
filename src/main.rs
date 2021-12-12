mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut img_buf = image::ImageBuffer::new(image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);
        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1.0);
            let g = j as f64 / (image_height as f64 - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            img_buf.put_pixel(i, image_height - j - 1, image::Rgb([ir, ig, ib]));
        }
    }
    img_buf.save("image.png").unwrap();
    eprintln!("Done.");
}

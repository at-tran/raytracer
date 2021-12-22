use crate::color::Color;
use crate::point::Point;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: &Point) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture + Send + Sync>,
    even: Box<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new(
        even: impl Texture + Send + Sync + 'static,
        odd: impl Texture + Send + Sync + 'static,
    ) -> Self {
        Self {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }

    pub fn from_colors(even: Color, odd: Color) -> Self {
        Self {
            odd: Box::new(SolidColor::new(odd)),
            even: Box::new(SolidColor::new(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = f64::sin(10.0 * p.0[0]) * f64::sin(10.0 * p.0[1]) * f64::sin(10.0 * p.0[2]);
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

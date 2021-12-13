use crate::vec3::Vec3;
use std::fmt::{Display, Formatter};

pub struct Color(pub Vec3);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.0[0]) as u8,
            (255.999 * self.0[1]) as u8,
            (255.999 * self.0[2]) as u8
        )
    }
}

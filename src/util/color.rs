use crate::util::Vec3;
use image::Rgb;

const COLOR_SCALE: f64 = 255.999;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub Vec3);

impl Color {
    pub fn to_pixel(self) -> Rgb<u8> {
        Rgb([
            (self.0.x() * COLOR_SCALE) as u8,
            (self.0.y() * COLOR_SCALE) as u8,
            (self.0.z() * COLOR_SCALE) as u8,
        ])
    }
}

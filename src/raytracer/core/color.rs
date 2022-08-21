use std::ops;

use image::Rgb;
use serde::{Deserialize, Serialize};

use crate::raytracer::core::Vec3;

const COLOR_SCALE: f64 = 255.999;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Color(pub Vec3);

impl Default for Color {
    fn default() -> Self {
        Color(Vec3::new(0.0, 0.0, 0.0))
    }
}

impl Color {
    pub fn ones() -> Color {
        Color(Vec3::new(1.0, 1.0, 1.0))
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Color {
    pub fn to_pixel(self) -> Rgb<u8> {
        Rgb([
            (self.0.x() * COLOR_SCALE) as u8,
            (self.0.y() * COLOR_SCALE) as u8,
            (self.0.z() * COLOR_SCALE) as u8,
        ])
    }
}

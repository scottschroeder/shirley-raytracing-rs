use crate::raytracer::core::{Color, Point};

pub mod checker;
pub mod solid;
pub mod image_texture;

pub trait Texture: std::fmt::Debug {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}


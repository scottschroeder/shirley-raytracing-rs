use crate::raytracer::core::{Color, Point};

pub mod checker;
pub mod image_texture;
pub mod loader;
pub mod solid;

pub trait Texture: std::fmt::Debug {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

use std::hash::Hash;

use crate::raytracer::core::{Color, Point};
use serde::{Deserialize, Serialize};

pub mod checker;
pub mod image_texture;
pub mod loader;
pub mod solid;

pub trait Texture: std::fmt::Debug {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorSetting(Color);

impl Hash for ColorSetting {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 .0.x() as u64).hash(state);
        (self.0 .0.y() as u64).hash(state);
        (self.0 .0.z() as u64).hash(state);
    }
}

impl PartialEq for ColorSetting {
    fn eq(&self, other: &Self) -> bool {
        let sx = self.0 .0.x() as u64;
        let sy = self.0 .0.y() as u64;
        let sz = self.0 .0.z() as u64;

        let ox = other.0 .0.x() as u64;
        let oy = other.0 .0.y() as u64;
        let oz = other.0 .0.z() as u64;

        sx.eq(&ox) && sy.eq(&oy) && sz.eq(&oz)
    }
}

impl Eq for ColorSetting {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScalarSetting(f64);

impl Hash for ScalarSetting {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as u64).hash(state);
    }
}

impl PartialEq for ScalarSetting {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as u64) == (other.0 as u64)
    }
}

impl Eq for ScalarSetting {}

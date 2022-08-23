pub mod dielectric;
pub mod lambertian;
pub mod lighting;
pub mod material_type;
pub mod metal;
pub mod perlin;
pub mod texture;

use rand::Rng;

use super::geometry::hittable::HitRecord;
use crate::core::{Color, Ray};

#[derive(Debug, Clone)]
pub struct Scatter {
    pub direction: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter<R: Rng>(&self, rng: &mut R, ray: &Ray, record: &HitRecord) -> Option<Scatter>;
    fn emitted(&self, _ray: &Ray, _record: &HitRecord) -> Option<Color> {
        None
    }
}

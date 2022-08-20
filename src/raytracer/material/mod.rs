
pub mod dielectric;
pub mod lambertian;
pub mod lighting;
pub mod metal;
pub mod perlin;
pub mod texture;
pub mod material_type;


use super::geometry::hittable::HitRecord;
use crate::raytracer::core::{Color, Ray};

#[derive(Debug, Clone)]
pub struct Scatter {
    pub direction: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<Scatter>;
    fn emitted(&self, _ray: &Ray, _record: &HitRecord) -> Option<Color> {
        None
    }
}


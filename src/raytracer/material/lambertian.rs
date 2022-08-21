use serde::{Deserialize, Serialize};

use super::{texture::Texture, Material, Scatter};
use crate::raytracer::{
    core::{math::random_unit_vector, Ray},
    geometry::hittable::HitRecord,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lambertian<T> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Lambertian<T> {
        Lambertian { albedo: texture }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let mut scatter = record.normal + random_unit_vector();
        if scatter.near_zero() {
            scatter = record.normal;
        }
        let direction = Ray {
            orig: record.point,
            direction: scatter,
        };

        Some(Scatter {
            direction,
            attenuation: self.albedo.value(record.u, record.v, &record.point),
        })
    }
}

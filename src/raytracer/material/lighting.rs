use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{texture::Texture, Material, Scatter};
use crate::{
    core::{math::random_unit_vector, Color, Ray},
    geometry::hittable::HitRecord,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffuseLight<T> {
    pub albedo: T,
}

impl<T> DiffuseLight<T> {
    pub fn new(texture: T) -> DiffuseLight<T> {
        DiffuseLight { albedo: texture }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, _ray: &Ray, record: &HitRecord) -> Option<Color> {
        Some(self.albedo.value(record.u, record.v, &record.point))
    }

    fn scatter<R: Rng>(&self, _rng: &mut R, _ray: &Ray, _record: &HitRecord) -> Option<Scatter> {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairyLight<T> {
    pub albedo: T,
}

impl<T> FairyLight<T> {
    pub fn new(texture: T) -> FairyLight<T> {
        FairyLight { albedo: texture }
    }
}

impl<T: Texture> Material for FairyLight<T> {
    fn scatter<R: Rng>(&self, rng: &mut R, _ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let mut scatter = record.normal + random_unit_vector(rng);
        if scatter.near_zero() {
            scatter = record.normal;
        }
        let direction = Ray {
            orig: record.point,
            direction: scatter,
        };
        let attenuation = self.albedo.value(record.u, record.v, &record.point);
        Some(Scatter {
            direction,
            attenuation: Color(attenuation.0.unit()),
        })
    }

    fn emitted(&self, ray: &Ray, record: &HitRecord) -> Option<Color> {
        let src_color = self.albedo.value(record.u, record.v, &record.point);

        let scale = record.normal.dot(&ray.direction.scale(-1.0));
        log::trace!("scale: {}", scale);
        Some(Color(src_color.0.scale(scale / ray.direction.length())))
        // Some(src_color)
    }
}

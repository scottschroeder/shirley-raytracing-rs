use serde::{Deserialize, Serialize};

use super::{texture::Texture, Material, Scatter};
use crate::raytracer::{
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
    fn scatter(&self, _ray: &Ray, _record: &HitRecord) -> Option<Scatter> {
        None
    }

    fn emitted(&self, _ray: &Ray, record: &HitRecord) -> Option<Color> {
        Some(self.albedo.value(record.u, record.v, &record.point))
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
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let mut scatter = record.normal + random_unit_vector();
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

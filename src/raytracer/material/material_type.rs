use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::{
    dielectric::Dielectric,
    lambertian::Lambertian,
    lighting::{DiffuseLight, FairyLight},
    metal::Metal,
    texture::{
        loader::{LoadableTexture, TextureManager},
        Texture,
    },
    Material,
};

pub type SceneMaterial = MaterialType<Arc<dyn Texture + Send + Sync>>;

#[derive(Serialize, Deserialize)]
pub enum MaterialType<T> {
    Metal(Metal),
    Dielectric(Dielectric),
    Lambertian(Lambertian<T>),
    DiffuseLight(DiffuseLight<T>),
    FairyLight(FairyLight<T>),
}

impl<T: LoadableTexture> MaterialType<T> {
    pub fn load_texture(self, manager: &mut TextureManager) -> anyhow::Result<SceneMaterial> {
        Ok(match self {
            MaterialType::Lambertian(l) => {
                let t = l.albedo.load_texture(manager)?;
                MaterialType::Lambertian(Lambertian { albedo: t })
            }
            MaterialType::DiffuseLight(d) => {
                let t = d.albedo.load_texture(manager)?;
                MaterialType::DiffuseLight(DiffuseLight { albedo: t })
            }
            MaterialType::FairyLight(f) => {
                let t = f.albedo.load_texture(manager)?;
                MaterialType::FairyLight(FairyLight { albedo: t })
            }
            MaterialType::Metal(m) => MaterialType::Metal(m),
            MaterialType::Dielectric(m) => MaterialType::Dielectric(m),
        })
    }
}

impl<T: Texture> Material for MaterialType<T> {
    fn scatter(
        &self,
        ray: &crate::core::Ray,
        record: &crate::geometry::hittable::HitRecord,
    ) -> Option<super::Scatter> {
        match self {
            MaterialType::Metal(m) => m.scatter(ray, record),
            MaterialType::Dielectric(m) => m.scatter(ray, record),
            MaterialType::Lambertian(m) => m.scatter(ray, record),
            MaterialType::DiffuseLight(m) => m.scatter(ray, record),
            MaterialType::FairyLight(m) => m.scatter(ray, record),
        }
    }

    fn emitted(
        &self,
        ray: &crate::core::Ray,
        record: &crate::geometry::hittable::HitRecord,
    ) -> Option<crate::core::Color> {
        match self {
            MaterialType::Metal(m) => m.emitted(ray, record),
            MaterialType::Dielectric(m) => m.emitted(ray, record),
            MaterialType::Lambertian(m) => m.emitted(ray, record),
            MaterialType::DiffuseLight(m) => m.emitted(ray, record),
            MaterialType::FairyLight(m) => m.emitted(ray, record),
        }
    }
}

impl<T> From<Metal> for MaterialType<T> {
    fn from(x: Metal) -> Self {
        MaterialType::Metal(x)
    }
}
impl<T> From<Dielectric> for MaterialType<T> {
    fn from(x: Dielectric) -> Self {
        MaterialType::Dielectric(x)
    }
}
impl<T> From<Lambertian<T>> for MaterialType<T> {
    fn from(x: Lambertian<T>) -> Self {
        MaterialType::Lambertian(x)
    }
}
impl<T> From<DiffuseLight<T>> for MaterialType<T> {
    fn from(x: DiffuseLight<T>) -> Self {
        MaterialType::DiffuseLight(x)
    }
}
impl<T> From<FairyLight<T>> for MaterialType<T> {
    fn from(x: FairyLight<T>) -> Self {
        MaterialType::FairyLight(x)
    }
}

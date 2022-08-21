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

#[derive(Serialize, Deserialize)]
pub enum MaterialType<T> {
    Metal(Metal),
    Dielectric(Dielectric),
    Lambertian(Lambertian<T>),
    DiffuseLight(DiffuseLight<T>),
    FairyLight(FairyLight<T>),
}

impl<T: LoadableTexture> MaterialType<T> {
    pub fn load_texture(
        self,
        manager: &mut TextureManager,
    ) -> anyhow::Result<MaterialType<Arc<dyn Texture + Send + Sync>>> {
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

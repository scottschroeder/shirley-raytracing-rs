use serde::{Deserialize, Serialize};

use super::{
    image_texture::{earth_builtin, ImageTexture},
    solid::ConstantTexture,
    Texture,
};
use crate::raytracer::{core::Color, material::perlin::NoiseTexture};

#[derive(Debug, Serialize, Deserialize)]
pub enum TextureLoader {
    Solid(Color),
    ImagePath(std::path::PathBuf),
    Perlin,
    EarthBuiltin,
}

impl TextureLoader {
    fn load(self) -> anyhow::Result<Box<dyn Texture + Send + Sync>> {
        Ok(match self {
            TextureLoader::Solid(c) => Box::new(ConstantTexture::from(c)),
            TextureLoader::ImagePath(p) => Box::new(ImageTexture::load_from_filename(p)?),
            TextureLoader::EarthBuiltin => Box::new(earth_builtin()),
            TextureLoader::Perlin => Box::new(NoiseTexture::default()),
        })
    }
}

// pub trait TextureLoader {
//     fn load(self) -> anyhow::Result<Box<dyn Texture + Send + Sync>>;
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureId {
    inner: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureManagerBuilder {
    inner: Vec<TextureLoader>,
}

impl TextureManagerBuilder {
    pub fn finalize(self) -> anyhow::Result<TextureManager> {
        let loaded = self
            .inner
            .into_iter()
            .map(|loader| loader.load())
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(TextureManager { inner: loaded })
    }
}

pub struct TextureManager {
    inner: Vec<Box<dyn Texture + Send + Sync>>,
}

impl TextureManager {
    pub fn resolve(&self, id: TextureId) -> &dyn Texture {
        self.inner[id.inner].as_ref()
    }
}

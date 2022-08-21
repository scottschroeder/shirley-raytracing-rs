use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use super::{
    checker::CheckerTexture,
    image_texture::{earth_builtin, ImageTexture},
    settings::{ColorSetting, ScalarSetting},
    solid::ConstantTexture,
    Texture,
};
use crate::raytracer::{
    core::{Color, Vec3},
    material::perlin::NoiseTexture,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TextureLoader {
    Solid(ColorSetting),
    ImagePath(std::path::PathBuf),
    Perlin(ScalarSetting),
    EarthBuiltin,
    Checker {
        size: ScalarSetting,
        odd: Box<TextureLoader>,
        even: Box<TextureLoader>,
    },
}

impl TextureLoader {
    pub fn solid(r: f64, g: f64, b: f64) -> TextureLoader {
        TextureLoader::Solid(ColorSetting(Color(Vec3::new(r, g, b))))
    }
    pub fn solid_from_vec(vec: Vec3) -> TextureLoader {
        TextureLoader::Solid(ColorSetting(Color(vec)))
    }
    pub fn checker(size: f64, odd: TextureLoader, even: TextureLoader) -> TextureLoader {
        TextureLoader::Checker {
            size: ScalarSetting(size),
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }
    pub fn noise(scalar: f64) -> TextureLoader {
        TextureLoader::Perlin(ScalarSetting(scalar))
    }
    fn load(&self) -> anyhow::Result<Arc<dyn Texture + Send + Sync>> {
        Ok(match self {
            TextureLoader::Solid(c) => Arc::new(ConstantTexture::from(c.0)),
            TextureLoader::ImagePath(p) => Arc::new(ImageTexture::load_from_filename(p)?),
            TextureLoader::EarthBuiltin => Arc::new(earth_builtin()),
            TextureLoader::Perlin(scalar) => Arc::new(NoiseTexture::scale(scalar.0)),
            TextureLoader::Checker { size, odd, even } => {
                let odd = odd.load()?;
                let even = even.load()?;
                let size = size.0;
                Arc::new(CheckerTexture { size, odd, even })
            }
        })
    }
}

pub trait LoadableTexture {
    fn load_texture(
        self,
        manager: &mut TextureManager,
    ) -> anyhow::Result<Arc<dyn Texture + Send + Sync>>;
}

impl LoadableTexture for TextureLoader {
    fn load_texture(
        self,
        manager: &mut TextureManager,
    ) -> anyhow::Result<Arc<dyn Texture + Send + Sync>> {
        manager.load(self)
    }
}

#[derive(Debug, Clone)]
pub enum TextureObject {
    Solid(ConstantTexture),
    Dynamic(std::sync::Arc<dyn Texture + Send + Sync>),
}

#[derive(Debug)]
pub enum TextureReference<'a> {
    Id(TextureId),
    // Constant(TextureObject),
    Ref(&'a dyn Texture),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureId {
    inner: usize,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TextureManagerBuilder {
//     inner: Vec<TextureLoader>,
// }

// impl TextureManagerBuilder {
//     pub fn finalize(self) -> anyhow::Result<TextureManager> {
//         let loaded = self
//             .inner
//             .into_iter()
//             .map(|loader| loader.load())
//             .collect::<anyhow::Result<Vec<_>>>()?;
//         Ok(TextureManager { inner: loaded })
//     }
// }

#[derive(Default)]
pub struct TextureManager {
    inner: HashMap<TextureLoader, Arc<dyn Texture + Send + Sync>>,
}

impl TextureManager {
    pub fn load(
        &mut self,
        loader: TextureLoader,
    ) -> anyhow::Result<Arc<dyn Texture + Send + Sync>> {
        match self.inner.entry(loader) {
            std::collections::hash_map::Entry::Occupied(o) => Ok(o.get().clone()),
            std::collections::hash_map::Entry::Vacant(v) => {
                let t = v.key().load()?;
                v.insert(t.clone());
                Ok(t)
            }
        }
    }
}

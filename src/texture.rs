use std::path::Path;

use image::GenericImageView;
use nalgebra::clamp;

use crate::{
    objects::texture::Texture,
    util::{Color, Vec3},
};

#[derive(Debug)]
pub struct ImageTexture {
    data: image::DynamicImage,
}

impl ImageTexture {
    pub fn load_from_filename<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let img = image::open(path)?;
        Ok(ImageTexture { data: img })
    }

    pub fn load_from_memory(buffer: &[u8]) -> anyhow::Result<Self> {
        let img = image::load_from_memory(buffer)?;
        Ok(ImageTexture { data: img })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &crate::util::Point) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let i = (u * (self.data.width() - 1) as f64) as u32;
        let j = (v * (self.data.height() - 1) as f64) as u32;

        let color_scale = 1.0 / 255.0;

        let pixel = self.data.get_pixel(i, j);

        // return Color(Vec3::new(
        //         u, 0.0, v
        // ));

        Color(Vec3::new(
            pixel.0[0] as f64 * color_scale,
            pixel.0[1] as f64 * color_scale,
            pixel.0[2] as f64 * color_scale,
        ))
    }
}

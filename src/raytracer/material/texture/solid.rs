use crate::{
    core::{Color, Point},
    material::texture::Texture,
};

#[derive(Debug, Clone)]
pub struct ConstantTexture {
    color: Color,
}

impl From<Color> for ConstantTexture {
    fn from(c: Color) -> Self {
        ConstantTexture { color: c }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color
    }
}

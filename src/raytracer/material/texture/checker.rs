use super::Texture;
use crate::raytracer::core::{Color, Point};

#[derive(Debug)]
pub struct CheckerTexture {
    size: f64,
    odd: Box<dyn Texture + Send + Sync>,
    even: Box<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new<T: Texture + Send + Sync + 'static, U: Texture + Send + Sync + 'static>(
        size: f64,
        odd: T,
        even: U,
    ) -> CheckerTexture {
        CheckerTexture {
            size,
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines =
            (self.size * p.0.x()).sin() * (self.size * p.0.y()).sin() * (self.size * p.0.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
use std::sync::Arc;

use super::Texture;
use crate::core::{Color, Point};

#[derive(Debug)]
pub struct CheckerTexture {
    pub size: f64,
    pub odd: Arc<dyn Texture + Send + Sync>,
    pub even: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new<T: Texture + Send + Sync + 'static, U: Texture + Send + Sync + 'static>(
        size: f64,
        odd: T,
        even: U,
    ) -> CheckerTexture {
        CheckerTexture {
            size,
            odd: Arc::new(odd),
            even: Arc::new(even),
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

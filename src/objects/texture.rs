use crate::util::{Color, Point};

pub trait Texture: std::fmt::Debug {
    fn value(&self, u: f64, v: f64, p: Point) -> Color;
}

#[derive(Debug)]
pub struct ConstantTexture {
    color: Color,
}

impl From<Color> for ConstantTexture {
    fn from(c: Color) -> Self {
        ConstantTexture { color: c }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: Point) -> Color {
        self.color
    }
}

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
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let sines =
            (self.size * p.0.x()).sin() * (self.size * p.0.y()).sin() * (self.size * p.0.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

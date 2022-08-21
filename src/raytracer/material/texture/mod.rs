use crate::raytracer::core::{Color, Point};

pub mod checker;
pub mod image_texture;
pub mod loader;
pub mod solid;

pub trait Texture: std::fmt::Debug {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

// impl<S> Texture for std::sync::Arc<S>
// where
//     S: Texture + std::fmt::Debug,
// {
//     fn value(&self, u: f64, v: f64, p: &Point) -> Color {
//         self.as_ref().value(u, v, p)
//     }
// }
// impl<T: AsRef<dyn Texture> + std::fmt::Debug> Texture for T {
//     fn value(&self, u: f64, v: f64, p: &Point) -> Color {
//         self.as_ref().value(u, v, p)
//     }
// }
//
impl<T: Texture> Texture for std::sync::Arc<T>
where
    T: std::fmt::Debug + ?Sized,
{
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        self.as_ref().value(u, v, p)
    }
}

mod settings {
    use std::hash::Hash;

    use serde::{Deserialize, Serialize};

    use crate::raytracer::core::Color;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ColorSetting(pub Color);

    impl Hash for ColorSetting {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            (self.0 .0.x().to_bits()).hash(state);
            (self.0 .0.y().to_bits()).hash(state);
            (self.0 .0.z().to_bits()).hash(state);
        }
    }

    impl PartialEq for ColorSetting {
        fn eq(&self, other: &Self) -> bool {
            let sx = self.0 .0.x().to_bits();
            let sy = self.0 .0.y().to_bits();
            let sz = self.0 .0.z().to_bits();

            let ox = other.0 .0.x().to_bits();
            let oy = other.0 .0.y().to_bits();
            let oz = other.0 .0.z().to_bits();

            sx.eq(&ox) && sy.eq(&oy) && sz.eq(&oz)
        }
    }

    impl Eq for ColorSetting {}

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ScalarSetting(pub f64);

    impl Hash for ScalarSetting {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            (self.0.to_bits()).hash(state);
        }
    }

    impl PartialEq for ScalarSetting {
        fn eq(&self, other: &Self) -> bool {
            (self.0.to_bits()) == (other.0.to_bits())
        }
    }

    impl Eq for ScalarSetting {}
}

use serde::{Deserialize, Serialize};

use crate::raytracer::core::{Color, Ray, Vec3};

fn skybox(r: &Ray) -> Color {
    let unit = r.direction.unit();
    let t = 0.5f64 * (unit.y() + 1f64);
    Color(Vec3::new(1.0, 1.0, 1.0).scale(1f64 - t) + Vec3::new(0.5, 0.7, 1.0).scale(t))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SkyBox {
    Above,
    Flat(Color),
    None,
}

impl SkyBox {
    pub fn background(&self, r: &Ray) -> Color {
        match self {
            SkyBox::Above => skybox(r),
            SkyBox::Flat(c) => *c,
            SkyBox::None => Color(Vec3::default()),
        }
    }
}

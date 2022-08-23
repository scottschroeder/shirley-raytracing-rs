use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{Material, Scatter};
use crate::{
    core::{math::fmin_one, Color, Ray},
    geometry::hittable::HitRecord,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dielectric {
    pub ir: f64,
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

impl Material for Dielectric {
    fn scatter<R: Rng>(&self, rng: &mut R, ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.unit();

        let cos_theta = fmin_one(unit_direction.scale(-1.0).dot(&record.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            unit_direction.reflect(&record.normal)
        } else {
            unit_direction.refract(&record.normal, refraction_ratio)
        };

        Some(Scatter {
            direction: Ray {
                orig: record.point,
                direction,
            },
            attenuation: Color::ones(),
        })
    }
}

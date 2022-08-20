use crate::raytracer::{core::{Color, Ray, math::random_in_unit_sphere}, geometry::hittable::HitRecord};

use super::{Material, Scatter};

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Option<f64>) -> Metal {
        let mut fuzz = fuzz.unwrap_or(0.0);
        if fuzz > 1.0 {
            fuzz = 1.0;
        }
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let reflected = ray.direction.unit().reflect(&record.normal);

        let direction = Ray {
            orig: record.point,
            direction: reflected + random_in_unit_sphere().scale(self.fuzz),
        };

        Some(Scatter {
            direction,
            attenuation: self.albedo,
        })
    }
}

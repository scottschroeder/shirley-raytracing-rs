use super::texture::Texture;
use crate::{
    objects::hittable::HitRecord,
    util::{
        math::{fmin_one, random_in_unit_sphere, random_unit_vector},
        Color, Ray,
    },
};

#[derive(Debug, Clone)]
pub struct Scatter {
    pub direction: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<Scatter>;
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: std::sync::Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new<T: Texture + Send + Sync + 'static>(texture: T) -> Lambertian {
        Lambertian {
            albedo: std::sync::Arc::new(texture),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let mut scatter = record.normal + random_unit_vector();
        if scatter.near_zero() {
            scatter = record.normal;
        }
        let direction = Ray {
            orig: record.point,
            direction: scatter,
        };

        Some(Scatter {
            direction,
            attenuation: self.albedo.value(record.u, record.v, record.point),
        })
    }
}

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

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub ir: f64,
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        use rand::prelude::*;
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.unit();

        let cos_theta = fmin_one(unit_direction.scale(-1.0).dot(&record.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || reflectance(cos_theta, refraction_ratio) > thread_rng().gen::<f64>()
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

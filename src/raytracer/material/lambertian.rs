use super::{texture::Texture, Material, Scatter};
use crate::raytracer::{
    core::{math::random_unit_vector, Ray},
    geometry::hittable::HitRecord,
};

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
            attenuation: self.albedo.value(record.u, record.v, &record.point),
        })
    }
}

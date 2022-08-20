use crate::raytracer::{
    core::{math::random_unit_vector, Color, Ray},
    geometry::hittable::HitRecord,
};

use super::{texture::Texture, Material, Scatter};

pub struct DiffuseLight {
    pub albedo: std::sync::Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new<T: Texture + Send + Sync + 'static>(texture: T) -> DiffuseLight {
        DiffuseLight {
            albedo: std::sync::Arc::new(texture),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord) -> Option<Scatter> {
        None
    }

    fn emitted(&self, _ray: &Ray, record: &HitRecord) -> Option<Color> {
        Some(self.albedo.value(record.u, record.v, &record.point))
    }

    // fn emitted(&self, u: f64, v: f64, point: &crate::util::Point) -> Option<crate::util::Color> {
    // }
}

pub struct FairyLight {
    pub albedo: std::sync::Arc<dyn Texture + Send + Sync>,
}

impl FairyLight {
    pub fn new<T: Texture + Send + Sync + 'static>(texture: T) -> FairyLight {
        FairyLight {
            albedo: std::sync::Arc::new(texture),
        }
    }
}

impl Material for FairyLight {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let mut scatter = record.normal + random_unit_vector();
        if scatter.near_zero() {
            scatter = record.normal;
        }
        let direction = Ray {
            orig: record.point,
            direction: scatter,
        };
        let attenuation = self.albedo.value(record.u, record.v, &record.point);
        Some(Scatter {
            direction,
            attenuation: Color(attenuation.0.unit()),
        })
    }

    fn emitted(&self, ray: &Ray, record: &HitRecord) -> Option<Color> {
        let src_color = self.albedo.value(record.u, record.v, &record.point);

        let scale = record.normal.dot(&ray.direction.scale(-1.0));
        log::trace!("scale: {}", scale);
        Some(Color(src_color.0.scale(scale / ray.direction.length())))
        // Some(src_color)
    }
}

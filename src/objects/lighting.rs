use super::{material::Material, texture::Texture};

struct DiffuseLight {
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
    fn scatter(
        &self,
        _ray: &crate::util::Ray,
        _record: &super::hittable::HitRecord,
    ) -> Option<super::material::Scatter> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &crate::util::Point) -> Option<crate::util::Color> {
        Some(self.albedo.value(u, v, point))
    }
}

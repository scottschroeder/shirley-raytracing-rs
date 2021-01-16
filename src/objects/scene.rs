use crate::objects::Geometry;

use super::hittable::HitRecord;
pub struct SceneObject {
    geometry: Box<dyn Geometry + Sync>,
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<SceneObject>,
}
impl Scene {
    pub fn add<G: Geometry + 'static + Sync>(&mut self, g: G) {
        let obj = SceneObject {
            geometry: Box::new(g),
        };
        self.objects.push(obj);
    }
}

impl Geometry for Scene {
    fn hit(
        &self,
        ray: &crate::util::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<super::hittable::HitRecord> {
        let mut closest = None;

        for obj in &self.objects {
            let t_closest = closest.as_ref().map(|x: &HitRecord| x.t).unwrap_or(t_max);
            if let Some(hit) = obj.geometry.hit(ray, t_min, t_closest) {
                closest = Some(hit)
            }
        }

        closest
    }
}

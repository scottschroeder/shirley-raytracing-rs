use super::hittable::HitRecord;
use crate::objects::{material::Material, Geometry, Hittable};

pub struct SceneObject {
    geometry: Box<dyn Geometry + Sync>,
    pub material: Box<dyn Material + Sync>,
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<SceneObject>,
}
impl Scene {
    pub fn add<G: Geometry + 'static + Sync, M: Material + 'static + Sync>(&mut self, g: G, m: M) {
        let obj = SceneObject {
            geometry: Box::new(g),
            material: Box::new(m),
        };
        self.objects.push(obj);
    }
}

impl Hittable for Scene {
    type Leaf = SceneObject;
    fn hit(
        &self,
        ray: &crate::util::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<(&SceneObject, HitRecord)> {
        let mut closest: Option<(&SceneObject, HitRecord)> = None;

        for obj in &self.objects {
            let t_closest = if let Some(ref x) = closest {
                x.1.t
            } else {
                t_max
            };
            if let Some(hit) = obj.geometry.hit(ray, t_min, t_closest) {
                closest = Some((obj, hit))
            }
        }

        closest
    }
}

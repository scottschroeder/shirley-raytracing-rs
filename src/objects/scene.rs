use super::{bbox_tree::BboxTree, hittable::HitRecord};
use crate::objects::{material::Material, Geometry, Hittable};

pub struct SceneObject {
    geometry: Box<dyn Geometry + Sync>,
    pub material: Box<dyn Material + Sync>,
}

impl Geometry for SceneObject {
    fn hit(&self, ray: &crate::util::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.geometry.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<super::Aabb> {
        self.geometry.bounding_box()
    }
}

#[derive(Default)]
pub struct SceneBuilder {
    objects: Vec<SceneObject>,
    bounded_objects: Vec<SceneObject>,
}

impl SceneBuilder {
    pub fn add<G: Geometry + 'static + Sync, M: Material + 'static + Sync>(&mut self, g: G, m: M) {
        let obj = SceneObject {
            geometry: Box::new(g),
            material: Box::new(m),
        };
        if obj.bounding_box().is_some() {
            self.bounded_objects.push(obj)
        } else {
            self.objects.push(obj);
        }
    }
    pub fn finalize_without_tree(mut self) -> Scene {
        self.objects.extend(self.bounded_objects);
        Scene {
            objects: self.objects,
            tree: BboxTree::default(),
        }
    }
    pub fn finalize(self) -> Scene {
        let tree = BboxTree::new(self.bounded_objects);
        Scene {
            objects: self.objects,
            tree,
        }
    }
}

pub struct Scene {
    objects: Vec<SceneObject>,
    tree: BboxTree<SceneObject>,
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
            let t_closest = closest.as_ref().map(|(_, r)| r.t).unwrap_or(t_max);
            if let Some(hit) = obj.geometry.hit(ray, t_min, t_closest) {
                closest = Some((obj, hit))
            }
        }
        let t_closest = closest.as_ref().map(|(_, r)| r.t).unwrap_or(t_max);

        let new_closest = self.tree.hit(ray, t_min, t_closest);

        new_closest.or(closest)
    }
}

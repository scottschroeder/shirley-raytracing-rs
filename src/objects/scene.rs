use super::{bbox_tree::BboxTree, hittable::HitRecord, skybox::SkyBox};
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

impl<'a> Geometry for &'a SceneObject {
    fn hit(&self, ray: &crate::util::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (*self).hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<super::Aabb> {
        (*self).bounding_box()
    }
}

pub struct HitList<T> {
    objects: Vec<T>,
}

impl<T> Default for HitList<T> {
    fn default() -> Self {
        Self {
            objects: Default::default(),
        }
    }
}

impl<T> Hittable for HitList<T>
where
    for<'a> &'a T: Geometry,
{
    type Leaf = T;
    fn hit(&self, ray: &crate::util::Ray, t_min: f64, t_max: f64) -> Option<(&T, HitRecord)> {
        let mut closest: Option<(&T, HitRecord)> = None;

        for obj in &self.objects {
            let t_closest = closest.as_ref().map(|(_, r)| r.t).unwrap_or(t_max);
            if let Some(hit) = obj.hit(ray, t_min, t_closest) {
                closest = Some((obj, hit))
            }
        }
        closest
    }
}

pub struct SceneBuilder {
    skybox: SkyBox,
    objects: HitList<SceneObject>,
    bounded_objects: Vec<SceneObject>,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self {
            skybox: SkyBox::Above,
            objects: Default::default(),
            bounded_objects: Default::default(),
        }
    }
}

impl SceneBuilder {
    pub fn set_skybox(&mut self, skybox: SkyBox) -> &mut Self {
        self.skybox = skybox;
        self
    }

    pub fn add<G: Geometry + 'static + Sync, M: Material + 'static + Sync>(&mut self, g: G, m: M) {
        let obj = SceneObject {
            geometry: Box::new(g),
            material: Box::new(m),
        };
        if obj.bounding_box().is_some() {
            self.bounded_objects.push(obj)
        } else {
            self.objects.objects.push(obj);
        }
    }
    pub fn finalize_without_tree(mut self) -> Scene {
        self.objects.objects.extend(self.bounded_objects);
        Scene {
            skybox: self.skybox,
            objects: self.objects,
            tree: BboxTree::default(),
        }
    }
    pub fn finalize(self) -> Scene {
        let tree = BboxTree::new(self.bounded_objects);
        Scene {
            skybox: self.skybox,
            objects: self.objects,
            tree,
        }
    }
}

pub struct Scene {
    pub skybox: SkyBox,
    objects: HitList<SceneObject>,
    tree: BboxTree<SceneObject>,
}

pub struct WorkspaceScene<'a, 'b> {
    objects: &'a HitList<SceneObject>,
    tree: &'a BboxTree<SceneObject>,
    stack: &'b mut Vec<usize>,
}

impl<'a, 'b> WorkspaceScene<'a, 'b> {
    pub fn hit_workspace(
        &mut self,
        ray: &crate::util::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<(&SceneObject, HitRecord)> {
        let closest = self.objects.hit(ray, t_min, t_max);
        let t_closest = closest.as_ref().map(|(_, r)| r.t).unwrap_or(t_max);
        let new_closest = self.tree.hit_workspace(self.stack, ray, t_min, t_closest);
        new_closest.or(closest)
    }
}

impl Scene {
    pub fn add<G: Geometry + 'static + Sync, M: Material + 'static + Sync>(&mut self, g: G, m: M) {
        let obj = SceneObject {
            geometry: Box::new(g),
            material: Box::new(m),
        };
        self.objects.objects.push(obj);
    }

    pub fn workspace_scene<'a, 'b>(
        &'a self,
        hit_stack: &'b mut Vec<usize>,
    ) -> WorkspaceScene<'a, 'b> {
        WorkspaceScene {
            objects: &self.objects,
            tree: &self.tree,
            stack: hit_stack,
        }
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
        let closest = self.objects.hit(ray, t_min, t_max);
        let t_closest = closest.as_ref().map(|(_, r)| r.t).unwrap_or(t_max);
        let new_closest = self.tree.hit(ray, t_min, t_closest);

        new_closest.or(closest)
    }
}

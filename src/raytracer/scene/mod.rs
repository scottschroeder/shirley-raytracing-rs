use serde::{Deserialize, Serialize};

use super::{
    bvh::{aabb::Aabb, bbox_tree::BboxTree},
    core::Ray,
    geometry::{
        hittable::{Geometry, HitRecord, Hittable},
        object::GeometricObject,
    },
    material::{
        material_type::{MaterialType, SceneMaterial},
        texture::loader::{TextureLoader, TextureManager},
    },
    skybox::SkyBox,
};
use crate::bvh::bbox_tree::BboxTreeWorkspace;

pub struct SceneObject {
    geometry: GeometricObject,
    pub material: SceneMaterial,
}

#[derive(Serialize, Deserialize)]
pub struct SceneLoadObject {
    geometry: GeometricObject,
    material: MaterialType<TextureLoader>,
}

impl Geometry for SceneObject {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.geometry.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        self.geometry.bounding_box()
    }
}

impl<'a> Geometry for &'a SceneObject {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (*self).hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<Aabb> {
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(&T, HitRecord)> {
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

#[derive(Serialize, Deserialize)]
pub struct SceneBuilder {
    skybox: SkyBox,
    objects: Vec<SceneLoadObject>,
}

impl Default for SceneBuilder {
    fn default() -> Self {
        Self {
            skybox: SkyBox::Above,
            objects: Default::default(),
        }
    }
}

impl SceneBuilder {
    pub fn set_skybox(&mut self, skybox: SkyBox) -> &mut Self {
        self.skybox = skybox;
        self
    }

    pub fn add<G: Into<GeometricObject>, M: Into<MaterialType<TextureLoader>>>(
        &mut self,
        g: G,
        m: M,
    ) {
        let obj = SceneLoadObject {
            geometry: g.into(),
            material: m.into(),
        };
        self.objects.push(obj);
    }
    pub fn finalize(self) -> anyhow::Result<Scene> {
        let mut bounded_objects = Vec::new();
        let mut unbounded_objects = HitList::default();

        let mut texture_manager = TextureManager::default();

        for load_obj in self.objects {
            let loaded_material = load_obj.material.load_texture(&mut texture_manager)?;
            let scene_obj = SceneObject {
                geometry: load_obj.geometry,
                material: loaded_material,
            };

            if scene_obj.bounding_box().is_some() {
                bounded_objects.push(scene_obj);
            } else {
                unbounded_objects.objects.push(scene_obj);
            }
        }

        let tree = BboxTree::new(bounded_objects);
        Ok(Scene {
            skybox: self.skybox,
            objects: unbounded_objects,
            tree,
        })
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
    stack: &'b mut BboxTreeWorkspace,
}

impl<'a, 'b> WorkspaceScene<'a, 'b> {
    pub fn hit_workspace(
        &mut self,
        ray: &Ray,
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
    pub fn workspace_scene<'a, 'b>(
        &'a self,
        hit_stack: &'b mut BboxTreeWorkspace,
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(&SceneObject, HitRecord)> {
        let closest = self.objects.hit(ray, t_min, t_max);
        let t_closest = closest.as_ref().map(|(_, r)| r.t).unwrap_or(t_max);
        let mut stack = BboxTreeWorkspace::default();
        log::warn!("creating new workspace stack, this should be done in the caller");
        let new_closest = self.tree.hit_workspace(&mut stack, ray, t_min, t_closest);

        new_closest.or(closest)
    }
}

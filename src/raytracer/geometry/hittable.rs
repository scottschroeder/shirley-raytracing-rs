use crate::{
    bvh::aabb::Aabb,
    core::{Point, Ray, Vec3},
};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(
        incoming: &Ray,
        point: Point,
        mut normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
    ) -> HitRecord {
        let front_face = incoming.direction.dot(&normal) < 0.0;
        if !front_face {
            normal.scale_mut(-1.0);
        }
        HitRecord {
            point,
            normal,
            t,
            front_face,
            u,
            v,
        }
    }
}

pub trait Geometry {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<Aabb>;
}

pub trait Hittable {
    type Leaf;
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(&Self::Leaf, HitRecord)>;
}

use crate::util::{Point, Ray, Vec3};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(incoming: &Ray, point: Point, mut normal: Vec3, t: f64) -> HitRecord {
        let front_face = incoming.direction.dot(&normal) < 0.0;
        if !front_face {
            normal.scale_mut(-1.0);
        }
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Geometry {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub trait Hittable {
    type Leaf;
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(&Self::Leaf, HitRecord)>;
}

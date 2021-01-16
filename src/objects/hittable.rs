use crate::util::{Point, Ray, Vec3};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

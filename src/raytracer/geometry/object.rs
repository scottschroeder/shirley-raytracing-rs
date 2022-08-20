use serde::{Deserialize, Serialize};

use super::{
    hittable::Geometry,
    rect::{RectBox, RectXY, RectXZ, RectYZ},
    sphere::Sphere,
};

#[derive(Serialize, Deserialize)]
pub enum GeometricObject {
    Sphere(Sphere),
    RectXY(RectXY),
    RectYZ(RectYZ),
    RectXZ(RectXZ),
    RectBox(RectBox),
}

impl From<Sphere> for GeometricObject {
    fn from(s: Sphere) -> Self {
        GeometricObject::Sphere(s)
    }
}
impl From<RectXY> for GeometricObject {
    fn from(s: RectXY) -> Self {
        GeometricObject::RectXY(s)
    }
}
impl From<RectYZ> for GeometricObject {
    fn from(s: RectYZ) -> Self {
        GeometricObject::RectYZ(s)
    }
}
impl From<RectXZ> for GeometricObject {
    fn from(s: RectXZ) -> Self {
        GeometricObject::RectXZ(s)
    }
}
impl From<RectBox> for GeometricObject {
    fn from(s: RectBox) -> Self {
        GeometricObject::RectBox(s)
    }
}

impl Geometry for GeometricObject {
    fn hit(
        &self,
        ray: &crate::raytracer::core::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<super::hittable::HitRecord> {
        match self {
            GeometricObject::Sphere(x) => x.hit(ray, t_min, t_max),
            GeometricObject::RectXY(x) => x.hit(ray, t_min, t_max),
            GeometricObject::RectYZ(x) => x.hit(ray, t_min, t_max),
            GeometricObject::RectXZ(x) => x.hit(ray, t_min, t_max),
            GeometricObject::RectBox(x) => x.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self) -> Option<crate::raytracer::bvh::aabb::Aabb> {
        match self {
            GeometricObject::Sphere(x) => x.bounding_box(),
            GeometricObject::RectXY(x) => x.bounding_box(),
            GeometricObject::RectYZ(x) => x.bounding_box(),
            GeometricObject::RectXZ(x) => x.bounding_box(),
            GeometricObject::RectBox(x) => x.bounding_box(),
        }
    }
}

use super::{hittable::HitRecord, Aabb, Geometry};
use crate::util::{Point, Vec3};

const BBOX_WIDTH: f64 = 0.0001;

pub fn xy_rect(d1_min: f64, d1_max: f64, d2_min: f64, d2_max: f64, offset: f64) -> Rect<0, 1> {
    Rect {
        d1_min,
        d1_max,
        d2_min,
        d2_max,
        offset,
    }
}

pub fn yz_rect(d1_min: f64, d1_max: f64, d2_min: f64, d2_max: f64, offset: f64) -> Rect<1, 2> {
    Rect {
        d1_min,
        d1_max,
        d2_min,
        d2_max,
        offset,
    }
}

pub fn xz_rect(d1_min: f64, d1_max: f64, d2_min: f64, d2_max: f64, offset: f64) -> Rect<0, 2> {
    Rect {
        d1_min,
        d1_max,
        d2_min,
        d2_max,
        offset,
    }
}

pub struct Rect<const D1: usize, const D2: usize> {
    d1_min: f64,
    d1_max: f64,
    d2_min: f64,
    d2_max: f64,
    offset: f64,
}

impl<const D1: usize, const D2: usize> Geometry for Rect<D1, D2> {
    fn hit(&self, ray: &crate::util::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let D3 = 3 - D1 - D2;
        let t = (self.offset - ray.orig.0[D3]) / ray.direction[D3];
        if t < t_min || t > t_max {
            return None;
        }
        let d1_value = ray.orig.0[D1] + t * ray.direction[D1];
        let d2_value = ray.orig.0[D2] + t * ray.direction[D2];

        if d1_value < self.d1_min
            || d1_value > self.d1_max
            || d2_value < self.d2_min
            || d2_value > self.d2_max
        {
            return None;
        }
        let u = (d1_value - self.d1_min) / (self.d1_max - self.d1_min);
        let v = (d2_value - self.d2_min) / (self.d2_max - self.d2_min);
        let root = t;

        let mut normal = Vec3::default();
        normal[D3] = 1.0;

        let point = ray.at(t);
        Some(HitRecord::new(ray, point, normal, root, u, v))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let D3 = 3 - D1 - D2;
        let mut min = Vec3::default();
        let mut max = Vec3::default();

        min[D1] = self.d1_min;
        min[D2] = self.d2_min;
        min[D3] = self.offset - BBOX_WIDTH;

        max[D1] = self.d1_max;
        max[D2] = self.d2_max;
        max[D3] = self.offset + BBOX_WIDTH;

        Some(Aabb {
            min: Point(min),
            max: Point(max),
        })
    }
}

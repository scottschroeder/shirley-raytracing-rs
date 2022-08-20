use super::{hittable::HitRecord, Aabb, Geometry};
use crate::util::{Point, Ray, Vec3};

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

pub struct RectBox {
    min: Point,
    max: Point,
    xy_sides: [Rect<0, 1>; 2],
    yz_sides: [Rect<1, 2>; 2],
    xz_sides: [Rect<0, 2>; 2],
}

impl RectBox {
    pub fn new(p0: Point, p1: Point) -> RectBox {
        RectBox {
            min: p0,
            max: p1,
            xy_sides: [
                xy_rect(p0.0.x(), p1.0.x(), p0.0.y(), p1.0.y(), p1.0.z()),
                xy_rect(p0.0.x(), p1.0.x(), p0.0.y(), p1.0.y(), p0.0.z()),
            ],
            yz_sides: [
                yz_rect(p0.0.y(), p1.0.y(), p0.0.z(), p1.0.z(), p1.0.x()),
                yz_rect(p0.0.y(), p1.0.y(), p0.0.z(), p1.0.z(), p0.0.x()),
            ],
            xz_sides: [
                xz_rect(p0.0.x(), p1.0.x(), p0.0.z(), p1.0.z(), p1.0.y()),
                xz_rect(p0.0.x(), p1.0.x(), p0.0.z(), p1.0.z(), p0.0.y()),
            ],
        }
    }
}

fn check_closer<const D1: usize, const D2: usize>(
    ray: &Ray,
    t_min: f64,
    t_max: f64,
    current: &mut Option<HitRecord>,
    r: &Rect<D1, D2>,
) {
    let t_closest = current.as_ref().map(|r| r.t).unwrap_or(t_max);
    if let Some(hit) = r.hit(ray, t_min, t_closest) {
        let mut new_hit = Some(hit);
        std::mem::swap(current, &mut new_hit)
    }
}

impl Geometry for RectBox {
    fn hit(&self, ray: &crate::util::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;
        check_closer(ray, t_min, t_max, &mut closest, &self.xy_sides[0]);
        check_closer(ray, t_min, t_max, &mut closest, &self.xy_sides[1]);
        check_closer(ray, t_min, t_max, &mut closest, &self.yz_sides[0]);
        check_closer(ray, t_min, t_max, &mut closest, &self.yz_sides[1]);
        check_closer(ray, t_min, t_max, &mut closest, &self.xz_sides[0]);
        check_closer(ray, t_min, t_max, &mut closest, &self.xz_sides[1]);
        closest
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            min: self.min,
            max: self.max,
        })
    }
}

use std::cmp::Ordering;

use crate::util::{Point, Ray, Vec3, EACH_DIMM};

fn fmin(a: f64, b: f64) -> f64 {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) => a,
        Some(_) => b,
        None => non_nan(a, b),
    }
}
fn fmax(a: f64, b: f64) -> f64 {
    match a.partial_cmp(&b) {
        Some(Ordering::Greater) => a,
        Some(_) => b,
        None => non_nan(a, b),
    }
}

#[inline]
fn non_nan(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        b
    } else {
        a
    }
}

pub fn surrounding_box(lhs: &Aabb, rhs: &Aabb) -> Aabb {
    let small = Vec3::new(
        fmin(lhs.min.0.x(), rhs.min.0.x()),
        fmin(lhs.min.0.y(), rhs.min.0.y()),
        fmin(lhs.min.0.z(), rhs.min.0.z()),
    );
    let large = Vec3::new(
        fmax(lhs.max.0.x(), rhs.max.0.x()),
        fmax(lhs.max.0.y(), rhs.max.0.y()),
        fmax(lhs.max.0.z(), rhs.max.0.z()),
    );
    Aabb {
        min: Point(small),
        max: Point(large),
    }
}

pub struct Aabb {
    pub min: Point,
    pub max: Point,
}

impl Aabb {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in EACH_DIMM.iter().cloned() {
            let t0 = fmin(
                (self.min.0[a] - r.orig.0[a]) / r.direction[a],
                (self.max.0[a] - r.orig.0[a]) / r.direction[a],
            );
            let t1 = fmax(
                (self.min.0[a] - r.orig.0[a]) / r.direction[a],
                (self.max.0[a] - r.orig.0[a]) / r.direction[a],
            );
            t_min = fmax(t0, t_min);
            t_max = fmin(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    pub fn hit2(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for a in EACH_DIMM.iter().cloned() {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min.0[a] - r.orig.0[a]) * inv_d;
            let mut t1 = (self.max.0[a] - r.orig.0[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1)
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 > t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

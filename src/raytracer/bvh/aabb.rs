use crate::core::{
    fp::{fmax, fmin},
    Point, Ray, Vec3, EACH_DIMM,
};

pub fn bounding<'a>(iter: impl Iterator<Item = &'a Aabb>) -> Option<Aabb> {
    let mut out = None;

    for b in iter {
        out = Some(match &out {
            Some(p) => surrounding_box(p, b),
            None => b.clone(),
        })
    }
    out
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

#[derive(Debug, Clone, PartialEq)]
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
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn area(&self) -> f64 {
        let x = self.max.0.x() - self.min.0.x();
        let y = self.max.0.y() - self.min.0.y();
        let z = self.max.0.z() - self.min.0.z();
        x * y * z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_two_identical_boxes() {
        let b1 = Aabb {
            min: Point(Vec3::new(0.0, 0.0, 0.0)),
            max: Point(Vec3::new(1.0, 1.0, 1.0)),
        };
        let r = surrounding_box(&b1, &b1);
        assert_eq!(r, b1);
    }

    #[test]
    fn combine_two_overlapping_boxes() {
        let b1 = Aabb {
            min: Point(Vec3::new(-0.5, -0.5, -0.5)),
            max: Point(Vec3::new(1.0, 1.0, 1.0)),
        };
        let b2 = Aabb {
            min: Point(Vec3::new(0.0, 0.0, 0.0)),
            max: Point(Vec3::new(2.0, 2.0, 2.0)),
        };
        let r = surrounding_box(&b1, &b2);
        let expected = Aabb {
            min: Point(Vec3::new(-0.5, -0.5, -0.5)),
            max: Point(Vec3::new(2.0, 2.0, 2.0)),
        };
        assert_eq!(r, expected);
    }

    #[test]
    fn combine_fully_contained_box() {
        let b1 = Aabb {
            min: Point(Vec3::new(0.5, 0.5, 0.5)),
            max: Point(Vec3::new(1.0, 1.0, 1.0)),
        };
        let b2 = Aabb {
            min: Point(Vec3::new(0.0, 0.0, 0.0)),
            max: Point(Vec3::new(2.0, 2.0, 2.0)),
        };
        let r = surrounding_box(&b1, &b2);
        assert_eq!(r, b2);
    }

    #[test]
    fn check_hit() {
        let b1 = Aabb {
            min: Point(Vec3::new(1.0, -1.0, -1.0)),
            max: Point(Vec3::new(2.0, 1.0, 1.0)),
        };
        let r = Ray::new(Point(Vec3::new(0.0, 0.0, 0.0)), Vec3::new(1.0, 0.0, 0.0));
        // assert!(b1.hit(&r, 0.0, f64::MAX));
        assert!(b1.hit2(&r, 0.0, f64::MAX));
    }

    #[test]
    fn check_miss() {
        let b1 = Aabb {
            min: Point(Vec3::new(1.0, -1.0, -1.0)),
            max: Point(Vec3::new(2.0, 1.0, 1.0)),
        };
        let r = Ray::new(Point(Vec3::new(0.0, 2.0, 2.0)), Vec3::new(1.0, 0.0, 0.0));
        // assert!(!b1.hit(&r, 0.0, f64::MAX));
        assert!(!b1.hit2(&r, 0.0, f64::MAX));
    }

    #[test]
    fn check_graze() {
        let b1 = Aabb {
            min: Point(Vec3::new(1.0, -1.0, -1.0)),
            max: Point(Vec3::new(2.0, 1.0, 1.0)),
        };
        let r = Ray::new(Point(Vec3::new(0.0, 1.0, 1.0)), Vec3::new(1.0, 0.0, 0.0));
        // assert!(b1.hit(&r, 0.0, f64::MAX));
        assert!(b1.hit2(&r, 0.0, f64::MAX));
    }

    #[test]
    fn check_graze_corner() {
        let _b1 = Aabb {
            min: Point(Vec3::new(1.0001, -1.0, -1.0)),
            max: Point(Vec3::new(2.0, 1.0001, 1.0001)),
        };
        let _r = Ray::new(Point(Vec3::new(0.0, 0.0, 0.0)), Vec3::new(1.00, 1.0, 1.0));
        // TODO should point grazing work?
        // assert!(b1.hit(&r, 0.0, f64::MAX));
        // assert!(b1.hit2(&r, 0.0, f64::MAX));
    }
}

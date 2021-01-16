use crate::util::{Point, Ray};

use super::{hittable::HitRecord, Hittable};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<super::hittable::HitRecord> {
        let oc = ray.orig.0 - self.center.0;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let point = ray.at(root);

        Some(HitRecord {
            t: root,
            point,
            normal: (point.0 - self.center.0).scale(1.0 / self.radius),
        })
    }
}

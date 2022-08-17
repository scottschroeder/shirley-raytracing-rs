use std::f64::consts::PI;

use super::{hittable::HitRecord, Aabb, Geometry};
use crate::util::{Point, Ray, Vec3};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    fn get_uv(&self, point: &Point) -> (f64, f64) {
        let theta = (-point.0.y()).acos();
        let phi = (-point.0.z()).atan2(point.0.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Geometry for Sphere {
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
        let normal = (point.0 - self.center.0).scale(1.0 / self.radius);
        let (u, v) = self.get_uv(&Point(normal));

        Some(HitRecord::new(ray, point, normal, root, u, v))
    }

    fn bounding_box(&self) -> Option<super::Aabb> {
        let r = Vec3::new(self.radius, self.radius, self.radius);
        Some(Aabb {
            min: Point(self.center.0 - r),
            max: Point(self.center.0 + r),
        })
    }
}

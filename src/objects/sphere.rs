use crate::util::{Point, Ray};

pub fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.orig.0 - center.0;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
        Some((-half_b - discriminant.sqrt()) / a)
    } else {
        None
    }
}

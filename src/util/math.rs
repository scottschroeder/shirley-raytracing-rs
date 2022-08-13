use rand::prelude::{Rng, ThreadRng};

use super::fp::fmin;
use crate::util::Vec3;
pub type Real = f64;

#[inline]
pub fn fmin_one(var: f64) -> f64 {
    fmin(var, 1.0)
}

#[inline]
pub fn random_real(rng: &mut ThreadRng, min: Real, max: Real) -> Real {
    min + (max - min) * rng.gen::<Real>()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() <= 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    let mut v = random_in_unit_sphere();
    v.unit_mut();
    v
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            random_real(&mut rng, -1.0, 1.0),
            random_real(&mut rng, -1.0, 1.0),
            0.0,
        );
        if p.length_squared() <= 1.0 {
            return p;
        }
    }
}

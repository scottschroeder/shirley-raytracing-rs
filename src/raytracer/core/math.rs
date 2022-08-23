use rand::prelude::{Distribution, Rng};

use super::fp::fmin;
use crate::core::Vec3;
pub type Real = f64;

#[inline]
fn convert_spherical_to_cartesian(r: f64, theta: f64, phi: f64) -> Vec3 {
    let sin_phi = phi.sin();
    Vec3::new(
        r * sin_phi * theta.cos(),
        r * phi.cos(),
        r * sin_phi * theta.sin(),
    )
}

#[inline]
pub fn fmin_one(var: f64) -> f64 {
    fmin(var, 1.0)
}

#[inline]
pub fn random_real<R: Rng>(rng: &mut R, min: Real, max: Real) -> Real {
    min + (max - min) * rng.gen::<Real>()
}

#[inline]
pub fn random_int<R: Rng>(rng: &mut R, min: i64, max: i64) -> i64 {
    rng.gen_range(min..max)
}

#[inline]
pub fn random_in_unit_sphere_guess_and_check<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::random_range_with_rng(rng, -1.0, 1.0);
        if p.length_squared() <= 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    random_in_unit_sphere_guess_and_check(rng)
}

#[inline]
pub fn random_on_unit_sphere_distribution<R: Rng>(rng: &mut R) -> Vec3 {
    let uniform = rand::distributions::Uniform::new(-1., 1.);
    loop {
        let (x1, x2) = (uniform.sample(rng), uniform.sample(rng));
        let sum: f64 = x1 * x1 + x2 * x2;
        if sum >= 1.0 {
            continue;
        }
        let factor = 2. * (1. - sum).sqrt();
        return Vec3::new(x1 * factor, 1. - 2. * sum, x2 * factor);
    }
}

#[inline]
pub fn random_unit_vector<R: Rng>(rng: &mut R) -> Vec3 {
    let mut v = random_in_unit_sphere(rng);
    v.unit_mut();
    v
    // TODO replace with this
    // random_on_unit_sphere_distribution(rng)
}

pub fn random_in_unit_disk<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(
            random_real(rng, -1.0, 1.0),
            random_real(rng, -1.0, 1.0),
            0.0,
        );
        if p.length_squared() <= 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk_non_vector<R: Rng>(rng: &mut R) -> Vec3 {
    let uniform = rand::distributions::Uniform::new(-1., 1.);
    loop {
        let (x1, x2) = (uniform.sample(rng), uniform.sample(rng));
        let sum: f64 = x1 * x1 + x2 * x2;
        if sum >= 1.0 {
            continue;
        }
        return Vec3::new(x1, x2, 0.);
    }
}

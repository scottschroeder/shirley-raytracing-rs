use std::ops;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::core::math::{fmin_one, random_real, Real};

const NEAR_ZERO: f64 = 1e-8;

pub const EACH_DIMM: [Dimm; 3] = [Dimm::X, Dimm::Y, Dimm::Z];

#[derive(Debug, Clone, Copy)]
pub enum Dimm {
    X,
    Y,
    Z,
}

impl Dimm {
    pub const fn dimm_num(self) -> usize {
        match self {
            Dimm::X => 0,
            Dimm::Y => 1,
            Dimm::Z => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Vec3 {
    vec: nalgebra::Vector3<Real>,
}

impl rand::distributions::Distribution<Vec3> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3::new(rng.gen::<Real>(), rng.gen::<Real>(), rng.gen::<Real>())
    }
}

impl ops::Index<Dimm> for Vec3 {
    type Output = Real;

    #[inline]
    fn index(&self, index: Dimm) -> &Self::Output {
        match index {
            Dimm::X => &self.vec.x,
            Dimm::Y => &self.vec.y,
            Dimm::Z => &self.vec.z,
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = Real;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.vec.x,
            1 => &self.vec.y,
            2 => &self.vec.z,
            _ => panic!("vec3 does not support dimmensions larger than 2"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.vec.x,
            1 => &mut self.vec.y,
            2 => &mut self.vec.z,
            _ => panic!("vec3 does not support dimmensions larger than 2"),
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    #[inline]
    pub fn new(x: Real, y: Real, z: Real) -> Vec3 {
        Vec3 {
            vec: nalgebra::Vector3::new(x, y, z),
        }
    }
    #[inline]
    pub fn x(&self) -> Real {
        self.vec.x
    }
    #[inline]
    pub fn y(&self) -> Real {
        self.vec.y
    }
    #[inline]
    pub fn z(&self) -> Real {
        self.vec.z
    }

    pub fn random_range_with_rng<R: Rng>(rng: &mut R, min: Real, max: Real) -> Vec3 {
        Vec3::new(
            random_real(rng, min, max),
            random_real(rng, min, max),
            random_real(rng, min, max),
        )
    }

    #[inline]
    pub fn length(&self) -> Real {
        self.vec.norm()
    }

    #[inline]
    pub fn length_squared(&self) -> Real {
        self.vec.norm_squared()
    }

    #[inline]
    pub fn scale(&self, s: Real) -> Vec3 {
        Vec3 {
            vec: self.vec.scale(s),
        }
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        self.vec.x.abs() < NEAR_ZERO && self.vec.y.abs() < NEAR_ZERO && self.vec.z.abs() < NEAR_ZERO
    }

    #[inline]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - normal.scale(2.0 * self.dot(normal))
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: Real) -> Vec3 {
        let cos_theta = fmin_one(self.scale(-1.0).dot(normal));
        let r_out_perp = (normal.scale(cos_theta) + *self).scale(etai_over_etat);
        let r_out_parallel_mag = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -1.0;
        let r_out_parallel = normal.scale(r_out_parallel_mag);
        r_out_perp + r_out_parallel
    }

    #[inline]
    pub fn sqrt_mut(&mut self) {
        self.vec.x = self.vec.x.sqrt();
        self.vec.y = self.vec.y.sqrt();
        self.vec.z = self.vec.z.sqrt();
    }

    #[inline]
    pub fn scale_mut(&mut self, s: Real) {
        self.vec.scale_mut(s)
    }
    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> Real {
        self.vec.dot(&rhs.vec)
    }
    #[inline]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            vec: self.vec.cross(&rhs.vec),
        }
    }

    #[inline]
    pub fn unit(&self) -> Vec3 {
        Vec3 {
            vec: self.vec.normalize(),
        }
    }
    #[inline]
    pub fn unit_mut(&mut self) -> Real {
        self.vec.normalize_mut()
    }

    pub fn component_add(&self, value: f64) -> Vec3 {
        Vec3::new(self.vec.x + value, self.vec.y + value, self.vec.z + value)
    }

    pub fn map<F: Fn(f64) -> f64>(&self, f: F) -> Vec3 {
        Vec3::new(f(self.vec.x), f(self.vec.y), f(self.vec.z))
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            vec: self.vec + rhs.vec,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            vec: self.vec - rhs.vec,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.vec.x * rhs.vec.x,
            self.vec.y * rhs.vec.y,
            self.vec.z * rhs.vec.z,
        )
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.vec += rhs.vec
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.vec -= rhs.vec
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point(pub Vec3);

impl Default for Point {
    fn default() -> Self {
        Point(Vec3::new(0.0, 0.0, 0.0))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ray {
    pub orig: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Ray {
        Ray {
            orig,
            direction: dir,
        }
    }
    pub fn at(&self, t: Real) -> Point {
        Point(self.orig.0 + self.direction.scale(t))
    }
}

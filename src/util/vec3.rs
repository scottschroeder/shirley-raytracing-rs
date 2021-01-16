use rand::prelude::{Rng, ThreadRng};
use std::ops;

type Real = f64;

const NEAR_ZERO: f64 = 1e-8;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    vec: nalgebra::Vector3<Real>,
}

#[inline]
fn random_real(rng: &mut ThreadRng, min: Real, max: Real) -> Real {
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

impl Vec3 {
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

    pub fn random() -> Vec3 {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<Real>(), rng.gen::<Real>(), rng.gen::<Real>())
    }

    pub fn random_range(min: Real, max: Real) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            random_real(&mut rng, min, max),
            random_real(&mut rng, min, max),
            random_real(&mut rng, min, max),
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

#[derive(Debug, Clone, Copy)]
pub struct Point(pub Vec3);

impl Default for Point {
    fn default() -> Self {
        Point(Vec3::new(0.0, 0.0, 0.0))
    }
}

#[derive(Debug, Clone, Copy)]
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

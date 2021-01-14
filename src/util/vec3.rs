type Real = f64;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    vec: nalgebra::Vector3<Real>,
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

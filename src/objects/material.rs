use crate::{
    objects::hittable::HitRecord,
    util::{random_in_unit_sphere, random_unit_vector, Color, Ray},
};

pub struct Scatter {
    pub direction: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let mut scatter = record.normal + random_unit_vector();
        if scatter.near_zero() {
            scatter = record.normal;
        }
        let direction = Ray {
            orig: record.point,
            direction: scatter,
        };
        Some(Scatter {
            direction,
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Option<f64>) -> Metal {
        let mut fuzz = fuzz.unwrap_or(0.0);
        if fuzz > 1.0 {
            fuzz = 1.0;
        }
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<Scatter> {
        let reflected = ray.direction.unit().reflect(&record.normal);

        let direction = Ray {
            orig: record.point,
            direction: reflected + random_in_unit_sphere().scale(self.fuzz),
        };

        Some(Scatter {
            direction,
            attenuation: self.albedo,
        })
    }
}

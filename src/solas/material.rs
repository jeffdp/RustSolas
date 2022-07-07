// Material

use super::*;
use cgmath::{prelude::*, Vector3};

// use super::ray::random_in_unit_sphere;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: super::Hit) -> Option<(Vector3<f64>, Ray)>;
}

#[derive(Copy, Clone)]
pub struct LambertianMaterial {
    albedo: Vector3<f64>,
}

impl LambertianMaterial {
    pub fn new(albedo: Vector3<f64>) -> LambertianMaterial {
        LambertianMaterial { albedo: albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray: &Ray, hit: Hit) -> Option<(Vector3<f64>, Ray)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        let attenuation = self.albedo;

        if scattered.direction.dot(hit.normal) <= 0.0 {
            return None;
        }

        Some((attenuation, scattered))
    }
}

// Material

use super::*;
use cgmath::{prelude::*, Vector3};
use random_number::random;

#[derive(Copy, Clone)]
// TODO: Super lame version of Material, to be replaced with a Material Trait once I know how to do that.
pub struct Material {
    lambertian: Option<LambertianMaterial>,
    metal: Option<MetalMaterial>,
    dialectric: Option<DialectricMaterial>,
}

// Temporary helper functions until a better Material API can be built.

pub fn make_lambertian(albedo: Vector3<f64>) -> Material {
    Material::new(Some(LambertianMaterial::new(albedo)), None, None)
}

pub fn make_metal(albedo: Vector3<f64>, fuzz: f64) -> Material {
    Material::new(None, Some(MetalMaterial::new(albedo, fuzz)), None)
}

pub fn make_dialectric(refractive_index: f64) -> Material {
    Material::new(None, None, Some(DialectricMaterial::new(refractive_index)))
}

impl Material {
    pub fn new(
        lambertian: Option<LambertianMaterial>,
        metal: Option<MetalMaterial>,
        dialectric: Option<DialectricMaterial>,
    ) -> Self {
        Material {
            lambertian,
            metal,
            dialectric,
        }
    }

    pub fn scatter(&self, ray: &Ray, hit: Hit) -> Option<(Vector3<f64>, Ray)> {
        if let Some(lambertian) = self.lambertian {
            return lambertian.scatter(ray, hit);
        }

        if let Some(metal) = self.metal {
            return metal.scatter(ray, hit);
        }

        if let Some(dialectric) = self.dialectric {
            return dialectric.scatter(ray, hit);
        }

        None
    }
}

#[derive(Copy, Clone)]
pub struct LambertianMaterial {
    albedo: Vector3<f64>,
}

impl LambertianMaterial {
    pub fn new(albedo: Vector3<f64>) -> Self {
        LambertianMaterial { albedo: albedo }
    }

    pub fn scatter(&self, _ray: &Ray, hit: Hit) -> Option<(Vector3<f64>, Ray)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}

#[derive(Copy, Clone)]
pub struct MetalMaterial {
    albedo: Vector3<f64>,
    fuzz: f64,
}

fn reflect(from: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    from - n * 2.0 * from.dot(n)
}

fn refract(v: Vector3<f64>, n: Vector3<f64>, ni_over_nt: f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant <= 0.0 {
        return Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt());
    } else {
        return None;
    }
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r02 = r0 * r0;

    r02 + (1.0 - r02) * (1.0 - cosine).powi(5)
}

impl MetalMaterial {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }

    pub fn scatter(&self, ray: &Ray, hit: Hit) -> Option<(Vector3<f64>, Ray)> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        let scattered = Ray::new(hit.p, reflected + random_in_unit_sphere());
        let attenuation = self.albedo;

        if scattered.direction.dot(hit.normal) <= 0.0 {
            return None;
        }

        Some((attenuation, scattered))
    }
}

#[derive(Copy, Clone)]
pub struct DialectricMaterial {
    refractive_index: f64,
}

impl DialectricMaterial {
    pub fn new(refractive_index: f64) -> Self {
        DialectricMaterial { refractive_index }
    }

    pub fn scatter(&self, ray: &Ray, hit: Hit) -> Option<(Vector3<f64>, Ray)> {
        let reflected = reflect(ray.direction, hit.normal);
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        let outward_normal: Vector3<f64>;
        let ni_over_nt: f64;
        let cosine: f64;
        if ray.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.refractive_index;
            cosine =
                self.refractive_index * ray.direction.dot(hit.normal) / ray.direction.magnitude();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -ray.direction.dot(hit.normal) / ray.direction.magnitude();
        }

        if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refractive_index);
            let refraction_chance: f64 = random!();
            if refraction_chance < reflect_prob {
                return Some((attenuation, Ray::new(hit.p, reflected)));
            } else {
                return Some((attenuation, Ray::new(hit.p, refracted)));
            }
        } else {
            return Some((attenuation, Ray::new(hit.p, reflected)));
        }
    }
}

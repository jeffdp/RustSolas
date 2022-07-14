/// Camera
use cgmath::{prelude::*, Vector3};
use random_number::random;

pub struct Camera {
    pub origin: Vector3<f64>,
    pub lower_left: Vector3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub w: Vector3<f64>,
    pub lens_radius: f64,
}

fn random_in_unit_disk() -> Vector3<f64> {
    loop {
        let p: Vector3::<f64> = 2.0 * Vector3::new(random!(), random!(), random!());

        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Vector3<f64>,
        look_at: Vector3<f64>,
        vup: Vector3<f64>,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        let lower_left =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> super::Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        
        super::Ray {
            origin: self.origin + offset,
            direction: self.lower_left + s * self.horizontal + t*self.vertical - self.origin - offset,
        }
    }
}

/// Camera

use cgmath::{Vector3, prelude::*};

pub struct Camera {
    pub lower_left: Vector3::<f64>,
    pub horizontal: Vector3::<f64>,
    pub vertical: Vector3::<f64>,
    pub origin: Vector3::<f64>
}

impl Camera {
    pub fn new(
        lower_left: Vector3::<f64>,
        horizontal: Vector3::<f64>,
        vertical: Vector3::<f64>,
        origin: Vector3::<f64>) -> Camera {
            Camera {
                lower_left: lower_left,
                horizontal: horizontal,
                vertical: vertical,
                origin: origin
            }
    }

    pub fn ray(&self, u: f64, v: f64) -> super::Ray {
        let direction = self.lower_left + u * self.horizontal + v * self.vertical - self.origin;
        
        super::Ray {
            origin: self.origin, 
            direction: direction
        }
    }
}

pub fn create_camera(
    look_from: Vector3::<f64>,
    look_at: Vector3::<f64>,
    vertical: Vector3::<f64>,
    vfov: f64,
    aspect: f64,
    aperture: f64,
    focus_dist: f64) -> Camera {
        let theta = vfov * 3.1415926 / 180.0;
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vertical.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera::new(
            lower_left,
            horizontal,
            vertical,
            origin
        )
    }
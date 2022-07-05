/// Camera

use cgmath::{Vector3};

pub struct Camera {
    pub lower_left: Vector3::<f64>,
    pub horizontal: Vector3::<f64>,
    pub vertical: Vector3::<f64>,
    pub origin: Vector3::<f64>,
    pub lens_radius: f64
}

impl Camera {
    pub fn new(lower_left: Vector3::<f64>,
        horizontal: Vector3::<f64>,
        vertical: Vector3::<f64>,
        origin: Vector3::<f64>,
        lens_radius: f64) -> Camera {
            Camera {
                lower_left: lower_left,
                horizontal: horizontal,
                vertical: vertical,
                origin: origin,
                lens_radius: lens_radius
            }
    }

    pub fn ray(&self, u: f64, v: f64) -> super::Ray {
        let rd = self.lens_radius * super::ray::random_in_unit_sphere();
        let offset = u * rd.x + v * rd.y;

        let ray_origin = Vector3 {
            x: self.origin.x + offset,
            y: self.origin.y + offset,
            z: self.origin.z + offset
        };
        
        let dir = self.lower_left + u * self.horizontal + v * self.vertical - self.origin;
        let ray_direction: Vector3<f64> = Vector3 {
            x: dir.x - offset,
            y: dir.y - offset,
            z: dir.z - offset
        };

        super::Ray {
            origin: ray_origin, 
            direction: ray_direction
        }
    }
}
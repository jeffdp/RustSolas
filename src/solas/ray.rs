/// Ray
use cgmath::{Vector3, prelude::*};
use random_number::random;

pub struct Ray {
    pub origin: Vector3::<f64>,
    pub direction: Vector3::<f64>
}

impl Ray {
    fn new(origin: Vector3::<f64>, direction: Vector3::<f64>) -> Ray {
        Ray {
            origin, 
            direction
        }
    }

    fn point(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    // let mut p = Vector3::new(0.0, 0.0, 0.0);

    loop {
        let x: f64 = random!();
        let y: f64 = random!();
        let z: f64 = random!();

        let p = 2.0 * Vector3::new(x, y, z) - Vector3::new(1.0, 1.0, 1.0);

        if p.magnitude() * p.magnitude() < 1.0 {
            return p
        }
    }
}
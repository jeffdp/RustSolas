use cgmath::Vector3;
use image::Rgb;

pub trait RgbExt {
    fn multiply(&self, x: f64) -> Self;
}

impl RgbExt for Rgb<f64> {
    fn multiply(&self, x: f64) -> Self {
        Rgb([self[0] * x, self[1] * x, self[2] * x])
    }
}

pub trait VectorExt {
    fn to_color(&self) -> Rgb<f64>;
    fn sub(&self, c: f64) -> Vector3<f64>;
}

impl VectorExt for Vector3<f64> {
    fn to_color(&self) -> Rgb<f64> {
        Rgb([self.x, self.y, self.z])
    }

    fn sub(&self, c: f64) -> Self {
        Vector3::new(self.x - c, self.y - c, self.z - c)
    }
}

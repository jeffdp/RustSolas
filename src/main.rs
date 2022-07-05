#![allow(dead_code)]

// Main

mod solas;
mod prelude {
    pub use crate::solas::{Camera, Color, Ray};
}

use cgmath::{Vector3, prelude::*};
use solas::*;
use image::{ImageBuffer, RgbImage, Rgb};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn main() {
    let image = gradient_image(WIDTH, HEIGHT);

    image.save("output/image.png").unwrap();
}

fn gradient(ray: Ray) -> Rgb<u8> {
    let direction = ray.direction.normalize();
    let t = (direction.y + 1.0) / 2.0;
    
    let start = Vector3::new(1.0 - t, 1.0 - t, 1.0 - t);
    let end = Vector3::new(0.5, 0.7, 1.0);

    image::Rgb([
        ((start.x + t * end.x) * 255.0) as u8, 
        ((start.y + t * end.y) * 255.0) as u8, 
        ((start.z + t * end.z) * 255.0) as u8])
}

fn gradient_image(width: u32, height: u32) -> RgbImage {
    let mut image: RgbImage = ImageBuffer::new(width, height);
    let camera = Camera {
        lower_left: Vector3::new(-2.0, -1.0, -1.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
        origin: Vector3::new(0.0, 0.0, 0.0),
        lens_radius: 1.0
    };

    let w = width as f64;
    let h = height as f64;
    for y in (0..height).rev() {
        for x in 0..width {
            let i = x as f64;
            let j = y as f64;

            let u = (i + 0.5) / w;
            let v = (j + 0.5) / h;
            let ray = camera.ray(u, v);
            let pixel = gradient(ray);

            image.put_pixel(x, height - y - 1, pixel);
        }
    }

    image
}

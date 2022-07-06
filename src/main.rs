#![allow(dead_code)]

// Main

mod solas;
mod prelude {
    pub use crate::solas::{Camera, Color, Ray};
}

use cgmath::{Vector3, prelude::*};
use solas::*;
use image::{ImageBuffer, RgbImage, Rgb};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = (1200.0 * 9.0/16.0) as u32;

fn main() {
    // let image = gradient_image(WIDTH, HEIGHT);
    let image = two_spheres(WIDTH, HEIGHT);

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

    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aspect_ratio = 16.0 / 9.0;
    let vfov = 20.0;
    let aperture = aspect_ratio;

    let camera = Camera::new(look_from, look_at, vup, vfov, 
        aspect_ratio, aperture, focus_dist);

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

fn two_spheres(width: u32, height: u32) -> RgbImage {
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aspect_ratio = 16.0 / 9.0;
    let vfov = 20.0;
    let aperture = aspect_ratio;

    let camera = Camera::new(look_from, look_at, vup, vfov, 
        aspect_ratio, aperture, focus_dist);

    let ground = Sphere::new(Vector3::new(0.0, -100.5, 0.5), 100.0);
    let ball = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let objects = [ground, ball];
    let samples = 1;

    trace(&objects, camera, width, height, samples)
}

fn trace(objects: &[Sphere], camera: Camera, width: u32, height: u32, _samples: u16) -> RgbImage {
    let mut image: RgbImage = ImageBuffer::new(width, height);

    let w = width as f64;
    let h = height as f64;
    for y in (0..height).rev() {
        for x in 0..width {
            let i = x as f64;
            let j = y as f64;

            let u = (i + 0.5) / w;
            let v = (j + 0.5) / h;
            let ray = camera.ray(u, v);

            if let Some(_hit) = hit(ray, 0.0, 10000.0, &objects) {
                let pixel =  image::Rgb([255, 0, 0]);
                image.put_pixel(x, height - y - 1, pixel);
            }
        }
    }

    image
}
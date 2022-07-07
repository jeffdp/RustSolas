#![allow(dead_code)]

// Main

mod solas;
mod prelude {
    pub use crate::solas::{Camera, Color, Ray};
}

use cgmath::{prelude::*, Vector3};
use image::{ImageBuffer, Rgb, RgbImage};
use solas::*;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = (1200.0 * 9.0 / 16.0) as u32;

fn main() {
    // let image = gradient_image(WIDTH, HEIGHT);
    let image = two_spheres(WIDTH, HEIGHT);

    image.save("output/image.png").unwrap();
}

/*
private func color(_ ray: Ray, objects: [Hitable], depth: Int) -> Color {
    if let hit = hit(ray: ray, objects: objects) {
        guard depth < 10, let (attenuation, scattered) = hit.material.scatter(ray: ray, hit: hit) else {
            return Color()
        }

        return attenuation * color(scattered, objects: objects, depth: depth+1)
    }

    let unitDirection = ray.direction.normalized()
    let t = 0.5 * unitDirection.y + 1.0
    let lerp = (1.0 - t) * vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0)

    return Color(lerp)
}
*/

fn color(ray: &Ray, objects: &[Sphere], depth: i8) -> Rgb<u8> {
    if let Some(hit) = hit(ray, 0.0, 10000.0, &objects) {
        if depth >= 10 {
            return Rgb([0, 0, 0]);
        }

        if let Some((attenuation, _)) = hit.material.scatter(ray, hit) {
            let red = (255.0 * attenuation.x) as u8;
            let green = (255.0 * attenuation.y) as u8;
            let blue = (255.0 * attenuation.z) as u8;
            let pixel = image::Rgb([red, green, blue]);

            return pixel;
        } else {
            return Rgb([0, 0, 0]);
        }
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * unit_direction.y + 1.0;
    let lerp = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);

    let red = (255.0 * lerp.x) as u8;
    let green = (255.0 * lerp.y) as u8;
    let blue = (255.0 * lerp.z) as u8;
    let pixel = image::Rgb([red, green, blue]);

    return pixel;
}

fn gradient(ray: Ray) -> Rgb<u8> {
    let direction = ray.direction.normalize();
    let t = (direction.y + 1.0) / 2.0;

    let start = Vector3::new(1.0 - t, 1.0 - t, 1.0 - t);
    let end = Vector3::new(0.5, 0.7, 1.0);

    image::Rgb([
        ((start.x + t * end.x) * 255.0) as u8,
        ((start.y + t * end.y) * 255.0) as u8,
        ((start.z + t * end.z) * 255.0) as u8,
    ])
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

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

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

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    let ground_material = LambertianMaterial::new(Vector3::new(0.8, 0.8, 0.0));
    let ground = Sphere::new(Vector3::new(0.0, -100.5, 0.5), 100.0, ground_material);

    let ball_material = LambertianMaterial::new(Vector3::new(0.1, 0.1, 0.8));
    let ball = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, ball_material);
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

            let pixel = color(&ray, &objects, 1);
            image.put_pixel(x, height - y - 1, pixel);
        }
    }

    image
}

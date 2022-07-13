#![allow(dead_code)]

// Main

mod solas;
mod prelude {
    pub use crate::solas::{Camera, Color, Ray};
}

use cgmath::{prelude::*, Vector3};
use image::{ImageBuffer, Rgb, RgbImage};
use random_number::random;

use solas::*;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = (1200.0 * 9.0 / 16.0) as u32;
const SAMPLES: u16 = 10;

fn main() {
    // let image = gradient_image(WIDTH, HEIGHT);
    // let image = two_spheres(WIDTH, HEIGHT);
    // let image = four_spheres(WIDTH, HEIGHT);
    let image = random_spheres(WIDTH, HEIGHT);

    image.save("output/image.png").unwrap();
}

fn mult(a: Rgb<f64>, b: Rgb<f64>) -> Rgb<f64> {
    Rgb([a[0] * b[0], a[1] * b[1], a[2] * b[2]])
}

fn background(ray: &Ray) -> Rgb<f64> {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * unit_direction.y + 1.0;
    let lerp = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);

    image::Rgb([lerp.x, lerp.y, lerp.z])
}

fn color_normal(ray: &Ray, objects: &[Sphere]) -> Rgb<f64> {
    if let Some(hit) = hit(ray, 0.001, 10000.0, &objects) {
        return Rgb([
            (hit.normal.x + 1.0) / 2.0,
            (hit.normal.y + 1.0) / 2.0,
            (hit.normal.z + 1.0) / 2.0,
        ]);
    }

    background(ray)
}

fn basic_color(ray: &Ray, objects: &[Sphere], depth: i8) -> Rgb<f64> {
    if depth < 10 {
        if let Some(hit) = hit(ray, 0.001, 10000.0, &objects) {
            let target = hit.p + hit.normal + random_in_unit_sphere();
            let ray = Ray::new(hit.p, target - hit.p);
            return color(&ray, objects, depth + 1).multiply(0.5);
        }
    }

    background(ray)
}

fn color(ray: &Ray, objects: &[Sphere], depth: i8) -> Rgb<f64> {
    if let Some(hit) = hit(ray, 0.001, 10000.0, &objects) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, hit) {
            if depth < 10 {
                let new_color = color(&scattered, objects, depth + 1);
                return mult(attenuation.to_color(), new_color);
            }
        }

        return Rgb([0.0, 0.0, 0.0]);
    }

    background(ray)
}

fn gradient(ray: Ray) -> Rgb<f64> {
    let direction = ray.direction.normalize();
    let t = (direction.y + 1.0) / 2.0;

    let start = Vector3::new(1.0 - t, 1.0 - t, 1.0 - t);
    let end = Vector3::new(0.5, 0.7, 1.0);

    image::Rgb([
        start.x + t * end.x,
        start.y + t * end.y,
        start.z + t * end.z,
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
            let pixel = Rgb([
                (pixel[0] * 255.0) as u8,
                (pixel[1] * 255.0) as u8,
                (pixel[2] * 255.0) as u8,
            ]);

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

    let ground_material = make_lambertian(Vector3::new(0.8, 0.8, 0.0));
    let ground = Sphere::new(Vector3::new(0.0, -100.5, 0.5), 100.0, ground_material);

    let ball_material = make_lambertian(Vector3::new(0.1, 0.1, 0.8));
    let ball = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, ball_material);
    let objects = [ground, ball];
    let samples = SAMPLES;

    trace(&objects, camera, width, height, samples)
}

fn four_spheres(width: u32, height: u32) -> RgbImage {
    let look_from = Vector3::new(0.0, 3.0, 6.0);
    let look_at = Vector3::new(0.0, 0.7, 0.0);
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

    let ground_material = make_lambertian(Vector3::new(0.8, 0.8, 0.0));
    let ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, ground_material);

    let left_material = make_dialectric(1.5);
    let left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, left_material);

    let middle_material = make_lambertian(Vector3::new(0.8, 0.3, 0.3));
    let middle = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, middle_material);

    let right_material = make_metal(Vector3::new(0.8, 0.6, 0.2), 0.0);
    let right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, right_material);

    let objects = [ground, left, middle, right];
    let samples = SAMPLES;

    trace(&objects, camera, width, height, samples)
}

fn random_spheres(width: u32, height: u32) -> RgbImage {
    let look_from = Vector3::new(16.0, 2.0, 4.0);
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

    let mut objects: Vec<Sphere> = vec![];

    // Ground sphere
    objects.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        make_lambertian(Vector3::new(0.5, 0.5, 0.5)),
    ));

    // Random spheres
    let rand = || {
        let x: f64 = random!();
        x
    };

    for x in -11..=11 {
        for y in -11..=11 {
            let a = x as f64;
            let b = y as f64;
            let center = Vector3::new(a + (0.9 * rand()), 0.2, b + 0.9 * rand());

            let material = rand();
            if material <= 0.8 {
                let albedo = Vector3::new(rand(), rand(), rand());
                objects.push(Sphere::new(center, 0.2, make_lambertian(albedo)));
            } else if material <= 0.95 {
                let albedo = Vector3::new(
                    0.5 * (1.0 + rand()),
                    0.5 * (1.0 + rand()),
                    0.5 * (1.0 + rand()),
                );
                let fuzz = 0.5 * rand();
                objects.push(Sphere::new(center, 0.2, make_metal(albedo, fuzz)))
            } else {
                objects.push(Sphere::new(center, 0.2, make_dialectric(1.5)));
            }
        }
    }

    // Far sphere
    objects.push(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        make_lambertian(Vector3::new(0.4, 0.2, 0.1)),
    ));

    // middle sphere
    objects.push(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        make_dialectric(1.5),
    ));

    // Near sphere
    objects.push(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        make_metal(Vector3::new(0.7, 0.6, 0.5), 0.0),
    ));

    let samples = SAMPLES;
    trace(&objects, camera, width, height, samples)
}

fn percent_complete(y: u32, height: u32) -> u32 {
    let y = y as f64;
    let height = height as f64;

    ((height - y) / height * 100.0) as u32
}

fn trace(objects: &[Sphere], camera: Camera, width: u32, height: u32, samples: u16) -> RgbImage {
    let mut image: RgbImage = ImageBuffer::new(width, height);

    let w = width as f64;
    let h = height as f64;
    let progress_step = (h / 10.0) as u32;
    for y in (0..height).rev() {
        if y % progress_step == 0 {
            println!("{}% complete", percent_complete(y, height));
        }

        for x in 0..width {
            let mut accumulated_color = Rgb([0.0, 0.0, 0.0]);

            for _ in 0..samples {
                let i = x as f64;
                let j = y as f64;

                let u = (i + 0.5) / w;
                let v = (j + 0.5) / h;
                let ray = camera.ray(u, v);

                let pixel = color(&ray, &objects, 1);
                accumulated_color[0] += pixel[0];
                accumulated_color[1] += pixel[1];
                accumulated_color[2] += pixel[2];
            }

            let accumulated_color = accumulated_color.gamma2();

            let pixel = Rgb([
                (accumulated_color[0] / samples as f64 * 255.0) as u8,
                (accumulated_color[1] / samples as f64 * 255.0) as u8,
                (accumulated_color[2] / samples as f64 * 255.0) as u8,
            ]);

            image.put_pixel(x, height - y - 1, pixel);
        }
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_to_color() {
        let vec = Vector3::new(0.25, 0.0, 0.0);
        let color = vec.to_color();

        assert_eq!(color[0], 0.25);
    }
}

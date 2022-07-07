mod camera;
mod color;
mod extensions;
mod intersections;
mod material;
mod ray;

pub use camera::Camera;
pub use color::Color;
pub use extensions::{RgbExt, VectorExt};
pub use intersections::{hit, Hit, Sphere};
pub use material::{DialectricMaterial, LambertianMaterial, Material, MetalMaterial};
pub use ray::{random_in_unit_sphere, Ray};

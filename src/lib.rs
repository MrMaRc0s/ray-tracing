//! Small, dependency-free ray tracer used by the `rt` binary.

pub mod canvas;
pub mod geometry;
pub mod material;
pub mod math;

pub use canvas::Canvas;
pub use geometry::{HitRecord, Hittable};
pub use material::Material;
pub use math::{Color, Point3, Ray, Vec3};

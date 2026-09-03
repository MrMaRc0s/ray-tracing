//! Small, dependency-free ray tracer used by the `rt` binary.

pub mod material;
pub mod math;

pub use material::Material;
pub use math::{Color, Point3, Ray, Vec3};

//! Small, dependency-free ray tracer used by the `rt` binary.

pub mod camera;
pub mod canvas;
pub mod cli;
pub mod geometry;
pub mod light;
pub mod material;
pub mod math;
pub mod preview;
pub mod renderer;
pub mod scene;
pub mod scenes;
pub mod shapes;

pub use camera::Camera;
pub use canvas::Canvas;
pub use cli::{CliCommand, RenderOptions};
pub use geometry::{HitRecord, Hittable};
pub use light::PointLight;
pub use material::Material;
pub use math::{Color, Point3, Ray, Vec3};
pub use preview::render_preview;
pub use renderer::render;
pub use scene::Scene;
pub use scenes::build_scene;
pub use shapes::{Cube, Cylinder, Plane, Sphere};

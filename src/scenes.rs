use crate::{Color, Cube, Cylinder, Material, Plane, Point3, PointLight, Scene, Sphere, Vec3};

pub const NAMES: [&str; 3] = ["sphere", "dim", "all"];

pub fn build_scene(name: &str) -> Result<(Scene, PointLight), String> {
    match name {
        "sphere" => Ok(sphere_only()),
        "dim" => Ok(plane_and_cube_dim()),
        "all" => Ok(all_objects()),
        _ => Err(format!(
            "scene '{name}' is not available; use one of: preview, {}",
            NAMES.join(", ")
        )),
    }
}

fn light_material(color: Color) -> Material {
    Material::new(color, 0.1, 0.9)
}

fn sphere_only() -> (Scene, PointLight) {
    let mut scene = Scene::new(Color::new(0.02, 0.02, 0.05));
    scene.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        1.0,
        light_material(Color::new(0.9, 0.2, 0.2)),
    ));

    let light = PointLight::new(Point3::new(5.0, 5.0, 5.0), Color::new(1.0, 1.0, 1.0), 1.0);
    (scene, light)
}

fn plane_and_cube_dim() -> (Scene, PointLight) {
    let mut scene = Scene::new(Color::new(0.02, 0.02, 0.05));
    scene.add(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        light_material(Color::new(0.7, 0.7, 0.7)),
    ));
    scene.add(Cube::new(
        Point3::new(0.0, -0.5, -1.0),
        0.5,
        light_material(Color::new(0.2, 0.3, 0.9)),
    ));

    let light = PointLight::new(Point3::new(5.0, 5.0, 5.0), Color::new(1.0, 1.0, 1.0), 0.3);
    (scene, light)
}

fn all_objects() -> (Scene, PointLight) {
    let mut scene = Scene::new(Color::new(0.02, 0.02, 0.05));
    scene.add(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        light_material(Color::new(0.7, 0.7, 0.7)),
    ));
    scene.add(Sphere::new(
        Point3::new(-3.0, 0.0, -1.0),
        1.0,
        light_material(Color::new(0.9, 0.2, 0.2)),
    ));
    scene.add(Cube::new(
        Point3::new(0.0, -0.5, -1.0),
        0.5,
        light_material(Color::new(0.2, 0.3, 0.9)),
    ));
    scene.add(Cylinder::new(
        Point3::new(3.0, -1.0, -1.5),
        0.6,
        1.4,
        light_material(Color::new(0.2, 0.8, 0.3)),
    ));

    let light = PointLight::new(Point3::new(5.0, 5.0, 5.0), Color::new(1.0, 1.0, 1.0), 1.0);
    (scene, light)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_the_sphere_only_scene() {
        let (scene, _light) = build_scene("sphere").unwrap();
        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn builds_the_dim_plane_and_cube_scene_with_lower_brightness() {
        let (scene, light) = build_scene("dim").unwrap();
        assert_eq!(scene.len(), 2);
        assert!(light.brightness() < 1.0);
    }

    #[test]
    fn builds_the_all_objects_scene() {
        let (scene, light) = build_scene("all").unwrap();
        assert_eq!(scene.len(), 4);
        assert_eq!(light.brightness(), 1.0);
    }

    #[test]
    fn rejects_an_unknown_scene_name() {
        assert!(build_scene("nonexistent").is_err());
    }
}

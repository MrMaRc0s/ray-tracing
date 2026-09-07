use crate::{
    scatter_particles, Color, Cube, Cylinder, Material, Plane, Point3, PointLight, Scene, Sphere,
    Texture, Vec3, WavyPlane,
};

pub const NAMES: [&str; 7] = [
    "sphere",
    "dim",
    "all",
    "textures",
    "glass",
    "particles",
    "fluid",
];

pub fn build_scene(name: &str) -> Result<(Scene, PointLight), String> {
    match name {
        "sphere" => Ok(sphere_only()),
        "dim" => Ok(plane_and_cube_dim()),
        "all" => Ok(all_objects()),
        "textures" => Ok(textured_floor()),
        "glass" => Ok(reflective_and_refractive()),
        "particles" => Ok(particle_cloud()),
        "fluid" => Ok(fluid_surface()),
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

/// Bonus: a checkered floor demonstrating procedural textures, with a plain
/// sphere on top so the same light shows both a textured and an untextured
/// surface side by side.
fn textured_floor() -> (Scene, PointLight) {
    let mut scene = Scene::new(Color::new(0.02, 0.02, 0.05));
    scene.add(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::new(Color::new(0.9, 0.9, 0.9), 0.1, 0.9).with_texture(Texture::checker(
            Color::new(0.9, 0.9, 0.9),
            Color::new(0.1, 0.1, 0.15),
            1.0,
        )),
    ));
    scene.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        1.0,
        light_material(Color::new(0.9, 0.2, 0.2)),
    ));

    let light = PointLight::new(Point3::new(5.0, 5.0, 5.0), Color::new(1.0, 1.0, 1.0), 1.0);
    (scene, light)
}

/// Bonus: a mirror-like sphere and a glass sphere over a checkered floor,
/// demonstrating both reflective and refractive materials in one scene.
fn reflective_and_refractive() -> (Scene, PointLight) {
    let mut scene = Scene::new(Color::new(0.02, 0.02, 0.05));
    scene.add(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::new(Color::new(0.8, 0.8, 0.8), 0.1, 0.9).with_texture(Texture::checker(
            Color::new(0.8, 0.8, 0.8),
            Color::new(0.15, 0.15, 0.2),
            1.0,
        )),
    ));
    scene.add(Sphere::new(
        Point3::new(-1.5, 0.0, -1.0),
        1.0,
        Material::new(Color::new(0.9, 0.9, 0.9), 0.05, 0.1).with_reflectivity(0.9),
    ));
    scene.add(Sphere::new(
        Point3::new(1.5, 0.0, -1.0),
        1.0,
        Material::new(Color::new(1.0, 1.0, 1.0), 0.02, 0.05).with_transparency(0.95, 1.5),
    ));

    let light = PointLight::new(Point3::new(5.0, 5.0, 5.0), Color::new(1.0, 1.0, 1.0), 1.0);
    (scene, light)
}

/// Bonus: a floor plus a scattered cloud of small spheres (dust/sparks),
/// demonstrating that particle effects are just many small objects added
/// to the same scene the four core shapes already build on.
fn particle_cloud() -> (Scene, PointLight) {
    let mut scene = Scene::new(Color::new(0.02, 0.02, 0.05));
    scene.add(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        light_material(Color::new(0.4, 0.4, 0.45)),
    ));

    scatter_particles(
        &mut scene,
        120,
        Point3::new(-2.5, -0.9, -3.0),
        Point3::new(2.5, 2.0, 0.0),
        0.07,
        1234,
        |i| {
            let warmth = (i % 5) as f64 / 4.0;
            Color::new(0.9, 0.6 + 0.3 * warmth, 0.3)
        },
    );

    let light = PointLight::new(Point3::new(5.0, 5.0, 5.0), Color::new(1.0, 1.0, 1.0), 1.0);
    (scene, light)
}

/// Bonus: a rippled, reflective/refractive water surface (see
/// [`WavyPlane`]) with a sphere floating on it and a checkered sea floor
/// beneath, so refracted rays through the water have something to reveal
/// rather than fading straight to the background.
fn fluid_surface() -> (Scene, PointLight) {
    let mut scene = Scene::new(Color::new(0.05, 0.08, 0.12));
    scene.add(Plane::new(
        Point3::new(0.0, -3.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::new(Color::new(0.76, 0.7, 0.5), 0.15, 0.6).with_texture(Texture::checker(
            Color::new(0.76, 0.7, 0.5),
            Color::new(0.55, 0.48, 0.3),
            0.6,
        )),
    ));
    scene.add(WavyPlane::new(
        -1.0,
        Material::new(Color::new(0.05, 0.25, 0.4), 0.1, 0.3)
            .with_reflectivity(0.3)
            .with_transparency(0.6, 1.33),
        0.15,
        1.5,
    ));
    scene.add(Sphere::new(
        Point3::new(0.0, -0.3, -1.2),
        0.7,
        light_material(Color::new(0.9, 0.7, 0.2)),
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

    #[test]
    fn builds_the_textured_floor_scene() {
        let (scene, _light) = build_scene("textures").unwrap();
        assert_eq!(scene.len(), 2);
    }

    #[test]
    fn builds_the_glass_scene_with_a_mirror_and_a_transparent_sphere() {
        let (scene, _light) = build_scene("glass").unwrap();
        assert_eq!(scene.len(), 3);
    }

    #[test]
    fn builds_the_particle_cloud_scene_with_every_particle_plus_the_floor() {
        let (scene, _light) = build_scene("particles").unwrap();
        assert_eq!(scene.len(), 121);
    }

    #[test]
    fn builds_the_fluid_scene_with_a_wavy_surface_and_a_sphere() {
        let (scene, _light) = build_scene("fluid").unwrap();
        assert_eq!(scene.len(), 3);
    }
}

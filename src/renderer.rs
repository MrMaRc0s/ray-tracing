use crate::{Camera, Canvas, Color, HitRecord, PointLight, Ray, Scene};

/// Minimum distance used both when searching for the closest scene hit along
/// a primary ray and as the origin offset for shadow, reflection, and
/// refraction rays. Without this bias a ray can immediately re-intersect the
/// surface it was just cast from due to floating point rounding, producing
/// "shadow acne" artifacts.
const SHADOW_BIAS: f64 = 1e-4;

/// How many times a ray may bounce (reflect or refract) before giving up and
/// contributing no further light. Bounds the cost of scenes built from
/// mutually-facing mirrors or nested glass, which would otherwise recurse
/// forever.
const MAX_BOUNCES: u32 = 5;

/// Renders the scene as seen by `camera`, lit by `light`.
///
/// For every pixel a camera ray is cast into the scene and traced
/// recursively: a hit is shaded using `light`, taking into account whether
/// the hit point is occluded from the light by another object (a hard
/// shadow, with no penumbra), then blended with a reflected and/or
/// refracted ray according to the hit material's `reflectivity` and
/// `transparency`. Pixels whose ray hits nothing are filled with the
/// scene's background color.
pub fn render(scene: &Scene, camera: &Camera, light: &PointLight) -> Canvas {
    let mut canvas = Canvas::new(camera.image_width(), camera.image_height());

    for y in 0..camera.image_height() {
        for x in 0..camera.image_width() {
            let ray = camera.ray_for_pixel(x, y);
            let color = trace(scene, &ray, light, MAX_BOUNCES);
            canvas.set_pixel(x, y, color);
        }
    }

    canvas
}

fn trace(scene: &Scene, ray: &Ray, light: &PointLight, bounces_left: u32) -> Color {
    let Some(hit) = scene.hit(ray, SHADOW_BIAS, f64::INFINITY) else {
        return scene.background();
    };

    let in_shadow = is_in_shadow(scene, light, &hit);
    let local_color = light.shade(&hit, in_shadow);

    let reflectivity = hit.material.reflectivity.clamp(0.0, 1.0);
    let transparency = hit.material.transparency.clamp(0.0, 1.0 - reflectivity);
    if bounces_left == 0 || (reflectivity == 0.0 && transparency == 0.0) {
        return local_color;
    }

    let incoming = ray.direction.normalized();
    let mut color = local_color * (1.0 - reflectivity - transparency);

    if reflectivity > 0.0 {
        let reflected = Ray::new(hit.point, incoming.reflect(hit.normal));
        color += trace(scene, &reflected, light, bounces_left - 1) * reflectivity;
    }

    if transparency > 0.0 {
        let eta_ratio = if hit.front_face {
            1.0 / hit.material.refractive_index
        } else {
            hit.material.refractive_index
        };

        let transmitted_direction = match incoming.refract(hit.normal, eta_ratio) {
            Some(direction) => direction,
            // Total internal reflection: no light escapes through the
            // surface, so it bounces back inside instead.
            None => incoming.reflect(hit.normal),
        };
        let transmitted = Ray::new(hit.point, transmitted_direction);
        color += trace(scene, &transmitted, light, bounces_left - 1) * transparency;
    }

    color
}

/// Determines whether `hit` is occluded from `light` by another object in
/// `scene`. A shadow ray is cast from the hit point towards the light; any
/// intersection strictly between the surface (offset by `SHADOW_BIAS`) and
/// the light itself means the point is in shadow.
fn is_in_shadow(scene: &Scene, light: &PointLight, hit: &HitRecord) -> bool {
    let to_light = light.position - hit.point;
    let distance_to_light = to_light.length();
    let shadow_ray = Ray::new(hit.point, to_light.normalized());

    scene
        .hit(&shadow_ray, SHADOW_BIAS, distance_to_light)
        .is_some()
}

#[cfg(test)]
mod tests {
    use crate::{Color, HitRecord, Hittable, Material, Point3, Ray, Vec3};

    use super::*;

    /// A fake `Hittable` representing an infinite plane perpendicular to the
    /// z axis at `z`. Unlike a fake that reports a fixed hit distance for
    /// every ray regardless of its origin or direction, this one computes a
    /// real intersection, so it behaves correctly both as a primary-ray
    /// target and as a potential shadow-ray occluder (including correctly
    /// missing its own shadow ray thanks to `SHADOW_BIAS`).
    struct TestObject {
        z: f64,
        normal: Vec3,
        material: Material,
    }

    impl Hittable for TestObject {
        fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
            if ray.direction.z == 0.0 {
                return None;
            }

            let distance = (self.z - ray.origin.z) / ray.direction.z;
            if distance < min_distance || distance > max_distance {
                return None;
            }

            Some(HitRecord::new(ray, distance, self.normal, self.material))
        }
    }

    fn test_camera(width: usize, height: usize) -> Camera {
        Camera::new(
            Point3::new(0.0, 0.0, 5.0),
            Point3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            width,
            height,
        )
    }

    #[test]
    fn fills_every_pixel_with_the_background_color_when_the_scene_is_empty() {
        let background = Color::new(0.1, 0.2, 0.3);
        let scene = Scene::new(background);
        let camera = test_camera(4, 3);
        let light = PointLight::new(Point3::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0), 1.0);

        let canvas = render(&scene, &camera, &light);

        for y in 0..camera.image_height() {
            for x in 0..camera.image_width() {
                assert_eq!(canvas.pixel(x, y), background);
            }
        }
    }

    #[test]
    fn shades_a_hit_object_using_the_light() {
        let material = Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9);
        let mut scene = Scene::new(Color::zero());
        // A flat surface facing the camera, 4 units along the primary ray.
        scene.add(TestObject {
            z: 1.0,
            normal: Vec3::new(0.0, 0.0, 1.0),
            material,
        });
        let camera = test_camera(3, 3);
        let light = PointLight::new(Point3::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0), 1.0);

        let canvas = render(&scene, &camera, &light);

        let ray = camera.ray_for_pixel(1, 1);
        let hit = HitRecord::new(&ray, 4.0, Vec3::new(0.0, 0.0, 1.0), material);
        let expected = light.shade(&hit, false);

        assert_eq!(canvas.pixel(1, 1), expected);
        assert_ne!(canvas.pixel(1, 1), scene.background());
    }

    #[test]
    fn shadows_a_hit_object_occluded_from_the_light() {
        let material = Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9);
        let mut scene = Scene::new(Color::zero());
        // The lit surface, 4 units along the primary ray from the camera.
        scene.add(TestObject {
            z: 1.0,
            normal: Vec3::new(0.0, 0.0, 1.0),
            material,
        });
        // An occluder sitting on the segment between the lit surface and the
        // light (z = 7, strictly between the surface at z = 1 and the light
        // at z = 10), but behind the camera (z = 5) along the primary ray's
        // direction, so it does not block the camera's view of the surface.
        scene.add(TestObject {
            z: 7.0,
            normal: Vec3::new(0.0, 0.0, -1.0),
            material,
        });
        let camera = test_camera(3, 3);
        let light = PointLight::new(Point3::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0), 1.0);

        let canvas = render(&scene, &camera, &light);

        let ray = camera.ray_for_pixel(1, 1);
        let hit = HitRecord::new(&ray, 4.0, Vec3::new(0.0, 0.0, 1.0), material);
        let expected = light.shade(&hit, true);

        assert_eq!(canvas.pixel(1, 1), expected);
    }

    #[test]
    fn image_dimensions_match_the_camera() {
        let scene = Scene::new(Color::zero());
        let camera = test_camera(7, 5);
        let light = PointLight::new(Point3::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0), 1.0);

        let canvas = render(&scene, &camera, &light);

        assert_eq!(canvas.width(), camera.image_width());
        assert_eq!(canvas.height(), camera.image_height());
    }

    #[test]
    fn a_reflective_surface_shows_the_color_of_what_it_reflects() {
        let mirror_material = Material::new(Color::zero(), 0.0, 0.0).with_reflectivity(1.0);
        let mut scene = Scene::new(Color::new(0.0, 0.0, 0.0));
        // A perfect mirror facing the camera, with a bright red wall behind
        // the camera for it to reflect back.
        scene.add(TestObject {
            z: -1.0,
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: mirror_material,
        });
        scene.add(TestObject {
            z: 10.0,
            normal: Vec3::new(0.0, 0.0, -1.0),
            material: Material::new(Color::new(1.0, 0.0, 0.0), 1.0, 0.0),
        });
        let camera = test_camera(1, 1);
        let light = PointLight::new(
            Point3::new(0.0, 0.0, -100.0),
            Color::new(1.0, 1.0, 1.0),
            0.0,
        );

        let canvas = render(&scene, &camera, &light);

        // A straight-on reflection off a flat mirror bounces the ray right
        // back the way it came, so the mirror should show the red wall.
        assert_eq!(canvas.pixel(0, 0), Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn a_fully_transparent_surface_lets_the_background_show_through() {
        let glass_material = Material::new(Color::zero(), 0.0, 0.0).with_transparency(1.0, 1.0);
        let mut scene = Scene::new(Color::new(0.2, 0.4, 0.6));
        scene.add(TestObject {
            z: -1.0,
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: glass_material,
        });
        let camera = test_camera(1, 1);
        let light = PointLight::new(
            Point3::new(0.0, 0.0, -100.0),
            Color::new(1.0, 1.0, 1.0),
            0.0,
        );

        let canvas = render(&scene, &camera, &light);

        // A refractive index of 1.0 bends light by exactly zero degrees, so
        // the ray passes straight through to the background untouched.
        assert_eq!(canvas.pixel(0, 0), Color::new(0.2, 0.4, 0.6));
    }

    #[test]
    fn an_opaque_material_ignores_reflectivity_and_transparency_entirely() {
        let material = Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9);
        let mut scene = Scene::new(Color::zero());
        scene.add(TestObject {
            z: 1.0,
            normal: Vec3::new(0.0, 0.0, 1.0),
            material,
        });
        let camera = test_camera(1, 1);
        let light = PointLight::new(Point3::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0), 1.0);

        let canvas = render(&scene, &camera, &light);

        let ray = camera.ray_for_pixel(0, 0);
        let hit = HitRecord::new(&ray, 4.0, Vec3::new(0.0, 0.0, 1.0), material);
        assert_eq!(canvas.pixel(0, 0), light.shade(&hit, false));
    }
}

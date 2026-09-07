use crate::{Camera, Canvas, PointLight, Scene};

/// Minimum distance used both when searching for the closest scene hit along
/// a primary ray and as the origin offset for shadow rays. Without this bias
/// a ray can immediately re-intersect the surface it was just cast from due
/// to floating point rounding, producing "shadow acne" artifacts.
const SHADOW_BIAS: f64 = 1e-4;

/// Renders the scene as seen by `camera`, lit by `light`.
///
/// For every pixel a single camera ray is cast into the scene. If it hits
/// an object, the closest hit is shaded using `light`, taking into account
/// whether the hit point is occluded from the light by another object
/// (a hard shadow, with no penumbra). Pixels whose ray hits nothing are
/// filled with the scene's background color.
pub fn render(scene: &Scene, camera: &Camera, light: &PointLight) -> Canvas {
    let mut canvas = Canvas::new(camera.image_width(), camera.image_height());

    for y in 0..camera.image_height() {
        for x in 0..camera.image_width() {
            let ray = camera.ray_for_pixel(x, y);
            let color = match scene.hit(&ray, SHADOW_BIAS, f64::INFINITY) {
                Some(hit) => {
                    let in_shadow = is_in_shadow(scene, light, &hit);
                    light.shade(&hit, in_shadow)
                }
                None => scene.background(),
            };

            canvas.set_pixel(x, y, color);
        }
    }

    canvas
}

/// Determines whether `hit` is occluded from `light` by another object in
/// `scene`. A shadow ray is cast from the hit point towards the light; any
/// intersection strictly between the surface (offset by `SHADOW_BIAS`) and
/// the light itself means the point is in shadow.
fn is_in_shadow(scene: &Scene, light: &PointLight, hit: &crate::HitRecord) -> bool {
    let to_light = light.position - hit.point;
    let distance_to_light = to_light.length();
    let shadow_ray = crate::Ray::new(hit.point, to_light.normalized());

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
}

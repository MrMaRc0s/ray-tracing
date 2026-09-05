use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};

/// A horizontal (y-up) plane whose shading normal ripples like the surface
/// of a liquid, approximating a fluid without the cost of simulating one.
///
/// The intersection itself stays perfectly flat at `height` — only the
/// normal used for shading is perturbed, a cheap bump-mapping trick that
/// gives water its characteristic sparkle and distorted reflections without
/// needing a displaced, more expensive-to-intersect surface.
pub struct WavyPlane {
    height: f64,
    material: Material,
    amplitude: f64,
    wavelength: f64,
}

impl WavyPlane {
    /// `amplitude` controls how far the shading normal tilts away from
    /// straight up (keep it well under `1.0` so the tilted normal stays
    /// physically sensible); `wavelength` controls the distance between
    /// ripple crests.
    pub fn new(height: f64, material: Material, amplitude: f64, wavelength: f64) -> Self {
        assert!(wavelength > 0.0, "wavelength must be greater than zero");

        Self {
            height,
            material,
            amplitude,
            wavelength,
        }
    }

    fn rippled_normal(&self, point: Point3) -> Vec3 {
        let frequency = std::f64::consts::TAU / self.wavelength;
        let tilt_x = self.amplitude * (frequency * point.x).sin();
        let tilt_z = self.amplitude * (frequency * point.z).sin();
        Vec3::new(tilt_x, 1.0, tilt_z).normalized()
    }
}

impl Hittable for WavyPlane {
    fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        if ray.direction.y.abs() < 1e-9 {
            return None;
        }

        let distance = (self.height - ray.origin.y) / ray.direction.y;
        if distance < min_distance || distance > max_distance {
            return None;
        }

        let point = ray.at(distance);
        Some(HitRecord::new(
            ray,
            distance,
            self.rippled_normal(point),
            self.material,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misses_a_ray_parallel_to_the_surface() {
        let water = WavyPlane::new(0.0, Material::default(), 0.1, 2.0);
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        assert!(water.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn hits_the_surface_at_the_configured_height() {
        let water = WavyPlane::new(-1.0, Material::default(), 0.1, 2.0);
        let ray = Ray::new(Point3::new(0.0, 4.0, 0.0), Vec3::new(0.0, -1.0, 0.0));

        let hit = water.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 5.0);
        assert_eq!(hit.point, Point3::new(0.0, -1.0, 0.0));
    }

    #[test]
    fn the_shading_normal_stays_close_to_straight_up() {
        let water = WavyPlane::new(0.0, Material::default(), 0.2, 4.0);
        let ray = Ray::new(Point3::new(1.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));

        let hit = water.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert!((hit.normal.length() - 1.0).abs() < 1e-9);
        assert!(hit.normal.y > 0.9);
    }

    #[test]
    fn the_normal_ripples_across_the_surface_instead_of_staying_flat() {
        let water = WavyPlane::new(0.0, Material::default(), 0.3, 4.0);
        let straight_down = Vec3::new(0.0, -1.0, 0.0);

        let center = water
            .hit(
                &Ray::new(Point3::new(0.0, 5.0, 0.0), straight_down),
                0.001,
                f64::INFINITY,
            )
            .unwrap();
        let offset = water
            .hit(
                &Ray::new(Point3::new(1.0, 5.0, 0.0), straight_down),
                0.001,
                f64::INFINITY,
            )
            .unwrap();

        assert_ne!(center.normal, offset.normal);
    }

    #[test]
    #[should_panic(expected = "wavelength must be greater than zero")]
    fn rejects_a_non_positive_wavelength() {
        WavyPlane::new(0.0, Material::default(), 0.1, 0.0);
    }
}

use crate::{HitRecord, Hittable, Material, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        assert!(radius > 0.0, "radius must be greater than zero");

        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let origin_to_center = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = origin_to_center.dot(ray.direction);
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < min_distance || root > max_distance {
            root = (-half_b + sqrt_discriminant) / a;
            if root < min_distance || root > max_distance {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center).normalized();
        Some(HitRecord::new(ray, root, outward_normal, self.material))
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    use super::*;

    #[test]
    fn misses_a_ray_that_never_comes_close() {
        let sphere = Sphere::new(Point3::zero(), 1.0, Material::default());
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 0.0, 0.0));

        assert!(sphere.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn hits_from_outside_on_the_front_face() {
        let sphere = Sphere::new(Point3::zero(), 1.0, Material::default());
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        let hit = sphere.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 4.0);
        assert_eq!(hit.point, Point3::new(0.0, 0.0, -1.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        assert!(hit.front_face);
    }

    #[test]
    fn returns_the_far_intersection_when_the_ray_starts_inside() {
        let sphere = Sphere::new(Point3::zero(), 1.0, Material::default());
        let ray = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, 1.0));

        let hit = sphere.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 1.0);
        assert_eq!(hit.point, Point3::new(0.0, 0.0, 1.0));
        // The outward normal here is (0, 0, 1), but HitRecord::new flips it to
        // face against the ray since this is a back face (the ray is exiting).
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        assert!(!hit.front_face);
    }

    #[test]
    fn rejects_a_hit_when_neither_root_lies_in_range() {
        let sphere = Sphere::new(Point3::zero(), 1.0, Material::default());
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        // Roots are at distances 4.0 and 6.0; neither fits inside [0.001, 3.0].
        assert!(sphere.hit(&ray, 0.001, 3.0).is_none());
    }

    #[test]
    fn falls_through_to_the_far_root_when_the_near_root_is_out_of_range() {
        let sphere = Sphere::new(Point3::zero(), 1.0, Material::default());
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));

        // The near root (4.0) is below the window, so the far root (6.0) is used.
        let hit = sphere.hit(&ray, 5.0, 10.0).unwrap();

        assert_eq!(hit.distance, 6.0);
        assert_eq!(hit.point, Point3::new(0.0, 0.0, 1.0));
        // The outward normal here is (0, 0, 1), but HitRecord::new flips it to
        // face against the ray since this is a back face (the ray is exiting).
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        assert!(!hit.front_face);
    }

    #[test]
    fn hits_a_translated_sphere_at_its_world_space_point() {
        let sphere = Sphere::new(Point3::new(2.0, 3.0, 4.0), 2.0, Material::default());
        let ray = Ray::new(Point3::new(2.0, 3.0, -6.0), Vec3::new(0.0, 0.0, 1.0));

        let hit = sphere.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 8.0);
        assert_eq!(hit.point, Point3::new(2.0, 3.0, 2.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        assert!(hit.front_face);
    }

    #[test]
    #[should_panic(expected = "radius must be greater than zero")]
    fn rejects_a_non_positive_radius() {
        Sphere::new(Point3::zero(), 0.0, Material::default());
    }
}

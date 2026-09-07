use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};

pub struct Plane {
    point: Point3,
    normal: Vec3,
    material: Material,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, material: Material) -> Self {
        Self {
            point,
            normal: normal.normalized(),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let denom = ray.direction.dot(self.normal);
        if denom.abs() < 1e-9 {
            return None;
        }

        let distance = (self.point - ray.origin).dot(self.normal) / denom;
        if distance < min_distance || distance > max_distance {
            return None;
        }

        Some(HitRecord::new(ray, distance, self.normal, self.material))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misses_a_ray_parallel_to_the_plane() {
        let plane = Plane::new(
            Point3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
            Material::default(),
        );
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        assert!(plane.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn hits_the_plane_from_the_front_side() {
        let plane = Plane::new(
            Point3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
            Material::default(),
        );
        let ray = Ray::new(Point3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));

        let hit = plane.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 5.0);
        assert_eq!(hit.point, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
        assert!(hit.front_face);
    }

    #[test]
    fn hits_the_plane_from_the_back_side() {
        let plane = Plane::new(
            Point3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
            Material::default(),
        );
        let ray = Ray::new(Point3::new(0.0, -5.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

        let hit = plane.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 5.0);
        assert!(!hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, -1.0, 0.0));
    }

    #[test]
    fn ignores_hits_outside_the_requested_distance_range() {
        let plane = Plane::new(
            Point3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
            Material::default(),
        );
        let ray = Ray::new(Point3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));

        assert!(plane.hit(&ray, 0.001, 4.0).is_none());
        assert!(plane.hit(&ray, 6.0, f64::INFINITY).is_none());
    }

    #[test]
    fn hits_a_translated_and_tilted_plane_in_world_space() {
        // Plane through (2.0, 3.0, -1.0) tilted so its normal points along +x and +y.
        let plane = Plane::new(
            Point3::new(2.0, 3.0, -1.0),
            Vec3::new(1.0, 1.0, 0.0),
            Material::default(),
        );
        let ray = Ray::new(Point3::new(6.0, 3.0, -1.0), Vec3::new(-1.0, 0.0, 0.0));

        let hit = plane.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 4.0);
        assert_eq!(hit.point, Point3::new(2.0, 3.0, -1.0));
        assert!(hit.front_face);
    }
}

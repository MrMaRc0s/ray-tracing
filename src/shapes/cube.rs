use std::mem::swap;

use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};

pub struct Cube {
    center: Point3,
    half_size: f64,
    material: Material,
}

impl Cube {
    pub fn new(center: Point3, half_size: f64, material: Material) -> Self {
        assert!(half_size > 0.0, "cube half size must be greater than zero");

        Self {
            center,
            half_size,
            material,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let min_corner = self.center - Vec3::new(self.half_size, self.half_size, self.half_size);
        let max_corner = self.center + Vec3::new(self.half_size, self.half_size, self.half_size);

        // For each axis we intersect the ray against the pair of planes bounding
        // that axis, tracking the largest "entering" distance and the smallest
        // "exiting" distance across all three axes, along with the outward
        // normal of whichever face produced each of those distances.
        let axes = [
            (
                ray.origin.x,
                ray.direction.x,
                min_corner.x,
                max_corner.x,
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            (
                ray.origin.y,
                ray.direction.y,
                min_corner.y,
                max_corner.y,
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            (
                ray.origin.z,
                ray.direction.z,
                min_corner.z,
                max_corner.z,
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
        ];

        let mut entry_distance = f64::NEG_INFINITY;
        let mut exit_distance = f64::INFINITY;
        let mut entry_normal = Vec3::zero();
        let mut exit_normal = Vec3::zero();

        for (origin, direction, slab_min, slab_max, min_normal, max_normal) in axes {
            let inverse_direction = 1.0 / direction;
            let mut near_distance = (slab_min - origin) * inverse_direction;
            let mut far_distance = (slab_max - origin) * inverse_direction;
            let mut near_normal = min_normal;
            let mut far_normal = max_normal;

            if near_distance > far_distance {
                swap(&mut near_distance, &mut far_distance);
                swap(&mut near_normal, &mut far_normal);
            }

            if near_distance > entry_distance {
                entry_distance = near_distance;
                entry_normal = near_normal;
            }
            if far_distance < exit_distance {
                exit_distance = far_distance;
                exit_normal = far_normal;
            }

            if entry_distance > exit_distance {
                return None;
            }
        }

        // A ray starting inside the cube has a negative (behind-the-origin)
        // entry distance; fall back to the exit distance in that case.
        let (distance, outward_normal) =
            if entry_distance >= min_distance && entry_distance <= max_distance {
                (entry_distance, entry_normal)
            } else if exit_distance >= min_distance && exit_distance <= max_distance {
                (exit_distance, exit_normal)
            } else {
                return None;
            };

        Some(HitRecord::new(ray, distance, outward_normal, self.material))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misses_a_cube_it_never_reaches() {
        let cube = Cube::new(Point3::zero(), 1.0, Material::default());
        let ray = Ray::new(Point3::new(10.0, 10.0, 10.0), Vec3::new(1.0, 1.0, 1.0));

        assert!(cube.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn hits_the_near_face_from_outside_along_an_axis() {
        let cube = Cube::new(Point3::zero(), 1.0, Material::default());
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = cube.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 4.0);
        assert_eq!(hit.point, Point3::new(0.0, 0.0, 1.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
        assert!(hit.front_face);
    }

    #[test]
    fn hits_the_exit_face_for_a_ray_starting_inside() {
        let cube = Cube::new(Point3::zero(), 1.0, Material::default());
        let ray = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, 1.0));

        let hit = cube.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 1.0);
        assert_eq!(hit.point, Point3::new(0.0, 0.0, 1.0));
        // HitRecord flips the outward normal to the inward direction when the
        // ray leaves through a face travelling the same way as its normal.
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        assert!(!hit.front_face);
    }

    #[test]
    fn chooses_the_tightest_slab_when_several_axes_could_bound_the_ray() {
        let cube = Cube::new(Point3::zero(), 1.0, Material::default());
        // Travels mostly upward (+y) with a slight +x drift, starting inside
        // the x slab but far below the y slab. The x slab is wide enough that
        // the y slab alone determines the entry and exit points.
        let ray = Ray::new(Point3::new(-0.5, -3.0, 0.0), Vec3::new(0.1, 1.0, 0.0));

        let hit = cube.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 2.0);
        assert_eq!(hit.point, Point3::new(-0.3, -1.0, 0.0));
        assert_eq!(hit.normal, Vec3::new(0.0, -1.0, 0.0));
        assert!(hit.front_face);
    }

    #[test]
    fn hits_a_translated_cube_at_the_correct_world_space_point() {
        let cube = Cube::new(Point3::new(5.0, 3.0, -2.0), 1.0, Material::default());
        let ray = Ray::new(Point3::new(5.0, 3.0, -10.0), Vec3::new(0.0, 0.0, 1.0));

        let hit = cube.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 7.0);
        assert_eq!(hit.point, Point3::new(5.0, 3.0, -3.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        assert!(hit.front_face);
    }

    #[test]
    #[should_panic(expected = "half size must be greater than zero")]
    fn rejects_a_non_positive_half_size() {
        Cube::new(Point3::zero(), 0.0, Material::default());
    }
}

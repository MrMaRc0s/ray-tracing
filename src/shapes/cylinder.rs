use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};

pub struct Cylinder {
    base_center: Point3,
    radius: f64,
    height: f64,
    material: Material,
}

impl Cylinder {
    pub fn new(base_center: Point3, radius: f64, height: f64, material: Material) -> Self {
        assert!(radius > 0.0, "cylinder radius must be greater than zero");
        assert!(height > 0.0, "cylinder height must be greater than zero");

        Self {
            base_center,
            radius,
            height,
            material,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let mut closest_distance = max_distance;
        let mut closest: Option<(f64, Vec3)> = None;

        // Curved side: solve the quadratic for the infinite tube in the X/Z plane,
        // then keep only the roots that land within the cylinder's finite height.
        let ocx = ray.origin.x - self.base_center.x;
        let ocz = ray.origin.z - self.base_center.z;
        let dx = ray.direction.x;
        let dz = ray.direction.z;
        let a = dx * dx + dz * dz;

        if a.abs() >= 1e-9 {
            let b = 2.0 * (dx * ocx + dz * ocz);
            let c = ocx * ocx + ocz * ocz - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;

            if discriminant >= 0.0 {
                let sqrt_discriminant = discriminant.sqrt();
                let roots = [
                    (-b - sqrt_discriminant) / (2.0 * a),
                    (-b + sqrt_discriminant) / (2.0 * a),
                ];

                for root in roots {
                    if root < min_distance || root > closest_distance {
                        continue;
                    }

                    let hit_point = ray.at(root);
                    if hit_point.y < self.base_center.y
                        || hit_point.y > self.base_center.y + self.height
                    {
                        continue;
                    }

                    let outward_normal = Vec3::new(
                        hit_point.x - self.base_center.x,
                        0.0,
                        hit_point.z - self.base_center.z,
                    )
                    .normalized();

                    closest_distance = root;
                    closest = Some((root, outward_normal));
                }
            }
        }

        // End caps: intersect the ray with each cap's plane, then keep the hit only
        // if it falls within the cap's disc.
        if ray.direction.y.abs() >= 1e-9 {
            let caps = [
                (self.base_center.y, Vec3::new(0.0, -1.0, 0.0)),
                (self.base_center.y + self.height, Vec3::new(0.0, 1.0, 0.0)),
            ];

            for (cap_y, outward_normal) in caps {
                let distance = (cap_y - ray.origin.y) / ray.direction.y;
                if distance < min_distance || distance > closest_distance {
                    continue;
                }

                let hit_point = ray.at(distance);
                let radial_dx = hit_point.x - self.base_center.x;
                let radial_dz = hit_point.z - self.base_center.z;
                if radial_dx * radial_dx + radial_dz * radial_dz > self.radius * self.radius {
                    continue;
                }

                closest_distance = distance;
                closest = Some((distance, outward_normal));
            }
        }

        closest.map(|(distance, normal)| HitRecord::new(ray, distance, normal, self.material))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misses_a_cylinder_passed_entirely_outside_its_radius() {
        let cylinder = Cylinder::new(Point3::zero(), 1.0, 2.0, Material::default());
        let ray = Ray::new(Point3::new(5.0, -10.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

        assert!(cylinder.hit(&ray, 0.001, f64::INFINITY).is_none());
    }

    #[test]
    fn hits_the_curved_side_from_outside() {
        let cylinder = Cylinder::new(Point3::zero(), 1.0, 2.0, Material::default());
        let ray = Ray::new(Point3::new(5.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));

        let hit = cylinder.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 4.0);
        assert_eq!(hit.point, Point3::new(1.0, 1.0, 0.0));
        assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));
        assert!(hit.front_face);
    }

    #[test]
    fn hits_the_top_cap_straight_down() {
        let cylinder = Cylinder::new(Point3::zero(), 1.0, 2.0, Material::default());
        let ray = Ray::new(Point3::new(0.3, 5.0, 0.4), Vec3::new(0.0, -1.0, 0.0));

        let hit = cylinder.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 3.0);
        assert_eq!(hit.point, Point3::new(0.3, 2.0, 0.4));
        assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
        assert!(hit.front_face);
    }

    #[test]
    fn hits_the_bottom_cap_from_below() {
        let cylinder = Cylinder::new(Point3::zero(), 1.0, 2.0, Material::default());
        let ray = Ray::new(Point3::new(0.2, -5.0, 0.1), Vec3::new(0.0, 1.0, 0.0));

        let hit = cylinder.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 5.0);
        assert_eq!(hit.point, Point3::new(0.2, 0.0, 0.1));
        assert_eq!(hit.normal, Vec3::new(0.0, -1.0, 0.0));
        assert!(hit.front_face);
    }

    #[test]
    fn hits_an_exit_surface_from_a_ray_starting_inside() {
        let cylinder = Cylinder::new(Point3::zero(), 1.0, 2.0, Material::default());
        let ray = Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

        let hit = cylinder.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 1.0);
        assert_eq!(hit.point, Point3::new(1.0, 1.0, 0.0));
        assert!(!hit.front_face);
        assert_eq!(hit.normal, Vec3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn hits_a_translated_cylinder_in_world_space() {
        let cylinder = Cylinder::new(Point3::new(3.0, 4.0, -2.0), 1.0, 2.0, Material::default());
        let ray = Ray::new(Point3::new(8.0, 5.0, -2.0), Vec3::new(-1.0, 0.0, 0.0));

        let hit = cylinder.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 4.0);
        assert_eq!(hit.point, Point3::new(4.0, 5.0, -2.0));
        assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));
        assert!(hit.front_face);
    }

    #[test]
    fn ignores_hits_outside_the_requested_distance_range() {
        let cylinder = Cylinder::new(Point3::zero(), 1.0, 2.0, Material::default());
        let ray = Ray::new(Point3::new(5.0, 1.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));

        assert!(cylinder.hit(&ray, 0.001, 3.0).is_none());
        assert!(cylinder.hit(&ray, 6.5, f64::INFINITY).is_none());
    }
}

use crate::{Material, Point3, Ray, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord {
    pub distance: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(ray: &Ray, distance: f64, outward_normal: Vec3, material: Material) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            distance,
            point: ray.at(distance),
            normal,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_an_outward_normal_for_a_front_face() {
        let ray = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, -1.0));
        let hit = HitRecord::new(&ray, 2.0, Vec3::new(0.0, 0.0, 1.0), Material::default());

        assert!(hit.front_face);
        assert_eq!(hit.point, Point3::new(0.0, 0.0, -2.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn flips_an_outward_normal_for_a_back_face() {
        let ray = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, 1.0));
        let hit = HitRecord::new(&ray, 1.0, Vec3::new(0.0, 0.0, 1.0), Material::default());

        assert!(!hit.front_face);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
    }
}

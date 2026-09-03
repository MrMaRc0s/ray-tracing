use crate::{Color, HitRecord, Hittable, Ray};

pub struct Scene {
    background: Color,
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(background: Color) -> Self {
        Self {
            background,
            objects: Vec::new(),
        }
    }

    pub const fn background(&self) -> Color {
        self.background
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        let mut closest_distance = max_distance;
        let mut closest_hit = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, min_distance, closest_distance) {
                closest_distance = hit.distance;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}

#[cfg(test)]
mod tests {
    use crate::{Material, Point3, Vec3};

    use super::*;

    struct TestObject {
        distance: f64,
        material: Material,
    }

    impl Hittable for TestObject {
        fn hit(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
            if self.distance < min_distance || self.distance > max_distance {
                return None;
            }

            Some(HitRecord::new(
                ray,
                self.distance,
                Vec3::new(0.0, 0.0, 1.0),
                self.material,
            ))
        }
    }

    #[test]
    fn adds_objects_to_the_scene() {
        let mut scene = Scene::new(Color::zero());
        assert!(scene.is_empty());

        scene.add(TestObject {
            distance: 1.0,
            material: Material::default(),
        });

        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn returns_the_closest_object_hit() {
        let near_material = Material::new(Color::new(1.0, 0.0, 0.0), 0.1, 0.9);
        let far_material = Material::new(Color::new(0.0, 0.0, 1.0), 0.1, 0.9);
        let mut scene = Scene::new(Color::zero());
        scene.add(TestObject {
            distance: 4.0,
            material: far_material,
        });
        scene.add(TestObject {
            distance: 2.0,
            material: near_material,
        });
        let ray = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, -1.0));

        let hit = scene.hit(&ray, 0.001, f64::INFINITY).unwrap();

        assert_eq!(hit.distance, 2.0);
        assert_eq!(hit.material, near_material);
    }

    #[test]
    fn ignores_hits_outside_the_requested_distance_range() {
        let mut scene = Scene::new(Color::new(0.1, 0.2, 0.3));
        scene.add(TestObject {
            distance: 5.0,
            material: Material::default(),
        });
        let ray = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, -1.0));

        assert!(scene.hit(&ray, 0.001, 4.0).is_none());
        assert_eq!(scene.background(), Color::new(0.1, 0.2, 0.3));
    }
}

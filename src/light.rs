use crate::{Color, HitRecord, Point3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
    pub position: Point3,
    pub color: Color,
    brightness: f64,
}

impl PointLight {
    pub fn new(position: Point3, color: Color, brightness: f64) -> Self {
        assert!(brightness >= 0.0, "light brightness cannot be negative");

        Self {
            position,
            color,
            brightness,
        }
    }

    pub const fn brightness(&self) -> f64 {
        self.brightness
    }

    pub fn set_brightness(&mut self, brightness: f64) {
        assert!(brightness >= 0.0, "light brightness cannot be negative");
        self.brightness = brightness;
    }

    pub fn shade(&self, hit: &HitRecord, in_shadow: bool) -> Color {
        let ambient = hit.material.color * hit.material.ambient;
        if in_shadow {
            return ambient;
        }

        let direction_to_light = (self.position - hit.point).normalized();
        let light_angle = hit.normal.dot(direction_to_light).max(0.0);
        let diffuse_strength = hit.material.diffuse * self.brightness * light_angle;
        ambient + hit.material.color * self.color * diffuse_strength
    }
}

#[cfg(test)]
mod tests {
    use crate::{Material, Ray, Vec3};

    use super::*;

    fn front_facing_hit() -> HitRecord {
        let ray = Ray::new(Point3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0));
        HitRecord::new(
            &ray,
            1.0,
            Vec3::new(0.0, 0.0, 1.0),
            Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9),
        )
    }

    #[test]
    fn lights_a_surface_facing_the_light() {
        let light = PointLight::new(Point3::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0), 1.0);

        assert_eq!(
            light.shade(&front_facing_hit(), false),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn lower_brightness_produces_a_darker_surface() {
        let mut light =
            PointLight::new(Point3::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0), 1.0);
        light.set_brightness(0.5);

        assert_eq!(light.brightness(), 0.5);
        assert_eq!(
            light.shade(&front_facing_hit(), false),
            Color::new(0.55, 0.55, 0.55)
        );
    }

    #[test]
    fn shadowed_surface_only_receives_ambient_light() {
        let light = PointLight::new(Point3::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0), 1.0);

        assert_eq!(
            light.shade(&front_facing_hit(), true),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn light_behind_the_surface_adds_no_diffuse_color() {
        let light = PointLight::new(Point3::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0), 1.0);

        assert_eq!(
            light.shade(&front_facing_hit(), false),
            Color::new(0.1, 0.1, 0.1)
        );
    }
}

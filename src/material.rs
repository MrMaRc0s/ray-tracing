use crate::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
}

impl Material {
    pub const fn new(color: Color, ambient: f64, diffuse: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_material_with_the_requested_surface_properties() {
        let material = Material::new(Color::new(0.2, 0.4, 0.8), 0.15, 0.7);

        assert_eq!(material.color, Color::new(0.2, 0.4, 0.8));
        assert_eq!(material.ambient, 0.15);
        assert_eq!(material.diffuse, 0.7);
    }
}

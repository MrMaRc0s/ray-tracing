use crate::{Color, Texture};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub texture: Texture,
    /// Fraction of light that bounces off as a mirror reflection, `0.0` to
    /// `1.0`. `0.0` (the default) means a fully matte surface.
    pub reflectivity: f64,
    /// Fraction of light that passes through the surface and refracts,
    /// `0.0` to `1.0`. `0.0` (the default) means a fully opaque surface.
    /// Reflectivity and transparency share the remaining local-shading
    /// weight, so `reflectivity + transparency` should not exceed `1.0`.
    pub transparency: f64,
    /// Index of refraction used when `transparency > 0.0` (e.g. `1.5` for
    /// glass, `1.33` for water). Ignored on an opaque surface.
    pub refractive_index: f64,
}

impl Material {
    pub const fn new(color: Color, ambient: f64, diffuse: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            texture: Texture::Solid(color),
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }

    /// Returns this material with `texture` applied in place of its solid
    /// color for shading purposes.
    pub const fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = texture;
        self
    }

    /// Returns this material as a mirror-like reflector of the given
    /// strength (`0.0` to `1.0`).
    pub const fn with_reflectivity(mut self, reflectivity: f64) -> Self {
        self.reflectivity = reflectivity;
        self
    }

    /// Returns this material as a transparent, refractive dielectric of the
    /// given strength (`0.0` to `1.0`) and refractive index.
    pub const fn with_transparency(mut self, transparency: f64, refractive_index: f64) -> Self {
        self.transparency = transparency;
        self.refractive_index = refractive_index;
        self
    }

    /// The color actually used for shading at `point`: the material's
    /// texture if one was set, otherwise its plain `color`.
    pub fn surface_color(&self, point: crate::Point3) -> Color {
        self.texture.color_at(point)
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

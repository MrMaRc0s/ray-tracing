use crate::{Color, Point3};

/// A surface's color pattern, sampled at a world-space hit point.
///
/// Patterns are procedural (computed from world-space coordinates) rather
/// than image-based, keeping the ray tracer dependency-free — no image
/// decoder is needed to add a texture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Texture {
    /// A single uniform color, ignoring the hit point entirely.
    Solid(Color),
    /// A 3D checkerboard: `primary` and `secondary` alternate every
    /// `1.0 / scale` world units along each axis. Using the point's world
    /// coordinates (rather than a per-shape UV mapping) means the same
    /// pattern applies unmodified to spheres, cubes, planes, and cylinders.
    Checker {
        primary: Color,
        secondary: Color,
        scale: f64,
    },
}

impl Texture {
    pub const fn solid(color: Color) -> Self {
        Self::Solid(color)
    }

    pub const fn checker(primary: Color, secondary: Color, scale: f64) -> Self {
        Self::Checker {
            primary,
            secondary,
            scale,
        }
    }

    pub fn color_at(&self, point: Point3) -> Color {
        match *self {
            Self::Solid(color) => color,
            Self::Checker {
                primary,
                secondary,
                scale,
            } => {
                let parity = (point.x * scale).floor() as i64
                    + (point.y * scale).floor() as i64
                    + (point.z * scale).floor() as i64;
                if parity.rem_euclid(2) == 0 {
                    primary
                } else {
                    secondary
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_solid_texture_ignores_the_hit_point() {
        let texture = Texture::solid(Color::new(0.2, 0.4, 0.6));

        assert_eq!(
            texture.color_at(Point3::new(100.0, -3.0, 42.0)),
            Color::new(0.2, 0.4, 0.6)
        );
    }

    #[test]
    fn a_checker_texture_alternates_between_adjacent_cells() {
        let texture = Texture::checker(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0), 1.0);

        assert_eq!(
            texture.color_at(Point3::new(0.5, 0.0, 0.5)),
            Color::new(1.0, 1.0, 1.0)
        );
        assert_eq!(
            texture.color_at(Point3::new(1.5, 0.0, 0.5)),
            Color::new(0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn checker_scale_controls_cell_size() {
        let texture = Texture::checker(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0), 0.5);

        // At scale 0.5, cells are 2 world units wide, so x = 1.5 is still in
        // the same cell as the origin.
        assert_eq!(
            texture.color_at(Point3::new(1.5, 0.0, 0.0)),
            texture.color_at(Point3::zero())
        );
    }

    #[test]
    fn checker_handles_negative_coordinates_without_gaps() {
        let texture = Texture::checker(Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0), 1.0);

        assert_eq!(
            texture.color_at(Point3::new(-0.5, 0.0, 0.5)),
            Color::new(0.0, 0.0, 0.0)
        );
    }
}

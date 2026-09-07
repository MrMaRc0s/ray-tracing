use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalized(self) -> Self {
        let length = self.length();
        assert!(length > 0.0, "cannot normalize a zero-length vector");
        self / length
    }

    /// Reflects `self` (an incoming direction) across a surface with the
    /// given `normal`, as if bouncing off a mirror. Works whether `normal`
    /// points toward or away from `self`.
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (2.0 * self.dot(normal))
    }

    /// Refracts `self` (a normalized incoming direction hitting a surface
    /// whose normal opposes it, per [`crate::HitRecord`]'s convention)
    /// through a boundary with refractive-index ratio `eta_ratio` (incident
    /// side over transmitted side), per Snell's law. Returns `None` when the
    /// angle is too shallow for the ray to cross the boundary at all (total
    /// internal reflection), leaving the caller to reflect instead.
    pub fn refract(self, normal: Self, eta_ratio: f64) -> Option<Self> {
        let cos_theta = (-self).dot(normal).min(1.0);
        let sin_theta_squared = (1.0 - cos_theta * cos_theta).max(0.0);
        if eta_ratio * eta_ratio * sin_theta_squared > 1.0 {
            return None;
        }

        let out_perpendicular = (self + normal * cos_theta) * eta_ratio;
        let out_parallel = -normal * (1.0 - out_perpendicular.length_squared()).max(0.0).sqrt();
        Some(out_perpendicular + out_parallel)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        vector * self
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        self * (1.0 / scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-12;

    fn assert_vec3_close(actual: Vec3, expected: Vec3) {
        assert!((actual.x - expected.x).abs() < EPSILON);
        assert!((actual.y - expected.y).abs() < EPSILON);
        assert!((actual.z - expected.z).abs() < EPSILON);
    }

    #[test]
    fn adds_and_subtracts_vectors() {
        let left = Vec3::new(1.0, 2.0, 3.0);
        let right = Vec3::new(-2.0, 0.5, 4.0);

        assert_eq!(left + right, Vec3::new(-1.0, 2.5, 7.0));
        assert_eq!(left - right, Vec3::new(3.0, 1.5, -1.0));
    }

    #[test]
    fn calculates_dot_and_cross_products() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);

        assert_eq!(x.dot(y), 0.0);
        assert_eq!(x.cross(y), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normalizes_a_vector() {
        let normalized = Vec3::new(3.0, 0.0, 4.0).normalized();

        assert_vec3_close(normalized, Vec3::new(0.6, 0.0, 0.8));
        assert!((normalized.length() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn reflects_a_ray_off_a_flat_surface_like_a_mirror() {
        let incoming = Vec3::new(1.0, -1.0, 0.0).normalized();
        let normal = Vec3::new(0.0, 1.0, 0.0);

        let reflected = incoming.reflect(normal);

        assert_vec3_close(reflected, Vec3::new(1.0, 1.0, 0.0).normalized());
    }

    #[test]
    fn refracts_straight_through_a_surface_hit_head_on() {
        let incoming = Vec3::new(0.0, -1.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);

        let refracted = incoming.refract(normal, 1.0 / 1.5).unwrap();

        assert_vec3_close(refracted, Vec3::new(0.0, -1.0, 0.0));
    }

    #[test]
    fn bends_a_ray_entering_a_denser_medium_at_an_angle() {
        let incoming = Vec3::new(1.0, -1.0, 0.0).normalized();
        let normal = Vec3::new(0.0, 1.0, 0.0);

        let refracted = incoming.refract(normal, 1.0 / 1.5).unwrap();

        // Bending toward the normal in a denser medium narrows the angle, so
        // the transmitted ray's x component shrinks relative to the incoming
        // ray while it keeps travelling in the same downward half-space.
        assert!(refracted.x.abs() < incoming.x.abs());
        assert!(refracted.y < 0.0);
        assert!((refracted.length() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn reports_total_internal_reflection_past_the_critical_angle() {
        let grazing = Vec3::new(0.999, -0.05, 0.0).normalized();
        let normal = Vec3::new(0.0, 1.0, 0.0);

        // Going from a denser medium (1.5) back out to air (1.0) at a steep
        // angle exceeds the critical angle, so no refracted ray exists.
        assert!(grazing.refract(normal, 1.5).is_none());
    }
}

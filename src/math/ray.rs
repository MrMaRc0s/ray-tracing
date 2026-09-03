use super::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, distance: f64) -> Point3 {
        self.origin + self.direction * distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_a_point_along_the_ray() {
        let ray = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(0.0, -1.0, 2.0));

        assert_eq!(ray.at(2.5), Point3::new(1.0, -0.5, 8.0));
    }
}

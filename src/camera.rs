use crate::{Point3, Ray, Vec3};

pub struct Camera {
    position: Point3,
    pixel_origin: Point3,
    pixel_step_x: Vec3,
    pixel_step_y: Vec3,
    image_width: usize,
    image_height: usize,
}

impl Camera {
    pub fn new(
        position: Point3,
        look_at: Point3,
        world_up: Vec3,
        vertical_fov_degrees: f64,
        image_width: usize,
        image_height: usize,
    ) -> Self {
        assert!(image_width > 0, "image width must be greater than zero");
        assert!(image_height > 0, "image height must be greater than zero");
        assert!(
            vertical_fov_degrees > 0.0 && vertical_fov_degrees < 180.0,
            "vertical field of view must be between 0 and 180 degrees"
        );

        let backwards = (position - look_at).normalized();
        let right = world_up.cross(backwards).normalized();
        let up = backwards.cross(right);

        let focal_length = 1.0;
        let viewport_height = 2.0 * (vertical_fov_degrees.to_radians() / 2.0).tan();
        let aspect_ratio = image_width as f64 / image_height as f64;
        let viewport_width = viewport_height * aspect_ratio;
        let viewport_x = viewport_width * right;
        let viewport_y = -viewport_height * up;
        let pixel_step_x = viewport_x / image_width as f64;
        let pixel_step_y = viewport_y / image_height as f64;
        let viewport_top_left =
            position - focal_length * backwards - viewport_x / 2.0 - viewport_y / 2.0;
        let pixel_origin = viewport_top_left + (pixel_step_x + pixel_step_y) / 2.0;

        Self {
            position,
            pixel_origin,
            pixel_step_x,
            pixel_step_y,
            image_width,
            image_height,
        }
    }

    pub const fn image_width(&self) -> usize {
        self.image_width
    }

    pub const fn image_height(&self) -> usize {
        self.image_height
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        assert!(
            x < self.image_width,
            "pixel x coordinate is outside the image"
        );
        assert!(
            y < self.image_height,
            "pixel y coordinate is outside the image"
        );

        let pixel_center =
            self.pixel_origin + x as f64 * self.pixel_step_x + y as f64 * self.pixel_step_y;
        Ray::new(self.position, (pixel_center - self.position).normalized())
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
    fn center_pixel_points_at_the_camera_target() {
        let camera = Camera::new(
            Point3::new(0.0, 0.0, 2.0),
            Point3::zero(),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            3,
            3,
        );

        let ray = camera.ray_for_pixel(1, 1);

        assert_eq!(ray.origin, Point3::new(0.0, 0.0, 2.0));
        assert_vec3_close(ray.direction, Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn moving_the_camera_changes_the_ray_origin_and_direction() {
        let position = Point3::new(2.0, 1.0, 3.0);
        let target = Point3::zero();
        let camera = Camera::new(position, target, Vec3::new(0.0, 1.0, 0.0), 45.0, 5, 5);

        let ray = camera.ray_for_pixel(2, 2);

        assert_eq!(ray.origin, position);
        assert_vec3_close(ray.direction, (target - position).normalized());
    }
}

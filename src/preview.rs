use crate::{Camera, Canvas, Color, RenderOptions, Vec3};

pub fn render_preview(options: &RenderOptions) -> Result<Canvas, String> {
    if options.scene != "preview" {
        return Err(format!(
            "scene '{}' is not available yet; use --scene preview",
            options.scene
        ));
    }

    let view_direction = options.look_at - options.camera_position;
    if view_direction.length_squared() == 0.0 {
        return Err("camera position and target must be different".to_owned());
    }

    let world_up = Vec3::new(0.0, 1.0, 0.0);
    if world_up.cross(view_direction).length_squared() < 1e-12 {
        return Err("camera direction cannot be parallel to world up".to_owned());
    }

    let camera = Camera::new(
        options.camera_position,
        options.look_at,
        world_up,
        60.0,
        options.width,
        options.height,
    );
    let mut canvas = Canvas::new(options.width, options.height);

    for y in 0..options.height {
        for x in 0..options.width {
            let ray = camera.ray_for_pixel(x, y);
            canvas.set_pixel(x, y, direction_color(ray.direction));
        }
    }

    Ok(canvas)
}

fn direction_color(direction: Vec3) -> Color {
    Color::new(
        (direction.x + 1.0) * 0.5,
        (direction.y + 1.0) * 0.5,
        (-direction.z + 1.0) * 0.5,
    )
}

#[cfg(test)]
mod tests {
    use crate::Point3;

    use super::*;

    #[test]
    fn preview_uses_the_requested_resolution() {
        let options = RenderOptions {
            width: 8,
            height: 6,
            ..RenderOptions::default()
        };

        let canvas = render_preview(&options).unwrap();

        assert_eq!(canvas.width(), 8);
        assert_eq!(canvas.height(), 6);
    }

    #[test]
    fn moving_the_camera_changes_the_preview() {
        let first = RenderOptions {
            width: 3,
            height: 3,
            ..RenderOptions::default()
        };
        let second = RenderOptions {
            width: 3,
            height: 3,
            camera_position: Point3::new(2.0, 1.0, 3.0),
            ..RenderOptions::default()
        };

        let first_center = render_preview(&first).unwrap().pixel(1, 1);
        let second_center = render_preview(&second).unwrap().pixel(1, 1);

        assert_ne!(first_center, second_center);
    }

    #[test]
    fn reports_an_unavailable_scene() {
        let options = RenderOptions {
            scene: "sphere".to_owned(),
            ..RenderOptions::default()
        };

        assert_eq!(
            render_preview(&options).err().unwrap(),
            "scene 'sphere' is not available yet; use --scene preview"
        );
    }

    #[test]
    fn rejects_a_camera_placed_on_its_target() {
        let options = RenderOptions {
            camera_position: Point3::zero(),
            look_at: Point3::zero(),
            ..RenderOptions::default()
        };

        assert_eq!(
            render_preview(&options).err().unwrap(),
            "camera position and target must be different"
        );
    }
}

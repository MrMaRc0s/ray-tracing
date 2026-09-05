use rt::{build_scene, render, scenes, Camera, Point3, Vec3};

fn camera_for(scene: &str, width: usize, height: usize) -> Camera {
    let (position, look_at) = match scene {
        "sphere" => (Point3::new(0.0, 0.0, 4.0), Point3::new(0.0, 0.0, -1.0)),
        "dim" => (Point3::new(0.0, 2.0, 5.0), Point3::new(0.0, -0.7, -1.0)),
        "all" => (Point3::new(0.0, 3.0, 10.0), Point3::new(0.0, -0.5, -1.0)),
        "textures" => (Point3::new(0.0, 1.0, 4.0), Point3::new(0.0, -0.3, -1.0)),
        "glass" => (Point3::new(0.0, 1.0, 6.0), Point3::new(0.0, -0.2, -1.0)),
        "particles" => (Point3::new(0.0, 1.0, 6.0), Point3::new(0.0, 0.2, -1.5)),
        "fluid" => (Point3::new(0.0, 1.5, 6.0), Point3::new(0.0, -0.4, -1.5)),
        other => panic!("no camera fixture for scene '{other}'"),
    };

    Camera::new(
        position,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        width,
        height,
    )
}

#[test]
fn every_named_scene_renders_at_a_small_resolution() {
    for name in scenes::NAMES {
        let (scene, light) = build_scene(name).unwrap();
        let camera = camera_for(name, 20, 15);

        let canvas = render(&scene, &camera, &light);

        assert_eq!(canvas.width(), 20);
        assert_eq!(canvas.height(), 15);
    }
}

#[test]
fn every_named_scene_shows_at_least_one_object_against_the_background() {
    for name in scenes::NAMES {
        let (scene, light) = build_scene(name).unwrap();
        let camera = camera_for(name, 20, 15);
        let background = scene.background();

        let canvas = render(&scene, &camera, &light);

        let shows_an_object = (0..canvas.height())
            .flat_map(|y| (0..canvas.width()).map(move |x| (x, y)))
            .any(|(x, y)| canvas.pixel(x, y) != background);
        assert!(shows_an_object, "scene '{name}' rendered only background");
    }
}

#[test]
fn dim_scene_is_darker_than_the_all_objects_scene_at_the_same_pixel() {
    let (dim_scene, dim_light) = build_scene("dim").unwrap();
    let (all_scene, all_light) = build_scene("all").unwrap();
    let camera = camera_for("dim", 20, 15);

    let dim_canvas = render(&dim_scene, &camera, &dim_light);
    let all_canvas = render(&all_scene, &camera, &all_light);

    let dim_pixel = dim_canvas.pixel(10, 10);
    let all_pixel = all_canvas.pixel(10, 10);
    assert!(dim_pixel.x <= all_pixel.x && dim_pixel.y <= all_pixel.y && dim_pixel.z <= all_pixel.z);
}

#[test]
fn rejects_an_unknown_scene_name() {
    assert!(build_scene("does-not-exist").is_err());
}

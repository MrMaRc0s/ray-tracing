use std::{
    io::{self, BufWriter},
    process,
};

use rt::{build_scene, render, render_preview, Camera, Canvas, CliCommand, RenderOptions, Vec3};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        eprintln!("try 'rt --help' for usage");
        process::exit(2);
    }
}

fn run() -> Result<(), String> {
    match CliCommand::from_env()? {
        CliCommand::Help => {
            print!("{}", rt::cli::HELP);
            Ok(())
        }
        CliCommand::Render(options) => {
            let canvas = render_canvas(&options)?;
            let stdout = io::stdout();
            let mut output = BufWriter::new(stdout.lock());
            canvas
                .write_ppm(&mut output)
                .map_err(|error| format!("failed to write PPM image: {error}"))
        }
    }
}

fn render_canvas(options: &RenderOptions) -> Result<Canvas, String> {
    if options.scene == "preview" {
        return render_preview(options);
    }

    let (scene, light) = build_scene(&options.scene)?;
    let camera = build_camera(options)?;
    Ok(render(&scene, &camera, &light))
}

fn build_camera(options: &RenderOptions) -> Result<Camera, String> {
    let view_direction = options.look_at - options.camera_position;
    if view_direction.length_squared() == 0.0 {
        return Err("camera position and target must be different".to_owned());
    }

    let world_up = Vec3::new(0.0, 1.0, 0.0);
    if world_up.cross(view_direction).length_squared() < 1e-12 {
        return Err("camera direction cannot be parallel to world up".to_owned());
    }

    Ok(Camera::new(
        options.camera_position,
        options.look_at,
        world_up,
        60.0,
        options.width,
        options.height,
    ))
}

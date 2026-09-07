use std::env;

use crate::Point3;

pub const HELP: &str = "\
Usage: rt [OPTIONS]

Options:
  --scene <NAME>       Scene name: preview, sphere, dim, all, textures,
                       glass, particles, fluid (default: preview)
  --width <PIXELS>     Image width (default: 800)
  --height <PIXELS>    Image height (default: 600)
  --camera <X,Y,Z>     Camera position (default: 0,0,2)
  --look-at <X,Y,Z>    Camera target (default: 0,0,0)
  -h, --help            Print this help

The PPM image is written to standard output.
";

#[derive(Clone, Debug, PartialEq)]
pub struct RenderOptions {
    pub scene: String,
    pub width: usize,
    pub height: usize,
    pub camera_position: Point3,
    pub look_at: Point3,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            scene: "preview".to_owned(),
            width: 800,
            height: 600,
            camera_position: Point3::new(0.0, 0.0, 2.0),
            look_at: Point3::zero(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CliCommand {
    Render(RenderOptions),
    Help,
}

impl CliCommand {
    pub fn from_env() -> Result<Self, String> {
        Self::parse(env::args().skip(1))
    }

    pub fn parse<I, S>(arguments: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut options = RenderOptions::default();
        let mut arguments = arguments
            .into_iter()
            .map(|argument| argument.as_ref().to_owned());

        while let Some(argument) = arguments.next() {
            match argument.as_str() {
                "-h" | "--help" => return Ok(Self::Help),
                "--scene" => {
                    options.scene = next_value(&mut arguments, "--scene")?;
                    if options.scene.is_empty() {
                        return Err("--scene cannot be empty".to_owned());
                    }
                }
                "--width" => {
                    let value = next_value(&mut arguments, "--width")?;
                    options.width = parse_dimension(&value, "--width")?;
                }
                "--height" => {
                    let value = next_value(&mut arguments, "--height")?;
                    options.height = parse_dimension(&value, "--height")?;
                }
                "--camera" => {
                    let value = next_value(&mut arguments, "--camera")?;
                    options.camera_position = parse_point(&value, "--camera")?;
                }
                "--look-at" => {
                    let value = next_value(&mut arguments, "--look-at")?;
                    options.look_at = parse_point(&value, "--look-at")?;
                }
                _ => return Err(format!("unknown option: {argument}")),
            }
        }

        Ok(Self::Render(options))
    }
}

fn next_value(
    arguments: &mut impl Iterator<Item = String>,
    option: &str,
) -> Result<String, String> {
    arguments
        .next()
        .ok_or_else(|| format!("missing value for {option}"))
}

fn parse_dimension(value: &str, option: &str) -> Result<usize, String> {
    let dimension = value
        .parse::<usize>()
        .map_err(|_| format!("{option} must be a positive whole number"))?;

    if dimension == 0 {
        return Err(format!("{option} must be greater than zero"));
    }

    Ok(dimension)
}

fn parse_point(value: &str, option: &str) -> Result<Point3, String> {
    let mut components = value.split(',');
    let x = parse_component(components.next(), option)?;
    let y = parse_component(components.next(), option)?;
    let z = parse_component(components.next(), option)?;

    if components.next().is_some() {
        return Err(format!("{option} must use the format X,Y,Z"));
    }

    Ok(Point3::new(x, y, z))
}

fn parse_component(component: Option<&str>, option: &str) -> Result<f64, String> {
    let component = component.ok_or_else(|| format!("{option} must use the format X,Y,Z"))?;
    let number = component
        .parse::<f64>()
        .map_err(|_| format!("{option} must use the format X,Y,Z"))?;

    if !number.is_finite() {
        return Err(format!("{option} coordinates must be finite numbers"));
    }

    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_auditor_friendly_defaults() {
        assert_eq!(
            CliCommand::parse(Vec::<String>::new()).unwrap(),
            CliCommand::Render(RenderOptions::default())
        );
        assert_eq!(RenderOptions::default().width, 800);
        assert_eq!(RenderOptions::default().height, 600);
    }

    #[test]
    fn parses_render_and_camera_options() {
        let command = CliCommand::parse([
            "--scene",
            "all",
            "--width",
            "320",
            "--height",
            "240",
            "--camera",
            "2,1,3",
            "--look-at",
            "0,0.5,-1",
        ])
        .unwrap();

        assert_eq!(
            command,
            CliCommand::Render(RenderOptions {
                scene: "all".to_owned(),
                width: 320,
                height: 240,
                camera_position: Point3::new(2.0, 1.0, 3.0),
                look_at: Point3::new(0.0, 0.5, -1.0),
            })
        );
    }

    #[test]
    fn returns_help_without_rendering() {
        assert_eq!(CliCommand::parse(["--help"]).unwrap(), CliCommand::Help);
    }

    #[test]
    fn rejects_zero_sized_images() {
        assert_eq!(
            CliCommand::parse(["--width", "0"]).unwrap_err(),
            "--width must be greater than zero"
        );
    }

    #[test]
    fn rejects_incomplete_camera_coordinates() {
        assert_eq!(
            CliCommand::parse(["--camera", "1,2"]).unwrap_err(),
            "--camera must use the format X,Y,Z"
        );
    }

    #[test]
    fn rejects_unknown_options() {
        assert_eq!(
            CliCommand::parse(["--quality", "high"]).unwrap_err(),
            "unknown option: --quality"
        );
    }
}

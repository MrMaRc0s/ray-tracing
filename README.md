# rt

A small, dependency-free ray tracer written in Rust. It renders spheres,
planes, cubes, and cylinders lit by a single point light with Lambert
diffuse shading, ambient light, and hard shadows, and writes the result as
a P3 (ASCII) PPM image on standard output.

## Building

```sh
cargo build --release
```

The binary is at `target/release/rt`. All examples below use
`cargo run --release --` to build and run in one step; drop `--release`
for faster compiles during development (renders are slower in debug mode).

## Usage

```sh
cargo run --release -- [OPTIONS] > output.ppm
```

| Option | Description | Default |
| --- | --- | --- |
| `--scene <NAME>` | Scene to render: `preview`, `sphere`, `dim`, `all` | `preview` |
| `--width <PIXELS>` | Image width | `800` |
| `--height <PIXELS>` | Image height | `600` |
| `--camera <X,Y,Z>` | Camera position | `0,0,2` |
| `--look-at <X,Y,Z>` | Point the camera looks at | `0,0,0` |
| `-h`, `--help` | Print usage and exit | |

The image is written to standard output as P3 PPM; redirect it to a file
(`> image.ppm`) or pipe it into an image viewer/converter.

## Audit checklist

One command per thing worth verifying, in the order you'd typically
check them:

| Check | Command |
| --- | --- |
| Build the project | `cargo build --release` |
| Run the full test suite | `cargo test` |
| Formatting is clean | `cargo fmt --check` |
| No lint warnings | `cargo clippy --all-targets -- -D warnings` |
| The 4 required renders exist | `ls -la renders/*.ppm` |
| View a render (macOS) | `sips -s format png renders/sphere.ppm --out /tmp/sphere.png && open /tmp/sphere.png` (or open the `.ppm` directly in Preview, which reads PPM natively) |
| Render a sphere-only scene | see [`sphere`](#sphere--a-single-sphere) below |
| Render a plane+cube scene at lower brightness | see [`dim`](#dim--a-plane-and-a-cube-at-lower-brightness) below |
| Render all four object types together | see [`all`](#all--sphere-cube-plane-and-cylinder-together) below |
| Render the same all-objects scene from a second camera | see [`all`](#all--sphere-cube-plane-and-cylinder-together) below, second command |
| Reduce the output resolution | append `--width 100 --height 75` (or any size) to any command above |
| Move the camera | change `--camera`/`--look-at` on any command above (see [Usage](#usage)) |
| Construct a new element / change brightness in code | see [Constructing a scene](#constructing-a-scene) below |

## Constructing a scene

Scenes are plain Rust, built with the same four constructors regardless
of which shape you're placing. Every shape takes a `Material` and
implements the shared `Hittable` trait, so `Scene::add` accepts any of
them interchangeably:

```rust
use rt::{Color, Cube, Cylinder, Material, Plane, Point3, PointLight, Scene, Sphere, Vec3};

let mut scene = Scene::new(Color::new(0.02, 0.02, 0.05)); // background color

// Sphere::new(center, radius, material)
scene.add(Sphere::new(
    Point3::new(0.0, 0.0, -1.0),
    1.0,
    Material::new(Color::new(0.9, 0.2, 0.2), /* ambient */ 0.1, /* diffuse */ 0.9),
));

// Plane::new(point_on_plane, normal, material) — infinite in both directions
scene.add(Plane::new(
    Point3::new(0.0, -1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Material::new(Color::new(0.7, 0.7, 0.7), 0.1, 0.7),
));

// Cube::new(center, half_size, material) — axis-aligned
scene.add(Cube::new(Point3::new(0.0, -0.5, -1.0), 0.5, Material::default()));

// Cylinder::new(base_center, radius, height, material) — vertical, base to base+height
scene.add(Cylinder::new(Point3::new(3.0, -1.0, -1.5), 0.6, 1.4, Material::default()));

// Light position, color, and brightness
let mut light = PointLight::new(Point3::new(5.0, 5.0, 5.0), Color::new(1.0, 1.0, 1.0), 1.0);
light.set_brightness(0.3); // change brightness at any time
```

Given a `scene`, a `Camera` (see [Usage](#usage) — `--camera`/`--look-at`
control its position and target), and a `light`, `rt::render(&scene,
&camera, &light)` returns the `Canvas` to write out as PPM.

The three named presets in the table above (`sphere`, `dim`, `all`) are
exactly this API used in [`src/scenes.rs`](src/scenes.rs) — open that
file to see the full working example, or add a new `match` arm there to
register a new named scene of your own; `cargo build` and `--scene
<your-name>` picks it up immediately.

## Scenes

### `preview`

A camera-ray direction preview (no lighting, no objects) proving that
resolution and camera options move the picture. This is Dev A's
foundation-verification scene.

```sh
cargo run --release -- --scene preview --width 400 --height 300 > preview.ppm
```

### `sphere` — a single sphere

A red sphere at the origin, demonstrating the quadratic sphere
intersection and diffuse/ambient shading.

```sh
cargo run --release -- --scene sphere --width 800 --height 600 \
  --camera 0,0,4 --look-at 0,0,-1 > renders/sphere.ppm
```

### `dim` — a plane and a cube at lower brightness

An infinite floor plane and a cube, lit at reduced light brightness
(`0.3` instead of the normal `1.0`) to demonstrate the point light's
brightness control and hard shadows on a flat surface.

```sh
cargo run --release -- --scene dim --width 800 --height 600 \
  --camera 0,2,5 --look-at 0,-0.7,-1 > renders/plane_cube_dim.ppm
```

### `all` — sphere, cube, plane, and cylinder together

All four object types on the floor plane at normal brightness, each
casting its own hard shadow toward the camera-facing side away from the
light.

```sh
cargo run --release -- --scene all --width 800 --height 600 \
  --camera 0,3,10 --look-at 0,-0.5,-1 > renders/all_objects.ppm
```

The same scene from a second camera position, demonstrating that
`--camera`/`--look-at` move the view without changing the scene itself:

```sh
cargo run --release -- --scene all --width 800 --height 600 \
  --camera 8,4,2 --look-at 0,-0.5,-1 > renders/all_objects_camera2.ppm
```

## Required renders

The four required 800x600 renders described above are committed under
[`renders/`](renders/):

- `renders/sphere.ppm` — sphere only
- `renders/plane_cube_dim.ppm` — plane and cube at lower light brightness
- `renders/all_objects.ppm` — sphere, cube, plane, and cylinder together
- `renders/all_objects_camera2.ppm` — the same all-objects scene from a
  second camera position

Regenerate them at any time with the four commands above.

## Fast auditor verification

The same scenes render in milliseconds at a small resolution, useful for
quickly checking that everything still works without waiting on an
800x600 render:

```sh
cargo run --release -- --scene sphere --width 100 --height 75 > /tmp/sphere.ppm
cargo run --release -- --scene dim --width 100 --height 75 > /tmp/dim.ppm
cargo run --release -- --scene all --width 100 --height 75 > /tmp/all.ppm
```

## Error handling

Invalid input is rejected with a message on standard error and a nonzero
exit code, for example:

```sh
cargo run --release -- --scene unknown
# error: scene 'unknown' is not available; use one of: preview, sphere, dim, all
# try 'rt --help' for usage

cargo run --release -- --camera 0,0,0 --look-at 0,0,0
# error: camera position and target must be different
```

## Tests and checks

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Unit tests live at the bottom of each source file, in a `#[cfg(test)]
mod tests` block — for example `src/shapes/sphere.rs`, `src/shapes/plane.rs`,
`src/shapes/cube.rs`, `src/shapes/cylinder.rs`, and `src/renderer.rs`.
They cover vector/ray math, camera rays, PPM structure and clamping,
closest-hit selection, CLI validation, and — per shape — misses, outside
hits, inside hits, nearest valid distances, normals, and translated
positions.

Integration tests live in [`tests/`](tests/):
[`tests/scene_presets.rs`](tests/scene_presets.rs) renders every named
scene at a small resolution, and [`tests/cli.rs`](tests/cli.rs)
exercises the compiled binary end to end (help output, PPM structure,
and error reporting).

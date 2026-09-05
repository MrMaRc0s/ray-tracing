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
| `--scene <NAME>` | Scene to render: `preview`, `sphere`, `dim`, `all`, `textures`, `glass`, `particles`, `fluid` | `preview` |
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
| Renders are valid, correctly-sized PPMs | `for f in renders/*.ppm; do echo "== $f =="; head -2 "$f"; done` (each should print `P3` then `800 600`) |
| Lower brightness is a measurable difference, not just a visual impression | `cargo test dim_scene_is_darker_than_the_all_objects_scene_at_the_same_pixel -- --nocapture` |
| CLI documents itself | `cargo run --release -- --help` |
| Malformed camera coordinates are rejected | `cargo run --release -- --camera 0,0` |
| A zero-sized image is rejected | `cargo run --release -- --width 0 --height 600` |
| **Bonus:** textures on a surface | see [`textures`](#textures--procedural-checker-pattern) below |
| **Bonus:** reflective and/or refractive objects | see [`glass`](#glass--mirror-and-refractive-spheres) below |
| **Bonus:** particles | see [`particles`](#particles--a-scattered-particle-cloud) below |
| **Bonus:** fluids | see [`fluid`](#fluid--a-rippled-reflectiverefractive-water-surface) below |

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

The seven named presets in the table above (`sphere`, `dim`, `all`,
`textures`, `glass`, `particles`, `fluid`) are exactly this API used in
[`src/scenes.rs`](src/scenes.rs) — open that file to see the full working
example, or add a new `match` arm there to register a new named scene of
your own; `cargo build` and `--scene <your-name>` picks it up immediately.

### Textures, reflection, and refraction

A `Material` defaults to a flat color, fully opaque and non-reflective.
Three builder methods opt into the bonus surface effects, and can be
combined:

```rust
use rt::{Color, Material, Texture};

// A procedural checker pattern, sampled from world-space coordinates —
// no image decoder needed, so the ray tracer stays dependency-free.
let checkered = Material::new(Color::new(0.9, 0.9, 0.9), 0.1, 0.9)
    .with_texture(Texture::checker(Color::new(0.9, 0.9, 0.9), Color::new(0.1, 0.1, 0.15), 1.0));

// A mirror: 90% of the shaded color comes from what the surface reflects.
let mirror = Material::new(Color::new(0.9, 0.9, 0.9), 0.05, 0.1).with_reflectivity(0.9);

// Glass: 95% transparent with a refractive index of 1.5 bends light passing through.
let glass = Material::new(Color::new(1.0, 1.0, 1.0), 0.02, 0.05).with_transparency(0.95, 1.5);
```

Reflective and refractive rays are traced recursively (up to 5 bounces,
see [`src/renderer.rs`](src/renderer.rs)), so mirrors can reflect other
mirrors and glass objects, and total internal reflection is handled when
a refracted ray's angle is too shallow to escape a denser medium.

### Particles

[`rt::scatter_particles`](src/particles.rs) adds many small spheres at
random (but seed-reproducible) positions within a box, using a tiny
built-in xorshift generator instead of an external `rand` crate:

```rust
use rt::{scatter_particles, Color, Point3, Scene};

let mut scene = Scene::new(Color::new(0.02, 0.02, 0.05));
scatter_particles(
    &mut scene,
    120,                                   // particle count
    Point3::new(-2.5, -0.9, -3.0),         // box min corner
    Point3::new(2.5, 2.0, 0.0),            // box max corner
    0.07,                                  // particle radius
    1234,                                  // seed — same seed, same scene
    |i| Color::new(0.9, 0.6, 0.3),         // color per particle index
);
```

### Fluids

[`WavyPlane`](src/shapes/wavy_plane.rs) approximates a liquid surface: a
perfectly flat intersection (so it's just as cheap to trace as `Plane`)
combined with a sine-rippled shading normal, so a reflective/refractive
material on it looks like rippling water rather than a static mirror.

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

## Bonus scenes

### `textures` — procedural checker pattern

A checkered floor (see [Textures, reflection, and refraction](#textures-reflection-and-refraction))
under a plain sphere, so the same light shows a textured and an untextured
surface side by side.

```sh
cargo run --release -- --scene textures --width 800 --height 600 \
  --camera 0,1,4 --look-at 0,-0.3,-1 > renders/bonus/textures.ppm
```

### `glass` — mirror and refractive spheres

A mirror-like sphere (90% reflective) and a glass sphere (95%
transparent, refractive index 1.5) over a checkered floor, each bouncing
rays recursively.

```sh
cargo run --release -- --scene glass --width 800 --height 600 \
  --camera 0,1,6 --look-at 0,-0.2,-1 > renders/bonus/glass.ppm
```

### `particles` — a scattered particle cloud

A floor plus 120 small spheres scattered at seed-reproducible random
positions (see [Particles](#particles)), each casting its own shadow.

```sh
cargo run --release -- --scene particles --width 800 --height 600 \
  --camera 0,1,6 --look-at 0,0.2,-1.5 > renders/bonus/particles.ppm
```

### `fluid` — a rippled, reflective/refractive water surface

A [`WavyPlane`](#fluids) water surface over a checkered sea floor, with a
sphere floating on top — the rippled shading normal distorts both the
mirror reflection and the refracted view of the floor below.

```sh
cargo run --release -- --scene fluid --width 800 --height 600 \
  --camera 0,0.8,4 --look-at 0,-0.6,-1.2 > renders/bonus/fluid.ppm
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

The four bonus renders are committed under [`renders/bonus/`](renders/bonus/)
— one per bonus feature (`textures.ppm`, `glass.ppm`, `particles.ppm`,
`fluid.ppm`) — and regenerate with the four commands in
[Bonus scenes](#bonus-scenes).

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
# error: scene 'unknown' is not available; use one of: preview, sphere, dim, all, textures, glass, particles, fluid
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
`src/shapes/cube.rs`, `src/shapes/cylinder.rs`, `src/shapes/wavy_plane.rs`,
`src/texture.rs`, `src/particles.rs`, and `src/renderer.rs`.
They cover vector/ray math (including `reflect`/`refract`), camera rays,
PPM structure and clamping, closest-hit selection, CLI validation, and —
per shape — misses, outside hits, inside hits, nearest valid distances,
normals, and translated positions, plus reflective/transparent rendering,
checker texture sampling, and seed-reproducible particle scattering.

Integration tests live in [`tests/`](tests/):
[`tests/scene_presets.rs`](tests/scene_presets.rs) renders every named
scene at a small resolution, and [`tests/cli.rs`](tests/cli.rs)
exercises the compiled binary end to end (help output, PPM structure,
and error reporting).

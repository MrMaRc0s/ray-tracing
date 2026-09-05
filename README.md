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

Unit tests cover vector/ray math, camera rays, PPM structure and
clamping, closest-hit selection, CLI validation, and — per shape —
misses, outside hits, inside hits, nearest valid distances, normals, and
translated positions. Integration tests in `tests/` render every named
scene at a small resolution and exercise the compiled binary end to end
(help output, PPM structure, and error reporting).

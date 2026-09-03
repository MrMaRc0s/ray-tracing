# Development plan

The goal is an audit-ready, dependency-free Rust ray tracer. The required work is
split by module ownership so both developers can work without editing the same
files. The only shared checkpoint is a small core-contract commit before parallel
development begins.

## Shared contract checkpoint

Dev A establishes these types before parallel work starts:

- `Vec3`, `Point3`, `Color`, and `Ray`
- `Material`
- `HitRecord` and the `Hittable` trait

Once this commit is available, the signatures stay stable until the two branches
are merged. Dev A can test the renderer with a tiny test-only `Hittable`; Dev B can
implement every object against the trait. Neither branch needs unfinished code
from the other to compile or run its unit tests.

## Dev A — core renderer (`dev-a/core-renderer`)

Owned files: shared contracts, camera, canvas/PPM, scene, lighting, renderer,
command-line entry point, and final `README.md`.

1. Establish the shared math, ray, material, and hit contracts.
2. Implement a movable look-at camera with configurable width and height.
3. Implement an in-memory canvas and ASCII P3 PPM output to standard output.
4. Implement a scene that finds the closest hit.
5. Add point-light brightness, Lambert diffuse lighting, ambient light, and hard
   shadows using secondary shadow rays.
6. Add a small command-line interface for selecting a scene and resolution.
7. Document object creation, brightness, camera position/angle, render commands,
   and the four required audit images.

Tests cover vector/ray math, camera rays, PPM structure and clamping, closest-hit
selection, brightness, and shadow visibility. Until Dev B's work is merged, scene
and renderer tests use a private fake object.

## Dev B — object geometry (`dev-b/geometry`)

Owned files: `src/shapes/` and geometry-focused tests.

1. Implement a translated sphere using a quadratic ray intersection.
2. Implement an infinite flat plane from a point and normal.
3. Implement an axis-aligned cube using the slab intersection method.
4. Implement a translated finite vertical cylinder, including its side and both
   end caps.
5. Test misses, outside hits, inside hits, nearest valid distances, normals, and
   translated positions for every object.

All constructors receive a `Material`, and all intersections return the shared
`HitRecord`. Dev B does not edit renderer or CLI files.

## Integration and audit pass

After both branches pass independently:

1. Merge geometry into the renderer branch and add named scene presets.
2. Render and commit four 800x600 P3 files:
   - sphere only;
   - plane and cube at lower light brightness;
   - sphere, cube, plane, and cylinder;
   - the same all-objects scene from a second camera position.
3. Test the same presets at a small resolution for fast auditor verification.
4. Run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test`.
5. Rehearse every functional audit question using only the README commands.

## Optional follow-up — not part of this implementation

Bonus work is deliberately deferred until every required audit item passes. A
future branch could add, in this order:

1. surface textures and UV coordinates;
2. recursive reflection/refraction with a maximum depth;
3. particle primitives;
4. a separate fluid simulation feeding renderable particles or a mesh.

Any expensive bonus would be disabled by default and enabled with explicit CLI
flags. No bonus code or placeholder flags are included in the required version.

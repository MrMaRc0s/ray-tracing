# Development plan

The goal is an audit-ready, dependency-free Rust ray tracer. The required work is
intentionally split about 45/55: Dev A builds the reusable foundation plus the
common CLI and a visual camera preview; Dev B owns the larger geometry, real
rendering, and audit-delivery part. Module ownership keeps both developers from
editing the same files.

## Shared contract checkpoint

Dev A establishes these types before parallel work starts:

- `Vec3`, `Point3`, `Color`, and `Ray`
- `Material`
- `HitRecord` and the `Hittable` trait

Once this commit is available, the signatures stay stable. Dev B can implement
every object against the trait while Dev A finishes the remaining foundation.
Neither branch needs unfinished object or renderer code to compile its unit
tests.

## Dev A — foundation, CLI, and preview, about 45% (`dev-a/core-renderer`)

Owned files: shared contracts, camera, canvas/PPM, scene, point-light shading,
and common CLI option parsing.

1. Establish the shared math, ray, material, and hit contracts.
2. Implement a movable look-at camera with configurable width and height.
3. Implement an in-memory canvas and ASCII P3 PPM output to standard output.
4. Implement a scene that finds the closest hit.
5. Add point-light brightness, Lambert diffuse lighting, and ambient light. The
   method accepts a shadow state, but Dev B owns detecting that state with a
   secondary ray.
6. Add dependency-free CLI options for scene name, width, height, camera position,
   and look-at target.
7. Replace the fixed color gradient with a visual preview colored from camera-ray
   directions. This proves that resolution and camera options affect an image,
   without implementing any of Dev B's objects or renderer.

Tests cover vector/ray math, camera rays, PPM structure and clamping, closest-hit
selection, brightness, shaded-light behavior, and CLI validation. Scene tests use
a private fake object, so Dev A never needs Dev B's unfinished shapes.

Dev A's `main.rs` writes the camera-ray preview. Dev B later replaces only the
preview wiring with the real renderer; the option parser stays unchanged.

## Dev B — geometry, renderer, and delivery, about 55% (`dev-b/ray-tracer`)

Owned files: `src/shapes/`, renderer, scene presets, final `main.rs` wiring,
integration tests, `README.md`, and required images.

1. Implement a translated sphere using a quadratic ray intersection.
2. Implement an infinite flat plane from a point and normal.
3. Implement an axis-aligned cube using the slab intersection method.
4. Implement a translated finite vertical cylinder, including its side and both
   end caps.
5. Test misses, outside hits, inside hits, nearest valid distances, normals, and
   translated positions for every object.
6. Implement the render loop that sends one camera ray per pixel and shades the
   closest hit.
7. Detect hard shadows with a ray from the hit point toward the light, limited to
   the light's distance.
8. Create the four required scene presets, including the lower-brightness scene
   and the second camera perspective.
9. Connect the final renderer to Dev A's CLI options, defaulting renders to
   800x600 while allowing smaller test images.
10. Write the public README with examples for every object, brightness, camera
    movement, and render commands.
11. Render, visually inspect, and commit the four required 800x600 PPM files.

All constructors receive a `Material`, and all intersections return the shared
`HitRecord`. Dev B can start with shapes immediately. CLI parsing can also be
used unchanged while Dev B builds and tests scene presets.

## Integration and audit pass

Dev B owns the final integration after both branches pass independently:

1. Merge Dev A's foundation once, then wire the renderer to the named presets.
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

use crate::{Color, Material, Point3, Scene, Sphere};

/// A tiny deterministic pseudo-random number generator (xorshift64), used
/// instead of pulling in an external `rand` crate so the ray tracer stays
/// dependency-free while particle scenes remain reproducible from a seed.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        // xorshift is undefined for a zero state, so nudge it away from zero.
        Self(seed | 1)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// A float uniformly distributed in `[0.0, 1.0)`.
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn range(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }
}

/// Generates `count` positions scattered uniformly at random inside the
/// axis-aligned box between `min_corner` and `max_corner`. The same `seed`
/// always produces the same positions, so a scene built from them renders
/// identically every time.
pub fn generate_positions(
    count: usize,
    min_corner: Point3,
    max_corner: Point3,
    seed: u64,
) -> Vec<Point3> {
    let mut rng = Rng::new(seed);
    (0..count)
        .map(|_| {
            Point3::new(
                rng.range(min_corner.x, max_corner.x),
                rng.range(min_corner.y, max_corner.y),
                rng.range(min_corner.z, max_corner.z),
            )
        })
        .collect()
}

/// Scatters `count` small spheres of `radius` at random positions inside the
/// box between `min_corner` and `max_corner`, coloring the `i`th particle
/// with `color_for(i)`, and adds them all to `scene` — a lightweight stand-in
/// for a full particle system (dust, sparks, rain) built entirely from the
/// existing `Sphere` shape and `Scene::add`.
pub fn scatter_particles(
    scene: &mut Scene,
    count: usize,
    min_corner: Point3,
    max_corner: Point3,
    radius: f64,
    seed: u64,
    color_for: impl Fn(usize) -> Color,
) {
    for (i, position) in generate_positions(count, min_corner, max_corner, seed)
        .into_iter()
        .enumerate()
    {
        let material = Material::new(color_for(i), 0.2, 0.8);
        scene.add(Sphere::new(position, radius, material));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_the_requested_number_of_positions() {
        let positions = generate_positions(25, Point3::zero(), Point3::new(1.0, 1.0, 1.0), 42);

        assert_eq!(positions.len(), 25);
    }

    #[test]
    fn every_position_stays_within_the_requested_bounds() {
        let min_corner = Point3::new(-2.0, 0.0, -3.0);
        let max_corner = Point3::new(2.0, 5.0, 3.0);

        for position in generate_positions(200, min_corner, max_corner, 7) {
            assert!((min_corner.x..=max_corner.x).contains(&position.x));
            assert!((min_corner.y..=max_corner.y).contains(&position.y));
            assert!((min_corner.z..=max_corner.z).contains(&position.z));
        }
    }

    #[test]
    fn the_same_seed_reproduces_the_same_positions() {
        let a = generate_positions(10, Point3::zero(), Point3::new(1.0, 1.0, 1.0), 99);
        let b = generate_positions(10, Point3::zero(), Point3::new(1.0, 1.0, 1.0), 99);

        assert_eq!(a, b);
    }

    #[test]
    fn different_seeds_produce_different_positions() {
        let a = generate_positions(10, Point3::zero(), Point3::new(1.0, 1.0, 1.0), 1);
        let b = generate_positions(10, Point3::zero(), Point3::new(1.0, 1.0, 1.0), 2);

        assert_ne!(a, b);
    }

    #[test]
    fn scattering_particles_adds_one_object_per_particle() {
        let mut scene = Scene::new(Color::zero());

        scatter_particles(
            &mut scene,
            30,
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(1.0, 1.0, 1.0),
            0.05,
            3,
            |_| Color::new(1.0, 1.0, 1.0),
        );

        assert_eq!(scene.len(), 30);
    }
}

use rand::Rng;

use crate::seeded_rng;

fn lattice_value(seed: u64, x: i32, y: i32) -> f32 {
    let cell_seed = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(x as u32 as u64)
        .wrapping_add((y as u32 as u64).wrapping_shl(32));
    seeded_rng(cell_seed).random::<f32>()
}

fn fade(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Simple 2D value noise: random values on a grid, smoothly interpolated.
pub fn value_noise_2d(x: f32, y: f32, seed: u64) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let tx = fade(x - x0 as f32);
    let ty = fade(y - y0 as f32);

    let v00 = lattice_value(seed, x0, y0);
    let v10 = lattice_value(seed, x0 + 1, y0);
    let v01 = lattice_value(seed, x0, y0 + 1);
    let v11 = lattice_value(seed, x0 + 1, y0 + 1);

    let vx0 = lerp(v00, v10, tx);
    let vx1 = lerp(v01, v11, tx);
    lerp(vx0, vx1, ty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_produces_identical_noise() {
        let a = value_noise_2d(1.25, 2.75, 42);
        let b = value_noise_2d(1.25, 2.75, 42);
        assert_eq!(a, b);
    }

    #[test]
    fn different_seeds_produce_different_noise() {
        let a = value_noise_2d(1.25, 2.75, 42);
        let b = value_noise_2d(1.25, 2.75, 43);
        assert_ne!(a, b);
    }

    #[test]
    fn noise_values_stay_in_unit_range() {
        for i in 0..20 {
            let x = i as f32 * 0.37;
            let y = i as f32 * 0.91;
            let value = value_noise_2d(x, y, 42);
            assert!((0.0..=1.0).contains(&value), "value was {value}");
        }
    }

    #[test]
    fn snapshot_seed_42_sample_points() {
        assert_eq!(value_noise_2d(0.5, 0.5, 42), 0.698_4364);
        assert_eq!(value_noise_2d(1.25, 2.75, 42), 0.419_47022);
        assert_eq!(value_noise_2d(10.0, 3.5, 42), 0.269_78937);
    }
}

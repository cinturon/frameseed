use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub fn seeded_rng(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn seeded_rng_produces_same_output() {
        let mut rng1 = seeded_rng(42);
        let mut rng2 = seeded_rng(42);

        let vals_a: Vec<u64> = (0..10).map(|_| rng1.random()).collect();
        let vals_b: Vec<u64> = (0..10).map(|_| rng2.random()).collect();

        assert_eq!(vals_a, vals_b);
    }

    #[test]
    fn seeded_rng_produces_different_output() {
        let mut rng1 = seeded_rng(42);
        let mut rng2 = seeded_rng(43);

        let vals_a: Vec<u64> = (0..10).map(|_| rng1.random()).collect();
        let vals_b: Vec<u64> = (0..10).map(|_| rng2.random()).collect();

        assert_ne!(vals_a, vals_b);
    }

    #[test]
    fn same_seed_produces_same_output() {
        let mut rng1 = seeded_rng(42);
        let mut rng2 = seeded_rng(42);

        let vals_a: Vec<u64> = (0..10).map(|_| rng1.random()).collect();
        let vals_b: Vec<u64> = (0..10).map(|_| rng2.random()).collect();

        assert_eq!(vals_a, vals_b);
    }
}

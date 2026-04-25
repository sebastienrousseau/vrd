// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! Integration tests for the `vrd` macros.

#[cfg(test)]
mod tests {

    use vrd::{Random, *};

    #[test]
    fn test_random_range_macro_within_bounds() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        for _ in 0..100 {
            let result = random_range!(rng, min, max);
            assert!(result >= min && result < max);
        }
    }

    #[test]
    fn test_rand_bool() {
        let mut rng = Random::new();
        let result = rand_bool!(rng, 0.5);
        let _ = result;
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_rand_bytes() {
        let mut rng = Random::new();
        let length = 8;
        let result = rand_bytes!(rng, length);
        assert_eq!(result.len(), length);
    }

    #[test]
    fn test_rand_char() {
        let mut rng = Random::new();
        let result = rand_char!(rng);
        assert!(result.is_ascii_lowercase());
    }

    #[test]
    fn test_rand_choose() {
        let mut rng = Random::new();
        let choices = [1, 2, 3, 4, 5];
        let result = rand_choose!(rng, &choices);
        assert!(choices.contains(result.unwrap()));
    }

    #[test]
    fn test_rand_float() {
        let mut rng = Random::new();
        let result = rand_float!(rng);
        assert!((0.0..1.0).contains(&result));
    }

    #[test]
    fn test_rand_int() {
        let mut rng = Random::new();
        let result = rand_int!(rng, 1, 10);
        assert!((1..=10).contains(&result));
    }

    #[test]
    fn test_rand_range() {
        let mut rng = Random::new();
        let result = rand_range!(rng, 1, 10);
        assert!((1..=10).contains(&result));
    }

    #[test]
    fn test_rand_seed() {
        let mut rng = Random::new();
        rand_seed!(rng, 42);
        let v1 = rng.rand();
        rand_seed!(rng, 42);
        let v2 = rng.rand();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_rand_twist() {
        let mut rng = Random::new_mersenne_twister();
        rand_twist!(rng);
        let _ = rng.rand();
    }

    #[test]
    fn test_rand_exponential() {
        let mut rng = Random::new();
        let result = rand_exponential!(rng, 1.0);
        assert!(result >= 0.0);
    }

    #[test]
    fn test_rand_normal() {
        let mut rng = Random::new();
        let result = rand_normal!(rng, 0.0, 1.0);
        assert!(result.is_finite());
    }

    #[test]
    fn test_rand_poisson() {
        let mut rng = Random::new();
        let result = rand_poisson!(rng, 3.0);
        let _ = result;
    }
}

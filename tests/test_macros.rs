// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {

    extern crate vrd;
    use vrd::random::Random;
    use vrd::*;


    #[test]
    fn test_random_range_macro_within_bounds() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = random_range!(rng, min, max);
        assert!(num >= min && num <= max, "Number should be within the given range.");
    }

    #[test]
    fn test_rand_bool_macro_always_true() {
        let mut rng = Random::new();
        let b = rand_bool!(rng, 1.0);
        assert!(b, "rand_bool should always return true with probability 1.0.");
    }

    #[test]
    fn test_rand_bool_macro_always_false() {
        let mut rng = Random::new();
        let b = rand_bool!(rng, 0.0);
        assert!(!b, "rand_bool should always return false with probability 0.0.");
    }

    #[test]
    fn test_rand_bytes_macro() {
        let mut rng = Random::new();
        let len = 10;
        let bytes = rand_bytes!(rng, len);
        assert_eq!(bytes.len(), len, "Length of bytes should be equal to the specified length.");
    }

    #[test]
    fn test_rand_char_macro_ascii_check() {
        let mut rng = Random::new();
        let c = rand_char!(rng);
        assert!(
            c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit(),
            "Generated character should be ASCII lowercase, uppercase, or digit."
        );
    }

    #[test]
    fn test_rand_choose_macro_value_in_slice() {
        let mut rng = Random::new();
        let values = vec![1, 2, 3, 4, 5];
        let chosen = rand_choose!(rng, &values).unwrap();
        assert!(values.contains(chosen), "Chosen value should be in the provided slice.");
    }

    #[test]
    fn test_rand_float_macro_within_bounds() {
        let mut rng = Random::new();
        let f = rand_float!(rng);
        assert!((0.0..1.0).contains(&f), "Generated float should be within 0.0 and 1.0.");
    }

    #[test]
    fn test_rand_int_macro_within_range() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = rand_int!(rng, min, max);
        assert!(num >= min && num <= max, "Generated integer should be within the specified range.");
    }

    #[test]
    fn test_rand_pseudo_macro_upper_bound() {
        let mut rng = Random::new();
        let p = rand_pseudo!(rng);
        assert!(p < 4294967295, "Generated pseudo random number should be less than 4294967295.");
    }

    #[test]
    fn test_rand_range_macro() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = rand_range!(rng, min, max);
        assert!(num >= min && num <= max);
    }

    #[test]
    fn test_rand_seed_macro() {
        let mut rng = Random::new();
        rand_seed!(rng, 42);
        let num = rng.rand();
        assert!(num < 4294967295);
    }

    #[test]
    fn test_rand_twist_macro() {
        let mut rng = Random::new();
        rand_twist!(rng);
        let num = rng.rand();
        assert!(num < 4294967295);
    }

    #[test]
    fn test_random_range_macro_valid_range() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = random_range!(rng, min, max);
        assert!(num >= min && num < max, "Number should be within the given range.");
    }

    #[test]
    fn test_rand_bool_macro_true() {
        let mut rng = Random::new();
        let p = 1.0; // Set p to 1.0 to always generate true
        let b = rand_bool!(rng, p);
        assert!(b, "The probability of 1.0 should always return true.");
    }

    #[test]
    fn test_rand_bool_macro_false() {
        let mut rng = Random::new();
        let p = 0.0; // Set p to 0.0 to always generate false
        let b = rand_bool!(rng, p);
        assert!(!b, "The probability of 0.0 should always return false.");
    }

    #[test]
    fn test_rand_int_macro_valid_range() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = rand_int!(rng, min, max);
        assert!(num >= min && num <= max, "Number should be within the given range.");
    }

}

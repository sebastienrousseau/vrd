// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! Integration tests for `vrd::random::Random`.

#[cfg(test)]
mod tests {
    use rand::rand_core::TryRng;
    use vrd::Random;

    // Initialization tests
    /// Tests the `new` method to ensure that the RNG is initialized correctly.
    #[test]
    fn test_new() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        assert_ne!(rng.rand(), rng.rand());
    }

    /// Tests the `seed` method to ensure that seeding produces consistent random numbers.
    #[test]
    fn test_seed() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        rng.seed(42);
        let val1 = rng.rand();
        rng.seed(42);
        let val2 = rng.rand();
        assert_eq!(val1, val2);
    }

    // Integer generation tests
    /// Tests the `int` method to ensure it generates integers within the specified range.
    #[test]
    fn test_int() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        rng.seed(20);
        for _ in 0..100 {
            let random_int = rng.int(1, 10);
            assert!((1..=10).contains(&random_int));
        }
    }

    /// Tests the `int` method to ensure it handles cases where min and max are equal.
    #[test]
    fn test_int_min_max_equal() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        assert_eq!(rng.int(5, 5), 5);
    }

    /// Tests the `int` method to ensure it panics when min is greater than max.
    #[test]
    #[should_panic(expected = "min must be <= max for int")]
    fn test_int_min_greater_than_max() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        rng.int(10, 5);
    }

    /// Tests the `uint` method to ensure it handles cases where min and max are equal.
    #[test]
    fn test_uint_min_max_equal() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        assert_eq!(rng.uint(5, 5), 5);
    }

    // Floating-point generation tests
    /// Tests the `float` method to ensure it generates floating-point numbers within the correct range.
    #[test]
    fn test_float() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        rng.seed(42);
        let result = rng.float();
        assert!((0.0..1.0).contains(&result));
    }

    /// Tests the `double` method to ensure it generates double-precision floating-point numbers within the correct range.
    #[test]
    fn test_double() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        rng.seed(42);
        let result = rng.double();
        assert!((0.0..1.0).contains(&result));
    }

    // Backend specific tests
    #[test]
    #[cfg(all(feature = "alloc", feature = "std"))]
    fn test_mersenne_twister_backend() {
        let mut rng = Random::new_mersenne_twister();
        rng.seed(12345);
        assert_eq!(rng.mti(), 624);
        let val = rng.rand();
        assert_ne!(val, 0);
    }

    #[test]
    fn test_xoshiro_backend() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        rng.seed(12345);
        assert_eq!(rng.mti(), 0); // Xoshiro should return 0 for mti
        let val = rng.rand();
        assert_ne!(val, 0);
    }

    #[test]
    fn test_from_seed_32() {
        let seed = [1u8; 32];
        let mut rng1 = Random::from_seed(seed);
        let mut rng2 = Random::from_seed(seed);
        assert_eq!(rng1.rand(), rng2.rand());
    }

    #[test]
    fn test_try_fill_bytes() {
        #[cfg(feature = "std")]
    let mut rng = Random::new();
    #[cfg(not(feature = "std"))]
    let mut rng = Random::from_u64_seed(0);
        let mut dest = [0u8; 32];
        rng.try_fill_bytes(&mut dest).unwrap();
        assert!(dest.iter().any(|&x| x != 0));
    }
}

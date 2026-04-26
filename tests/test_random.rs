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

    /// Display impl on the MT-backed `Random` includes the `mti` index;
    /// previous suite only exercised the Xoshiro Display branch.
    #[test]
    #[cfg(all(feature = "alloc", feature = "std"))]
    fn test_display_mersenne_backend() {
        let rng = Random::new_mersenne_twister_with_seed(42);
        let s = format!("{rng}");
        assert!(s.contains("MersenneTwister"), "got: {s}");
        assert!(s.contains("mti:"), "got: {s}");
    }

    /// `set_mti` and `twist` are no-ops on the Xoshiro backend; verify
    /// we don't silently corrupt subsequent draws.
    #[test]
    fn test_set_mti_and_twist_noop_on_xoshiro() {
        let mut a = Random::from_u64_seed(123);
        let baseline = a.rand();

        let mut b = Random::from_u64_seed(123);
        b.set_mti(999);
        b.twist();
        assert_eq!(b.rand(), baseline);
    }

    /// `set_mti` on MT actually moves the index.
    #[test]
    #[cfg(all(feature = "alloc", feature = "std"))]
    fn test_set_mti_on_mt() {
        let mut rng = Random::new_mersenne_twister_with_seed(0);
        rng.set_mti(0);
        assert_eq!(rng.mti(), 0);
    }

    /// Walks every public `Random` method on the Mersenne-Twister
    /// backend end-to-end. Pure smoke coverage — asserts only that
    /// each call produces a finite/in-range result, since the
    /// statistical guarantees are tested via the Xoshiro-default suite.
    #[test]
    #[cfg(all(feature = "alloc", feature = "std"))]
    fn test_full_api_on_mersenne_backend() {
        let mut rng = Random::new_mersenne_twister_with_seed(2024);

        // raw output
        let _ = rng.rand();
        let _ = rng.u64();
        let _ = rng.i64();
        let _ = rng.float();
        let _ = rng.double();
        let _ = rng.f64();

        // bounded
        assert!(rng.bounded(100) < 100);
        assert!((1..=10).contains(&rng.int(1, 10)));
        assert!((1..=10).contains(&rng.uint(1, 10)));
        assert!((1..=10).contains(&rng.range(1, 10)));
        assert!(rng.random_range(0, 100) < 100);

        // bools/chars/strings
        let _ = rng.bool(0.5);
        assert!(rng.char().is_ascii_lowercase());
        assert_eq!(rng.string(8).len(), 8);
        assert_eq!(rng.bytes(16).len(), 16);

        // slice ops
        let pool = [10, 20, 30, 40, 50];
        assert!(rng.choose(&pool).is_some());
        let mut shuf = [1, 2, 3, 4, 5];
        rng.shuffle(&mut shuf);
        assert_eq!(rng.sample(&pool, 3).len(), 3);
        assert_eq!(rng.sample_with_replacement(&pool, 5).len(), 5);
        assert!(rng.rand_slice(&pool, 2).is_ok());

        // distributions
        assert!(rng.normal(0.0, 1.0).is_finite());
        assert!(rng.exponential(1.0) >= 0.0);
        let _ = rng.poisson(3.0);

        // TryRng path
        let mut buf = [0u8; 24];
        rng.try_fill_bytes(&mut buf).unwrap();
        assert!(buf.iter().any(|&b| b != 0));

        // MT-specific helpers
        rng.seed(99);
        let pre_twist_mti = rng.mti();
        rng.twist();
        let post_twist_mti = rng.mti();
        assert!(post_twist_mti < pre_twist_mti);
        rng.set_mti(0);
        assert_eq!(rng.mti(), 0);

        // Display + backend introspection
        let s = format!("{rng}");
        assert!(s.contains("MersenneTwister"));
        let _ = rng.backend();
    }
}

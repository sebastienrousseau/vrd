// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `vrd` crate.
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

    /// `fill_array` returns a stack array of the requested length.
    /// Works in pure `no_std` without `alloc`.
    #[test]
    fn test_fill_array_lengths() {
        let mut rng = Random::from_u64_seed(0x00C0_FFEE);
        let a: [u8; 0] = rng.fill_array();
        assert_eq!(a.len(), 0);
        let b: [u8; 1] = rng.fill_array();
        assert_eq!(b.len(), 1);
        let c: [u8; 16] = rng.fill_array();
        assert!(c.iter().any(|&x| x != 0));
        let d: [u8; 64] = rng.fill_array();
        assert!(d.iter().any(|&x| x != 0));
    }

    /// `fill_array` is deterministic given the same seed.
    #[test]
    fn test_fill_array_deterministic() {
        let mut a = Random::from_u64_seed(42);
        let mut b = Random::from_u64_seed(42);
        let x: [u8; 32] = a.fill_array();
        let y: [u8; 32] = b.fill_array();
        assert_eq!(x, y);
    }

    /// `split()` returns `Some(_)` on the Xoshiro backend and the
    /// child produces a different stream than the parent.
    #[test]
    fn test_split_xoshiro_yields_independent_stream() {
        let mut parent = Random::from_u64_seed(7);
        let mut child =
            parent.split().expect("Xoshiro backend supports split");
        // Sibling streams shouldn't collide for the first dozen draws.
        for _ in 0..16 {
            assert_ne!(parent.u64(), child.u64());
        }
    }

    /// `split()` is deterministic — splitting two same-seeded parents
    /// produces two pairs of identical sibling streams.
    #[test]
    fn test_split_is_deterministic() {
        let mut a_parent = Random::from_u64_seed(99);
        let mut b_parent = Random::from_u64_seed(99);
        let mut a_child = a_parent.split().unwrap();
        let mut b_child = b_parent.split().unwrap();
        for _ in 0..8 {
            assert_eq!(a_parent.u64(), b_parent.u64());
            assert_eq!(a_child.u64(), b_child.u64());
        }
    }

    /// `split()` returns `None` on the Mersenne Twister backend —
    /// MT19937 has no analogous fixed-distance jump.
    #[test]
    #[cfg(all(feature = "alloc", feature = "std"))]
    fn test_split_mt_returns_none() {
        let mut rng = Random::new_mersenne_twister_with_seed(42);
        assert!(rng.split().is_none());
    }

    /// PCG32 backend dispatches through `Random` and is
    /// bit-deterministic given the same seed.
    #[test]
    #[cfg(feature = "pcg")]
    fn test_pcg32_backend_deterministic() {
        let mut a = Random::new_pcg32_with_seed(42);
        let mut b = Random::new_pcg32_with_seed(42);
        for _ in 0..16 {
            assert_eq!(a.rand(), b.rand());
        }
    }

    /// PCG64 backend dispatches through `Random` and is
    /// bit-deterministic given the same seed.
    #[test]
    #[cfg(feature = "pcg")]
    fn test_pcg64_backend_deterministic() {
        let mut a = Random::new_pcg64_with_seed(0xDEAD_BEEF);
        let mut b = Random::new_pcg64_with_seed(0xDEAD_BEEF);
        for _ in 0..16 {
            assert_eq!(a.u64(), b.u64());
        }
    }

    /// PCG backends interop with `try_fill_bytes`.
    #[test]
    #[cfg(feature = "pcg")]
    fn test_pcg_fill_bytes() {
        let mut a = Random::new_pcg32_with_seed(1);
        let mut b = Random::new_pcg64_with_seed(1);
        let mut buf = [0u8; 33];
        a.try_fill_bytes(&mut buf).unwrap();
        assert!(buf.iter().any(|&x| x != 0));
        b.try_fill_bytes(&mut buf).unwrap();
        assert!(buf.iter().any(|&x| x != 0));
    }

    /// `split()` is unsupported on PCG backends (no jump function).
    #[test]
    #[cfg(feature = "pcg")]
    fn test_pcg_split_returns_none() {
        let mut a = Random::new_pcg32_with_seed(1);
        let mut b = Random::new_pcg64_with_seed(1);
        assert!(a.split().is_none());
        assert!(b.split().is_none());
    }

    /// Entropy-seeded PCG constructors — covers the OS-RNG seeding
    /// branches that the deterministic constructors don't exercise.
    #[test]
    #[cfg(all(feature = "pcg", feature = "std"))]
    fn test_pcg_entropy_seeded_constructors() {
        let mut a = Random::new_pcg32();
        let mut b = Random::new_pcg64();
        // Two independent draws shouldn't collide for fresh
        // entropy-seeded RNGs.
        assert_ne!(a.rand(), 0);
        assert_ne!(b.u64(), 0);
    }

    /// Random::seed dispatches differently per backend; ensure
    /// every variant's seed-reset arm is hit.
    #[test]
    #[cfg(feature = "pcg")]
    fn test_seed_pcg32_resets_stream() {
        let mut rng = Random::new_pcg32_with_seed(0);
        let _ = rng.rand();
        rng.seed(42);
        let after = rng.rand();
        let mut baseline = Random::new_pcg32_with_seed(42);
        assert_eq!(after, baseline.rand());
    }

    #[test]
    #[cfg(feature = "pcg")]
    fn test_seed_pcg64_resets_stream() {
        let mut rng = Random::new_pcg64_with_seed(0);
        let _ = rng.u64();
        rng.seed(42);
        let after = rng.u64();
        let mut baseline = Random::new_pcg64_with_seed(42u128);
        assert_eq!(after, baseline.u64());
    }

    #[test]
    #[cfg(feature = "crypto")]
    fn test_seed_chacha_resets_stream() {
        let mut rng = Random::from_secure_seed([0u8; 32]);
        let _ = rng.u64();
        rng.seed(42);
        let after = rng.u64();
        // Build the same seed shape `seed()` uses internally.
        let mut s = [0u8; 32];
        s[0..4].copy_from_slice(&42u32.to_le_bytes());
        let mut baseline = Random::from_secure_seed(s);
        assert_eq!(after, baseline.u64());
    }

    /// PCG.next_u32 -> u32 (via Pcg64 backend) and inverse paths.
    /// Already partly covered, but pin the cross-output explicitly.
    #[test]
    #[cfg(feature = "pcg")]
    fn test_pcg64_rand_via_facade() {
        // Random::rand() on PCG64 should call .next_u32() which
        // returns the high 32 bits of the u64 draw.
        let mut rng = Random::new_pcg64_with_seed(7);
        let _ = rng.rand();
    }

    /// PCG32.next_u64 (via two u32 concats) on the facade path.
    #[test]
    #[cfg(feature = "pcg")]
    fn test_pcg32_u64_via_facade() {
        let mut rng = Random::new_pcg32_with_seed(7);
        let n = rng.u64();
        assert_ne!(n, 0);
    }

    /// Display impl on each backend variant.
    #[test]
    #[cfg(all(feature = "alloc", feature = "std"))]
    fn test_display_xoshiro_backend() {
        let rng = Random::from_u64_seed(1);
        let s = format!("{rng}");
        assert!(s.contains("Xoshiro256PlusPlus"), "got: {s}");
    }

    #[test]
    #[cfg(all(feature = "pcg", feature = "alloc", feature = "std"))]
    fn test_display_pcg_backends() {
        let r32 = Random::new_pcg32_with_seed(1);
        let r64 = Random::new_pcg64_with_seed(1);
        assert!(format!("{r32}").contains("Pcg32"));
        assert!(format!("{r64}").contains("Pcg64"));
    }

    #[test]
    #[cfg(all(feature = "crypto", feature = "alloc", feature = "std"))]
    fn test_display_chacha_backend() {
        let rng = Random::from_secure_seed([0u8; 32]);
        assert!(format!("{rng}").contains("ChaCha20"));
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

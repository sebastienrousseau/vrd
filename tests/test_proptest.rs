// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Property-based invariants for the public `Random` API.
//!
//! Each test runs proptest's default 256 iterations against random
//! seeds and (where applicable) random bounds. They check structural
//! properties that fixed-seed unit tests can miss.

use proptest::prelude::*;
use vrd::Random;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        ..ProptestConfig::default()
    })]

    /// `bounded(range)` always returns a value < `range`.
    #[test]
    fn bounded_always_below_range(seed: u64, range in 1u32..=u32::MAX / 2) {
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..32 {
            prop_assert!(rng.bounded(range) < range);
        }
    }

    /// `int(min, max)` always returns a value in `[min, max]`.
    #[test]
    fn int_within_inclusive_bounds(
        seed: u64,
        a in -10_000i32..=10_000,
        b in -10_000i32..=10_000,
    ) {
        let (min, max) = (a.min(b), a.max(b));
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..32 {
            let v = rng.int(min, max);
            prop_assert!(v >= min);
            prop_assert!(v <= max);
        }
    }

    /// `uint(min, max)` always returns a value in `[min, max]`.
    #[test]
    fn uint_within_inclusive_bounds(
        seed: u64,
        a in 0u32..=u32::MAX / 2,
        b in 0u32..=u32::MAX / 2,
    ) {
        let (min, max) = (a.min(b), a.max(b));
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..32 {
            let v = rng.uint(min, max);
            prop_assert!(v >= min);
            prop_assert!(v <= max);
        }
    }

    /// `random_range(min, max)` always returns a value in `[min, max)`.
    #[test]
    fn random_range_within_half_open_bounds(
        seed: u64,
        a in 0u32..=u32::MAX / 2,
        delta in 1u32..=10_000,
    ) {
        let min = a;
        let max = a.saturating_add(delta);
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..32 {
            let v = rng.random_range(min, max);
            prop_assert!(v >= min);
            prop_assert!(v < max);
        }
    }

    /// `float()` always lies in `[0.0, 1.0)`.
    #[test]
    fn float_in_zero_one(seed: u64) {
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..32 {
            let f = rng.float();
            prop_assert!(f.is_finite());
            prop_assert!((0.0..1.0).contains(&f));
        }
    }

    /// `double()` and `f64()` always lie in `[0.0, 1.0)`.
    #[test]
    fn double_in_zero_one(seed: u64) {
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..32 {
            let d = rng.double();
            let f = rng.f64();
            prop_assert!(d.is_finite() && f.is_finite());
            prop_assert!((0.0..1.0).contains(&d));
            prop_assert!((0.0..1.0).contains(&f));
        }
    }

    /// `uniform(low, high)` always lies in `[low, high)`.
    #[test]
    fn uniform_within_bounds(
        seed: u64,
        a in -1_000_000.0..1_000_000.0,
        delta in 1.0..1_000_000.0,
    ) {
        let low = a;
        let high = a + delta;
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..32 {
            let v = rng.uniform(low, high);
            prop_assert!(v >= low);
            prop_assert!(v < high);
        }
    }

    /// `bool(0.0)` always returns `false`; `bool(1.0)` always returns
    /// `true`.
    #[test]
    fn bool_edges_are_deterministic(seed: u64) {
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..16 {
            prop_assert!(!rng.bool(0.0));
            prop_assert!(rng.bool(1.0));
        }
    }

    /// `char()` always returns a lowercase ASCII letter.
    #[test]
    fn char_is_lowercase_ascii(seed: u64) {
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..32 {
            prop_assert!(rng.char().is_ascii_lowercase());
        }
    }

    /// `choose` returns `Some(&T)` whose value is in the slice.
    #[test]
    fn choose_returns_member(seed: u64, len in 1usize..=64) {
        let mut rng = Random::from_u64_seed(seed);
        let pool: Vec<u32> = (0..len as u32).collect();
        for _ in 0..32 {
            let pick = rng.choose(&pool).copied().expect("non-empty");
            prop_assert!(pool.contains(&pick));
        }
    }

    /// `shuffle` is a permutation: same multiset of elements after.
    #[test]
    fn shuffle_preserves_multiset(seed: u64, len in 0usize..=128) {
        let mut rng = Random::from_u64_seed(seed);
        let mut data: Vec<u32> = (0..len as u32).collect();
        let original = data.clone();
        rng.shuffle(&mut data);
        let mut sorted = data.clone();
        sorted.sort_unstable();
        prop_assert_eq!(sorted, original);
    }

    /// `sample(slice, k)` returns exactly `k` distinct indices.
    #[test]
    fn sample_yields_distinct_picks(
        seed: u64,
        len in 4usize..=64,
        k in 1usize..=4,
    ) {
        prop_assume!(k <= len);
        let mut rng = Random::from_u64_seed(seed);
        let pool: Vec<u32> = (0..len as u32).collect();
        let picks: Vec<u32> = rng
            .sample(&pool, k)
            .into_iter()
            .copied()
            .collect();
        prop_assert_eq!(picks.len(), k);
        let mut sorted = picks.clone();
        sorted.sort_unstable();
        sorted.dedup();
        prop_assert_eq!(sorted.len(), k);
    }

    /// `from_u64_seed` is deterministic: same seed -> same sequence.
    #[test]
    fn deterministic_under_same_seed(seed: u64) {
        let mut a = Random::from_u64_seed(seed);
        let mut b = Random::from_u64_seed(seed);
        for _ in 0..32 {
            prop_assert_eq!(a.rand(), b.rand());
        }
    }

    /// `iter_u32().take(n)` yields exactly `n` items.
    #[test]
    fn iter_u32_respects_take(seed: u64, n in 0usize..=128) {
        let mut rng = Random::from_u64_seed(seed);
        let xs: Vec<u32> = rng.iter_u32().take(n).collect();
        prop_assert_eq!(xs.len(), n);
    }

    /// `iter_bytes().take(n)` yields exactly `n` bytes.
    #[test]
    fn iter_bytes_respects_take(seed: u64, n in 0usize..=256) {
        let mut rng = Random::from_u64_seed(seed);
        let bytes: Vec<u8> = rng.iter_bytes().take(n).collect();
        prop_assert_eq!(bytes.len(), n);
    }

    /// `uuid_v4_bytes` always sets version=4 and variant=10x bits.
    #[test]
    fn uuid_v4_format_invariants(seed: u64) {
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..16 {
            let b = rng.uuid_v4_bytes();
            prop_assert_eq!(b[6] >> 4, 0x4);
            prop_assert_eq!(b[8] >> 6, 0b10);
        }
    }

    /// `uuid_v4` string is 36 chars and follows 8-4-4-4-12 hyphenation.
    #[test]
    fn uuid_v4_string_shape(seed: u64) {
        let mut rng = Random::from_u64_seed(seed);
        for _ in 0..16 {
            let s = rng.uuid_v4();
            prop_assert_eq!(s.len(), 36);
            prop_assert_eq!(s.as_bytes()[8],  b'-');
            prop_assert_eq!(s.as_bytes()[13], b'-');
            prop_assert_eq!(s.as_bytes()[18], b'-');
            prop_assert_eq!(s.as_bytes()[23], b'-');
        }
    }

    /// `hex_token(n)` is exactly 2*n lowercase hex chars.
    #[test]
    fn hex_token_shape(seed: u64, n in 0usize..=64) {
        let mut rng = Random::from_u64_seed(seed);
        let s = rng.hex_token(n);
        prop_assert_eq!(s.len(), n * 2);
        prop_assert!(s.chars().all(|c|
            c.is_ascii_digit() || ('a'..='f').contains(&c)
        ));
    }

    /// `base64_token(n)` length matches `((n + 2) / 3) * 4 - padding`
    /// and uses only the URL-safe alphabet.
    #[test]
    fn base64_token_shape(seed: u64, n in 0usize..=64) {
        let mut rng = Random::from_u64_seed(seed);
        let s = rng.base64_token(n);
        let expected = match n % 3 {
            0 => (n / 3) * 4,
            1 => (n / 3) * 4 + 2,
            _ => (n / 3) * 4 + 3,
        };
        prop_assert_eq!(s.len(), expected);
        prop_assert!(s.chars().all(|c| matches!(c,
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_'
        )));
    }
}

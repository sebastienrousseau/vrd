// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Convenience macros over the [`crate::Random`] facade.
//!
//! These wrap the inherent methods on `Random` so callers can write
//! short-form expressions without importing the type explicitly.

/// Generates an unbiased `u32` in the half-open range `[min, max)`.
#[macro_export]
macro_rules! random_range {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.random_range($min, $max)
    };
}

/// Generates a random `bool` whose probability of `true` is the second
/// argument. Panics if the probability is outside `[0.0, 1.0]`.
#[macro_export]
macro_rules! rand_bool {
    ($rng:expr, $probability:expr) => {{
        $rng.bool($probability)
    }};
}

/// Returns a `Vec<u8>` of `len` random bytes. Requires the `alloc`
/// feature.
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! rand_bytes {
    ($rng:expr, $len:expr) => {
        $rng.bytes($len)
    };
}

/// Returns a random lowercase ASCII char (`'a'..='z'`).
#[macro_export]
macro_rules! rand_char {
    ($rng:expr) => {
        $rng.char()
    };
}

/// Picks a random reference into the given slice (`Option<&T>`).
#[macro_export]
macro_rules! rand_choose {
    ($rng:expr, $values:expr) => {
        $rng.choose($values)
    };
}

/// Generates an `f32` in `[0.0, 1.0)`.
#[macro_export]
macro_rules! rand_float {
    ($rng:expr) => {
        $rng.float()
    };
}

/// Generates an unbiased `i32` in the inclusive range `[min, max]`.
#[macro_export]
macro_rules! rand_int {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.int($min, $max)
    };
}

/// Generates an unbiased `u32` in the inclusive range `[min, max]`.
#[macro_export]
macro_rules! rand_uint {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.uint($min, $max)
    };
}

/// Generates an `f64` in `[0.0, 1.0)`.
#[macro_export]
macro_rules! rand_double {
    ($rng:expr) => {
        $rng.double()
    };
}

/// Inclusive `[min, max]` range for `i32`.
#[macro_export]
macro_rules! rand_range {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.range($min, $max)
    };
}

/// Re-seeds the active backend.
#[macro_export]
macro_rules! rand_seed {
    ($rng:expr, $seed:expr) => {
        $rng.seed($seed)
    };
}

/// Forces a Mersenne-Twister twist; no-op on Xoshiro.
#[macro_export]
macro_rules! rand_twist {
    ($rng:expr) => {
        $rng.twist()
    };
}

/// Returns a random alphanumeric ASCII char.
#[macro_export]
macro_rules! rand_alphanumeric {
    ($rng:expr) => {{
        const CHARS: &[u8; 62] =
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let idx = $rng.bounded(CHARS.len() as u32) as usize;
        CHARS[idx] as char
    }};
}

/// Returns a fresh lowercase ASCII `String` of `length` chars. Requires
/// `alloc`.
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! rand_string {
    ($rng:expr, $length:expr) => {
        $rng.string($length)
    };
}

/// Fisher-Yates shuffle in place.
#[macro_export]
macro_rules! rand_shuffle {
    ($rng:expr, $slice:expr) => {{
        $rng.shuffle($slice)
    }};
}

/// Selects a reference into `$choices` weighted by `$weights`.
#[macro_export]
macro_rules! rand_weighted_choice {
    ($rng:expr, $choices:expr, $weights:expr) => {{
        assert_eq!(
            $choices.len(),
            $weights.len(),
            "choices and weights must have the same length"
        );
        let total: u32 = $weights.iter().sum();
        assert!(total > 0, "total weight must be positive");
        let mut rnd = $rng.bounded(total);
        let mut selected = None;
        for (i, &w) in $weights.iter().enumerate() {
            if rnd < w {
                selected = Some(&$choices[i]);
                break;
            }
            rnd -= w;
        }
        selected.expect("weighted choice failed to select")
    }};
}

/// Standard Box-Muller normal sample.
#[macro_export]
macro_rules! rand_normal {
    ($rng:expr, $mu:expr, $sigma:expr) => {{
        $rng.normal($mu, $sigma)
    }};
}

/// Exponential sample with the given rate.
#[macro_export]
macro_rules! rand_exponential {
    ($rng:expr, $rate:expr) => {{
        $rng.exponential($rate)
    }};
}

/// Poisson sample with the given mean.
#[macro_export]
macro_rules! rand_poisson {
    ($rng:expr, $mean:expr) => {{
        $rng.poisson($mean)
    }};
}

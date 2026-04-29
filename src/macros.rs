// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Convenience macros over the [`crate::Random`] facade.
//!
//! These wrap the inherent methods on `Random` so callers can write
//! short-form expressions without importing the type explicitly.

/// Generates an unbiased `u32` in the half-open range `[min, max)`.
///
/// # Examples
///
/// ```
/// use vrd::{Random, random_range};
///
/// let mut rng = Random::from_u64_seed(42);
/// let n = random_range!(rng, 10, 20);
/// assert!(n >= 10 && n < 20);
/// ```
#[macro_export]
macro_rules! random_range {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.random_range($min, $max)
    };
}

/// Generates a random `bool` whose probability of `true` is the second
/// argument. Panics if the probability is outside `[0.0, 1.0]`.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_bool};
///
/// let mut rng = Random::from_u64_seed(42);
/// let b = rand_bool!(rng, 0.5);
/// ```
#[macro_export]
macro_rules! rand_bool {
    ($rng:expr, $probability:expr) => {{
        $rng.bool($probability)
    }};
}

/// Returns a `Vec<u8>` of `len` random bytes. Requires the `alloc`
/// feature.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_bytes};
///
/// # #[cfg(feature = "alloc")]
/// # {
/// let mut rng = Random::from_u64_seed(42);
/// let bytes = rand_bytes!(rng, 16);
/// assert_eq!(bytes.len(), 16);
/// # }
/// ```
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! rand_bytes {
    ($rng:expr, $len:expr) => {
        $rng.bytes($len)
    };
}

/// Returns a random lowercase ASCII char (`'a'..='z'`).
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_char};
///
/// let mut rng = Random::from_u64_seed(42);
/// let c = rand_char!(rng);
/// assert!(c.is_ascii_lowercase());
/// ```
#[macro_export]
macro_rules! rand_char {
    ($rng:expr) => {
        $rng.char()
    };
}

/// Picks a random reference into the given slice (`Option<&T>`).
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_choose};
///
/// let mut rng = Random::from_u64_seed(42);
/// let choices = [1, 2, 3];
/// let pick = rand_choose!(rng, &choices);
/// ```
#[macro_export]
macro_rules! rand_choose {
    ($rng:expr, $values:expr) => {
        $rng.choose($values)
    };
}

/// Generates an `f32` in `[0.0, 1.0)`.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_float};
///
/// let mut rng = Random::from_u64_seed(42);
/// let f = rand_float!(rng);
/// ```
#[macro_export]
macro_rules! rand_float {
    ($rng:expr) => {
        $rng.float()
    };
}

/// Generates an unbiased `i32` in the inclusive range `[min, max]`.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_int};
///
/// let mut rng = Random::from_u64_seed(42);
/// let n = rand_int!(rng, -10, 10);
/// ```
#[macro_export]
macro_rules! rand_int {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.int($min, $max)
    };
}

/// Generates an unbiased `u32` in the inclusive range `[min, max]`.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_uint};
///
/// let mut rng = Random::from_u64_seed(42);
/// let n = rand_uint!(rng, 0, 100);
/// ```
#[macro_export]
macro_rules! rand_uint {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.uint($min, $max)
    };
}

/// Generates an `f64` in `[0.0, 1.0)`.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_double};
///
/// let mut rng = Random::from_u64_seed(42);
/// let d = rand_double!(rng);
/// ```
#[macro_export]
macro_rules! rand_double {
    ($rng:expr) => {
        $rng.double()
    };
}

/// Inclusive `[min, max]` range for `i32`.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_range};
///
/// let mut rng = Random::from_u64_seed(42);
/// let n = rand_range!(rng, 1, 10);
/// ```
#[macro_export]
macro_rules! rand_range {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.range($min, $max)
    };
}

/// Re-seeds the active backend.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_seed};
///
/// let mut rng = Random::from_u64_seed(42);
/// rand_seed!(rng, 123);
/// ```
#[macro_export]
macro_rules! rand_seed {
    ($rng:expr, $seed:expr) => {
        $rng.seed($seed)
    };
}

/// Forces a Mersenne-Twister twist; no-op on Xoshiro.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_twist};
///
/// # #[cfg(all(feature = "alloc", feature = "std"))]
/// # {
/// let mut rng = Random::new_mersenne_twister();
/// rand_twist!(rng);
/// # }
/// ```
#[macro_export]
macro_rules! rand_twist {
    ($rng:expr) => {
        $rng.twist()
    };
}

/// Returns a random alphanumeric ASCII char.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_alphanumeric};
///
/// let mut rng = Random::from_u64_seed(42);
/// let c = rand_alphanumeric!(rng);
/// ```
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
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_string};
///
/// # #[cfg(feature = "alloc")]
/// # {
/// let mut rng = Random::from_u64_seed(42);
/// let s = rand_string!(rng, 10);
/// # }
/// ```
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! rand_string {
    ($rng:expr, $length:expr) => {
        $rng.string($length)
    };
}

/// Fisher-Yates shuffle in place.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_shuffle};
///
/// let mut rng = Random::from_u64_seed(42);
/// let mut nums = [1, 2, 3];
/// rand_shuffle!(rng, &mut nums);
/// ```
#[macro_export]
macro_rules! rand_shuffle {
    ($rng:expr, $slice:expr) => {{
        $rng.shuffle($slice)
    }};
}

/// Selects a reference into `$choices` weighted by `$weights`.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_weighted_choice};
///
/// let mut rng = Random::from_u64_seed(42);
/// let choices = ["a", "b"];
/// let weights = [10, 90];
/// let pick = rand_weighted_choice!(rng, &choices, &weights);
/// ```
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
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_normal};
///
/// let mut rng = Random::from_u64_seed(42);
/// let n = rand_normal!(rng, 0.0, 1.0);
/// ```
#[macro_export]
macro_rules! rand_normal {
    ($rng:expr, $mu:expr, $sigma:expr) => {{
        $rng.normal($mu, $sigma)
    }};
}

/// Exponential sample with the given rate.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_exponential};
///
/// let mut rng = Random::from_u64_seed(42);
/// let e = rand_exponential!(rng, 1.5);
/// ```
#[macro_export]
macro_rules! rand_exponential {
    ($rng:expr, $rate:expr) => {{
        $rng.exponential($rate)
    }};
}

/// Poisson sample with the given mean.
///
/// # Examples
///
/// ```
/// use vrd::{Random, rand_poisson};
///
/// let mut rng = Random::from_u64_seed(42);
/// let p = rand_poisson!(rng, 3.0);
/// ```
#[macro_export]
macro_rules! rand_poisson {
    ($rng:expr, $mean:expr) => {{
        $rng.poisson($mean)
    }};
}

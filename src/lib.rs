// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full
// license information.

#![no_std]
#![doc(
    html_favicon_url = "https://cloudcdn.pro/vrd/v1/favicon.ico",
    html_logo_url = "https://cloudcdn.pro/vrd/v1/logos/vrd.svg",
    html_root_url = "https://docs.rs/vrd"
)]
#![crate_name = "vrd"]
#![crate_type = "lib"]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]
#![doc = "Minimum supported Rust version: 1.70.0"]

//! # Versatile Random Distributions (VRD)
//!
//! [![Crates.io](https://img.shields.io/crates/v/vrd.svg)](https://crates.io/crates/vrd)
//! [![Docs.rs](https://img.shields.io/docsrs/vrd.svg)](https://docs.rs/vrd)
//! [![License](https://img.shields.io/badge/license-MIT_or_Apache--2.0-blue.svg)](https://github.com/sebastienrousseau/vrd#license)
//!
//! A lightweight, `no_std`-friendly random number generator backed by
//! **Xoshiro256++**, with optional **Mersenne Twister (MT19937)** support.
//!
//! ## Features
//! - **High performance:** Xoshiro256++ default — 32-byte state, period
//!   2^256 - 1, SplitMix64 seed whitening.
//! - **Legacy reproducibility:** opt-in MT19937 backend.
//!   `Random::new_mersenne_twister()` requires `alloc + std`;
//!   `Random::new_mersenne_twister_with_seed(u32)` is `alloc`-only.
//! - **`no_std` ready:** pure-core build with `default-features = false`,
//!   validated on `thumbv7em-none-eabihf` (Cortex-M) and
//!   `wasm32-unknown-unknown` (WebAssembly) in CI.
//! - **Unbiased sampling:** `int`, `uint`, `random_range`, and the public
//!   `bounded` use Lemire's nearly-divisionless method.
//! - **Bit-precise floats:** `float()` carries 24 mantissa bits, `double()`
//!   carries 53. Always `[0.0, 1.0)`.
//! - **Distributions:** `uniform(low, high)`, `normal`, `exponential`,
//!   `poisson` — `std`-free via `libm`.
//! - **Convenience helpers:** `iter_u32` / `iter_u64` / `iter_bytes`
//!   iterator adapters; `uuid_v4_bytes` (`no_std`) and `uuid_v4`
//!   (`alloc`); `hex_token` and `base64_token` for URL-safe random
//!   tokens.
//! - **`rand 0.10` traits:** `TryRng`, the blanket-implemented `Rng`, and
//!   `SeedableRng`.
//!
//! ## Quickstart
//!
//! ```
//! use vrd::Random;
//!
//! let mut rng = Random::from_u64_seed(42);   // deterministic, allocation-free
//!
//! let n: u32 = rng.rand();                    // any u32
//! let _      = rng.u64();                     // any u64
//! let _      = rng.int(1, 100);               // i32 in [1, 100], uniform
//! let _      = rng.double();                  // f64 in [0.0, 1.0)
//! let _      = rng.bool(0.5);                 // 50/50 coin
//! assert!(n > 0 || n == 0);
//! ```
//!
//! Use [`Random::new()`] for entropy-seeded randomness on `std` targets,
//! [`Random::from_seed()`] / [`Random::from_u64_seed()`] for deterministic
//! / `no_std` use, and [`Random::new_mersenne_twister()`] / [`Random::new_mersenne_twister_with_seed()`]
//! when you need bit-for-bit MT19937 reproducibility against existing
//! test vectors.
//!
//! ## Not a CSPRNG
//!
//! `Random` is **not** cryptographically secure. For credentials,
//! session IDs, or anything an attacker would benefit from predicting,
//! use `rand::rngs::OsRng` or `getrandom`. A built-in ChaCha20-based
//! CSPRNG backend is tracked in
//! [issue #90](https://github.com/sebastienrousseau/vrd/issues/90).

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::fmt;

/// Crate-level error type for the `vrd` library.
///
/// This error type is used to represent general failures within the library.
/// It is kept allocation-free by using static error messages, ensuring it
/// works correctly in pure `no_std` environments without requiring an
/// allocator.
///
/// # Examples
///
/// ```
/// use vrd::VrdError;
///
/// let err = VrdError::GeneralError("something went wrong");
/// println!("{}", err);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum VrdError {
    /// A general error with a static message.
    ///
    /// This variant is used for unexpected conditions where a more specific
    /// error type isn't applicable.
    GeneralError(&'static str),
}

impl fmt::Display for VrdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VrdError::GeneralError(msg) => {
                write!(f, "General error: {}", msg)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VrdError {}

/// Convenience macros.
pub mod macros;
/// Mersenne Twister configuration and constants.
pub mod mersenne_twister;
/// The core `Random` facade.
pub mod random;
/// Xoshiro256++ implementation.
pub mod xoshiro;

pub use mersenne_twister::{
    MersenneTwisterConfig, MersenneTwisterError, MersenneTwisterParams,
};
pub use random::{FloatExt, Random, RngBackend};

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::format;
    #[cfg(all(not(feature = "alloc"), feature = "std"))]
    use std::format;

    #[test]
    #[cfg(any(feature = "alloc", feature = "std"))]
    fn test_vrd_error_display() {
        let err = VrdError::GeneralError("test error");
        let s = format!("{}", err);
        assert_eq!(s, "General error: test error");
    }

    #[test]
    #[cfg(any(feature = "alloc", feature = "std"))]
    fn test_vrd_error_debug() {
        let err = VrdError::GeneralError("test error");
        let s = format!("{:?}", err);
        assert!(s.contains("GeneralError"));
        assert!(s.contains("test error"));
    }
}

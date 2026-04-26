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
//! - **Legacy reproducibility:** opt-in MT19937 backend behind the `alloc`
//!   feature.
//! - **`no_std` ready:** pure-core build with `default-features = false`,
//!   validated on `thumbv7em-none-eabihf` (Cortex-M) in CI.
//! - **Unbiased sampling:** `int`, `uint`, `random_range`, and the public
//!   `bounded` use Lemire's nearly-divisionless method.
//! - **Bit-precise floats:** `float()` carries 24 mantissa bits, `double()`
//!   carries 53. Always `[0.0, 1.0)`.
//! - **Distributions:** uniform, normal, exponential, Poisson — `std`-free
//!   via `libm`.
//! - **`rand 0.10` traits:** `TryRng`, the blanket-implemented `Rng`, and
//!   `SeedableRng`.
//!
//! ## Not a CSPRNG
//!
//! `Random` is **not** cryptographically secure. For credentials or
//! security-sensitive randomness, use `rand::rngs::OsRng` or `getrandom`.

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

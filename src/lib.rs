// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full
// license information.

//! # Random (VRD)
//!
//! A lightweight, `no_std`-friendly random number generator built around
//! **Xoshiro256++** as the default backend, with optional **Mersenne
//! Twister (MT19937)** support for callers who need MT-specific
//! reproducibility.
//!
//! ## Features
//!
//! - `default = ["std"]` — `std`-backed entropy seeding via `rand::rng()`.
//! - `alloc` — enables [`Random::bytes`], [`Random::string`],
//!   [`Random::sample`], and the heap-stored Mersenne Twister backend.
//! - `serde` — derives `Serialize`/`Deserialize` for the public types.
//!
//! On a pure `no_std` target without `alloc`, [`Random::from_seed`] gives
//! you a fully working Xoshiro256++ generator with no allocations.
//!
//! ## Quickstart
//!
//! ```
//! use vrd::Random;
//! # #[cfg(feature = "std")]
//! # {
//! let mut rng = Random::new();
//! let n = rng.rand();          // u32
//! let f: f64 = rng.f64();      // [0.0, 1.0)
//! # }
//! ```
//!
//! ## `no_std` quickstart
//!
//! ```
//! use vrd::Random;
//! let mut rng = Random::from_seed([0x42; 32]);
//! let n = rng.rand();
//! ```

#![doc(
    html_favicon_url = "https://kura.pro/vrd/images/favicon.ico",
    html_logo_url = "https://kura.pro/vrd/images/logos/vrd.svg",
    html_root_url = "https://docs.rs/vrd"
)]
#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::fmt;

/// Mersenne Twister algorithm types and configuration.
pub mod mersenne_twister;

/// Xoshiro256++ algorithm.
pub mod xoshiro;

/// Convenience macros that wrap [`Random`] methods.
pub mod macros;

/// The `Random` facade and its enum-dispatched backends.
pub mod random;

pub use mersenne_twister::{
    MersenneTwisterConfig, MersenneTwisterError, MersenneTwisterParams,
};
pub use random::{FloatExt, Random, RngBackend};
pub use xoshiro::Xoshiro256PlusPlus;

/// Crate-level error type.
///
/// Kept allocation-free so it works under pure `no_std`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum VrdError {
    /// A general error with a static message.
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

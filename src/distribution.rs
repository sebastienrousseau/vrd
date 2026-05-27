// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Trait-based distributions and pluggable user-defined samplers.
//!
//! [`Distribution<T>`] is the universal handshake between a sampler
//! and an RNG: anything that can map a [`Random`] into a `T` is a
//! distribution. The built-in samplers (`Normal`, `Exponential`,
//! `Uniform`, `Poisson`) live here as concrete impls and forward to
//! the optimised methods on [`Random`]. Users add their own
//! distributions by implementing the trait.
//!
//! # Examples
//!
//! ```
//! use vrd::{Random, Distribution};
//! use vrd::distribution::{Normal, Exponential};
//!
//! let mut rng = Random::from_u64_seed(1);
//! let z = Normal { mu: 0.0, sigma: 1.0 }.sample(&mut rng);
//! let x = Exponential { rate: 1.5 }.sample(&mut rng);
//! # let _ = (z, x);
//! ```
//!
//! ```
//! use vrd::{Random, Distribution};
//!
//! // User-defined distribution: Bernoulli(p).
//! struct Bernoulli { p: f64 }
//!
//! impl Distribution<bool> for Bernoulli {
//!     fn sample(&self, rng: &mut Random) -> bool {
//!         rng.double() < self.p
//!     }
//! }
//!
//! let mut rng = Random::from_u64_seed(1);
//! let coin = Bernoulli { p: 0.5 }.sample(&mut rng);
//! # let _ = coin;
//! ```

use crate::Random;

/// A distribution that can be sampled with a mutable [`Random`].
///
/// `T` is the sample type — usually `f64` for continuous
/// distributions, `u64` for integer ones, but any `T` is allowed.
pub trait Distribution<T> {
    /// Draws one sample from `self` using `rng`.
    fn sample(&self, rng: &mut Random) -> T;

    /// Returns an iterator that calls `sample` repeatedly. Useful
    /// for `take(n).collect()`-style consumption.
    fn samples<'a>(
        &'a self,
        rng: &'a mut Random,
    ) -> Iter<'a, Self, T>
    where
        Self: Sized,
    {
        Iter {
            dist: self,
            rng,
            _t: core::marker::PhantomData,
        }
    }
}

/// Iterator returned by [`Distribution::samples`].
#[derive(Debug)]
pub struct Iter<'a, D: Distribution<T> + ?Sized, T> {
    dist: &'a D,
    rng: &'a mut Random,
    _t: core::marker::PhantomData<T>,
}

impl<D: Distribution<T>, T> Iterator for Iter<'_, D, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        Some(self.dist.sample(self.rng))
    }
}

// ---------------- Built-in continuous distributions ----------------------

/// Standard normal `N(mu, sigma^2)` — Ziggurat sampler, see
/// [`Random::normal`].
#[derive(Clone, Copy, Debug)]
pub struct Normal {
    /// Mean.
    pub mu: f64,
    /// Standard deviation. Must be ≥ 0.
    pub sigma: f64,
}

impl Distribution<f64> for Normal {
    fn sample(&self, rng: &mut Random) -> f64 {
        rng.normal(self.mu, self.sigma)
    }
}

/// Exponential with rate `lambda`. Mean is `1/lambda`.
#[derive(Clone, Copy, Debug)]
pub struct Exponential {
    /// Rate parameter `λ`. Must be > 0.
    pub rate: f64,
}

impl Distribution<f64> for Exponential {
    fn sample(&self, rng: &mut Random) -> f64 {
        rng.exponential(self.rate)
    }
}

/// Continuous uniform on `[low, high)`.
#[derive(Clone, Copy, Debug)]
pub struct Uniform {
    /// Inclusive lower bound.
    pub low: f64,
    /// Exclusive upper bound. Must be > `low`.
    pub high: f64,
}

impl Distribution<f64> for Uniform {
    fn sample(&self, rng: &mut Random) -> f64 {
        rng.uniform(self.low, self.high)
    }
}

/// Poisson with mean `lambda`.
#[derive(Clone, Copy, Debug)]
pub struct Poisson {
    /// Mean parameter `λ`. Must be > 0.
    pub mean: f64,
}

impl Distribution<u64> for Poisson {
    fn sample(&self, rng: &mut Random) -> u64 {
        rng.poisson(self.mean)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;
    #[cfg(all(not(feature = "alloc"), feature = "std"))]
    use std::vec::Vec;

    #[test]
    fn normal_distribution_samples() {
        let mut rng = Random::from_u64_seed(1);
        let n = Normal { mu: 0.0, sigma: 1.0 };
        for _ in 0..256 {
            assert!(n.sample(&mut rng).is_finite());
        }
    }

    #[test]
    fn exponential_distribution_samples() {
        let mut rng = Random::from_u64_seed(1);
        let e = Exponential { rate: 2.0 };
        for _ in 0..256 {
            assert!(e.sample(&mut rng) >= 0.0);
        }
    }

    #[test]
    fn uniform_distribution_in_range() {
        let mut rng = Random::from_u64_seed(1);
        let u = Uniform { low: -5.0, high: 5.0 };
        for _ in 0..256 {
            let s = u.sample(&mut rng);
            assert!((-5.0..5.0).contains(&s));
        }
    }

    #[test]
    fn poisson_distribution_samples() {
        let mut rng = Random::from_u64_seed(1);
        let p = Poisson { mean: 3.0 };
        for _ in 0..256 {
            // u64 is always >= 0; just confirm it doesn't panic /
            // exhaust the iterator.
            let _ = p.sample(&mut rng);
        }
    }

    /// User-defined distribution interop check.
    #[test]
    fn custom_distribution_compiles_and_runs() {
        struct Coin {
            bias: f64,
        }
        impl Distribution<bool> for Coin {
            fn sample(&self, rng: &mut Random) -> bool {
                rng.double() < self.bias
            }
        }
        let mut rng = Random::from_u64_seed(7);
        let fair: Vec<bool> =
            Coin { bias: 0.5 }.samples(&mut rng).take(64).collect();
        assert_eq!(fair.len(), 64);
    }
}

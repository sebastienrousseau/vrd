// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The [`Random`] facade and the enum-dispatched [`RngBackend`] backends.

use crate::mersenne_twister::MersenneTwisterConfig;
use crate::xoshiro::Xoshiro256PlusPlus;
use core::convert::Infallible;
use rand::rand_core::{SeedableRng, TryRng};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_big_array::BigArray;

// ---------------------------------------------------------------------------
// FloatExt — abstraction over std vs libm for no_std math.
// ---------------------------------------------------------------------------

/// Floating-point math operations bridged across `std` / `libm`.
pub trait FloatExt {
    /// Natural logarithm.
    fn ln(self) -> Self;
    /// Square root.
    fn sqrt(self) -> Self;
    /// Cosine.
    fn cos(self) -> Self;
    /// Exponential.
    fn exp(self) -> Self;
}

impl FloatExt for f64 {
    #[inline]
    fn ln(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::ln(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::log(self)
        }
    }
    #[inline]
    fn sqrt(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::sqrt(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::sqrt(self)
        }
    }
    #[inline]
    fn cos(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::cos(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::cos(self)
        }
    }
    #[inline]
    fn exp(self) -> Self {
        #[cfg(feature = "std")]
        {
            f64::exp(self)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::exp(self)
        }
    }
}

// ---------------------------------------------------------------------------
// MersenneTwister generator — relegated, available only with `alloc`.
// ---------------------------------------------------------------------------

/// Canonical MT19937 generator (`N = 624`, `M = 397`).
///
/// 2496-byte state — kept behind `alloc` because [`RngBackend`] always
/// boxes it.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MersenneTwister {
    /// Internal state vector.
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub mt: [u32; 624],
    /// Current index into `mt`.
    pub mti: usize,
}

impl Default for MersenneTwister {
    fn default() -> Self {
        Self::new()
    }
}

impl MersenneTwister {
    /// Creates a new generator with an empty state. Call [`Self::seed`]
    /// before use.
    pub fn new() -> Self {
        Self {
            mt: [0; 624],
            mti: 625,
        }
    }

    /// Seeds the generator from a 32-byte buffer; only the low 4 bytes
    /// are needed for MT19937 itself, but accepting 32 bytes keeps the
    /// API consistent with [`Xoshiro256PlusPlus::from_seed`]. The full
    /// 32 bytes are mixed via XOR-fold so callers don't silently lose
    /// entropy.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let mut s = 0u32;
        for chunk in seed.chunks_exact(4) {
            s ^= u32::from_le_bytes([
                chunk[0], chunk[1], chunk[2], chunk[3],
            ]);
        }
        let mut mt = Self::new();
        mt.seed(s);
        mt
    }

    /// Seeds the MT state from a 32-bit value using the canonical
    /// Knuth multiplier constant.
    pub fn seed(&mut self, seed: u32) {
        const N: usize = 624;
        self.mt[0] = seed;
        for i in 1..N {
            self.mt[i] = 1_812_433_253u32
                .wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        self.mti = N;
    }

    /// Performs the MT twist on the state vector.
    pub fn twist(&mut self) {
        const N: usize = 624;
        const M: usize = 397;
        let config = MersenneTwisterConfig::<N, M>::default();
        for i in 0..N {
            let x = (self.mt[i] & config.params.upper_mask)
                + (self.mt[(i + 1) % N] & config.params.lower_mask);
            let x_a = x >> 1;
            self.mt[i] = if x % 2 != 0 {
                self.mt[(i + M) % N]
                    ^ x_a
                    ^ config.params.matrix_a
            } else {
                self.mt[(i + M) % N] ^ x_a
            };
        }
        self.mti = 0;
    }

    /// Generates the next `u32`.
    #[inline]
    pub fn rand(&mut self) -> u32 {
        const N: usize = 624;
        if self.mti >= N {
            self.twist();
        }
        let mut y = self.mt[self.mti];
        self.mti += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^ (y >> 18)
    }

    /// Generates the next `u64` by combining two `u32` outputs.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let hi = self.rand() as u64;
        let lo = self.rand() as u64;
        (hi << 32) | lo
    }

    /// Returns the current index into the state vector.
    pub fn mti(&self) -> usize {
        self.mti
    }

    /// Forces the index to `value`; mostly useful for round-trip tests.
    pub fn set_mti(&mut self, value: usize) {
        self.mti = value;
    }
}

impl TryRng for MersenneTwister {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.rand())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.next_u64())
    }

    fn try_fill_bytes(
        &mut self,
        dest: &mut [u8],
    ) -> Result<(), Self::Error> {
        let mut i = 0;
        while i + 4 <= dest.len() {
            let bytes = self.rand().to_le_bytes();
            dest[i..i + 4].copy_from_slice(&bytes);
            i += 4;
        }
        if i < dest.len() {
            let bytes = self.rand().to_le_bytes();
            let remaining = dest.len() - i;
            dest[i..].copy_from_slice(&bytes[..remaining]);
        }
        Ok(())
    }
}

impl SeedableRng for MersenneTwister {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        MersenneTwister::from_seed(seed)
    }
}

// ---------------------------------------------------------------------------
// RngBackend — Xoshiro inline (always), MT boxed (alloc-gated).
// ---------------------------------------------------------------------------

/// Available backends for [`Random`].
///
/// Xoshiro is inline (no allocation) so it works on pure `no_std`. MT is
/// only available with the `alloc` feature because its 2496-byte state is
/// stored on the heap to keep the enum size small.
// Xoshiro inline is 32 bytes; MT-via-Box is 8 bytes. Boxing Xoshiro to
// equalize sizes would force a heap allocation per `Random` instance and
// defeat the purpose of inlining a small-state generator. Accept the
// imbalance.
#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum RngBackend {
    /// Xoshiro256++ — fast, small-state, statistically strong.
    Xoshiro256PlusPlus(Xoshiro256PlusPlus),
    /// Mersenne Twister (MT19937) — for callers needing legacy
    /// reproducibility. Requires the `alloc` feature.
    #[cfg(feature = "alloc")]
    MersenneTwister(Box<MersenneTwister>),
}

// ---------------------------------------------------------------------------
// Random — the user-facing facade.
// ---------------------------------------------------------------------------

/// Random number generator dispatched over [`RngBackend`].
///
/// The default backend is Xoshiro256++. Construct with [`Random::new`] for
/// entropy-seeded operation under `std`, or [`Random::from_seed`] for a
/// deterministic, allocation-free generator on any target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Random {
    backend: RngBackend,
}

impl Random {
    // ----------------------------- constructors -----------------------------

    /// Creates a new entropy-seeded Xoshiro256++ generator. Requires
    /// `std` for the OS entropy source.
    #[cfg(feature = "std")]
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        let mut sm: u64 = rand::random();
        for chunk in seed.chunks_exact_mut(8) {
            sm = sm.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            let v: u64 = rand::random::<u64>() ^ sm;
            chunk.copy_from_slice(&v.to_le_bytes());
        }
        Self::from_seed(seed)
    }

    /// Creates a Xoshiro256++-backed [`Random`] from a 32-byte seed.
    /// Allocation-free; available on any target.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            backend: RngBackend::Xoshiro256PlusPlus(
                Xoshiro256PlusPlus::from_seed(seed),
            ),
        }
    }

    /// Convenience constructor for a Xoshiro256++-backed instance from a
    /// `u64` seed. Allocation-free.
    pub fn from_u64_seed(seed: u64) -> Self {
        Self {
            backend: RngBackend::Xoshiro256PlusPlus(
                Xoshiro256PlusPlus::from_u64_seed(seed),
            ),
        }
    }

    /// Creates a Mersenne-Twister-backed [`Random`] seeded with a `u32`.
    /// Requires `alloc`.
    #[cfg(feature = "alloc")]
    pub fn new_mersenne_twister_with_seed(seed: u32) -> Self {
        let mut mt = MersenneTwister::new();
        mt.seed(seed);
        Self {
            backend: RngBackend::MersenneTwister(Box::new(mt)),
        }
    }

    /// Creates an entropy-seeded Mersenne-Twister-backed [`Random`].
    /// Requires `alloc` + `std`.
    #[cfg(all(feature = "alloc", feature = "std"))]
    pub fn new_mersenne_twister() -> Self {
        Self::new_mersenne_twister_with_seed(rand::random())
    }

    // ------------------------------ raw output ------------------------------

    /// Generates the next `u32`.
    #[inline]
    pub fn rand(&mut self) -> u32 {
        match &mut self.backend {
            RngBackend::Xoshiro256PlusPlus(xs) => xs.next_u32(),
            #[cfg(feature = "alloc")]
            RngBackend::MersenneTwister(mt) => mt.rand(),
        }
    }

    /// Generates the next `u64` natively when on Xoshiro; otherwise
    /// concatenates two 32-bit MT outputs.
    #[inline]
    pub fn u64(&mut self) -> u64 {
        match &mut self.backend {
            RngBackend::Xoshiro256PlusPlus(xs) => xs.next_u64(),
            #[cfg(feature = "alloc")]
            RngBackend::MersenneTwister(mt) => mt.next_u64(),
        }
    }

    /// Generates the next `i64`.
    #[inline]
    pub fn i64(&mut self) -> i64 {
        self.u64() as i64
    }

    /// Re-seeds the active backend from a `u32`. The seed is expanded
    /// via SplitMix64 before priming the Xoshiro state to avoid
    /// low-entropy starting points.
    pub fn seed(&mut self, seed: u32) {
        match &mut self.backend {
            RngBackend::Xoshiro256PlusPlus(xs) => {
                *xs = Xoshiro256PlusPlus::from_u64_seed(seed as u64);
            }
            #[cfg(feature = "alloc")]
            RngBackend::MersenneTwister(mt) => mt.seed(seed),
        }
    }

    /// Returns a reference to the active backend.
    pub fn backend(&self) -> &RngBackend {
        &self.backend
    }

    // -------------------------- bounded sampling ---------------------------

    /// Generates an unbiased `u32` in `[0, range)` using Lemire's
    /// nearly-divisionless method.
    ///
    /// Panics on `range == 0`.
    #[inline]
    pub fn bounded(&mut self, range: u32) -> u32 {
        assert!(range > 0, "range must be greater than zero");
        let mut x = u64::from(self.rand()).wrapping_mul(u64::from(range));
        let mut l = x as u32;
        if l < range {
            let t = range.wrapping_neg() % range;
            while l < t {
                x = u64::from(self.rand())
                    .wrapping_mul(u64::from(range));
                l = x as u32;
            }
        }
        (x >> 32) as u32
    }

    /// Generates an unbiased `u32` in `[min, max)`. Panics if `max <= min`.
    #[inline]
    pub fn random_range(&mut self, min: u32, max: u32) -> u32 {
        assert!(max > min, "max must be greater than min");
        min + self.bounded(max - min)
    }

    /// Generates an unbiased `i32` in `[min, max]` (inclusive). Panics if
    /// `min > max`.
    pub fn int(&mut self, min: i32, max: i32) -> i32 {
        assert!(min <= max, "min must be <= max for int");
        if min == max {
            return min;
        }
        // Width fits in u32 because max - min <= 2 * i32::MAX which fits.
        let range = (max as i64 - min as i64 + 1) as u32;
        min.wrapping_add(self.bounded(range) as i32)
    }

    /// Inclusive alias for [`Self::int`].
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.int(min, max)
    }

    /// Generates an unbiased `u32` in `[min, max]` (inclusive). Panics if
    /// `min > max`.
    pub fn uint(&mut self, min: u32, max: u32) -> u32 {
        assert!(min <= max, "min must be <= max for uint");
        if min == max {
            return min;
        }
        let range = max - min + 1;
        min + self.bounded(range)
    }

    // --------------------------- bools, chars ------------------------------

    /// Generates a random `bool` whose probability of being `true` is
    /// `probability`. Panics if `probability` is outside `[0.0, 1.0]`.
    pub fn bool(&mut self, probability: f64) -> bool {
        assert!(
            (0.0..=1.0).contains(&probability),
            "probability must be in [0.0, 1.0]"
        );
        self.f64() < probability
    }

    /// Generates a lowercase ASCII character in `'a'..='z'`.
    pub fn char(&mut self) -> char {
        let v = self.bounded(26) as u8;
        (b'a' + v) as char
    }

    /// Picks a random reference into `values`, or `None` if the slice is
    /// empty.
    pub fn choose<'a, T>(
        &mut self,
        values: &'a [T],
    ) -> Option<&'a T> {
        if values.is_empty() {
            return None;
        }
        let idx = self.bounded(values.len() as u32) as usize;
        Some(&values[idx])
    }

    // ------------------------------ floats ---------------------------------

    /// Generates an `f32` in `[0.0, 1.0)` with 24 bits of mantissa
    /// precision (the maximum representable in `f32`).
    #[inline]
    pub fn float(&mut self) -> f32 {
        // Top 24 bits of a uniform u32, scaled by 2^-24.
        const SCALE: f32 = 1.0 / ((1u32 << 24) as f32);
        ((self.rand() >> 8) as f32) * SCALE
    }

    /// Generates an `f64` in `[0.0, 1.0)` with 53 bits of mantissa
    /// precision (the maximum representable in `f64`).
    #[inline]
    pub fn double(&mut self) -> f64 {
        // Top 53 bits of a uniform u64, scaled by 2^-53.
        const SCALE: f64 = 1.0 / ((1u64 << 53) as f64);
        ((self.u64() >> 11) as f64) * SCALE
    }

    /// Alias for [`Self::double`].
    #[inline]
    pub fn f64(&mut self) -> f64 {
        self.double()
    }

    // -------------------------- byte / Vec output --------------------------

    /// Returns a fresh `Vec<u8>` of `len` random bytes.
    #[cfg(feature = "alloc")]
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut buf = alloc::vec![0u8; len];
        let _ = self.try_fill_bytes(&mut buf);
        buf
    }

    /// Returns a fresh `String` of `length` lowercase ASCII characters.
    #[cfg(feature = "alloc")]
    pub fn string(&mut self, length: usize) -> String {
        (0..length).map(|_| self.char()).collect()
    }

    // ------------------------------ shuffling -------------------------------

    /// Fisher-Yates shuffle in place.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        if slice.len() < 2 {
            return;
        }
        for i in (1..slice.len()).rev() {
            let j = self.bounded((i + 1) as u32) as usize;
            slice.swap(i, j);
        }
    }

    /// Sample `amount` references without replacement via partial
    /// Fisher-Yates with `swap_remove` — O(amount) draws, each O(1).
    #[cfg(feature = "alloc")]
    pub fn sample<'a, T>(
        &mut self,
        slice: &'a [T],
        amount: usize,
    ) -> Vec<&'a T> {
        let mut result = Vec::with_capacity(amount);
        let mut indices: Vec<usize> = (0..slice.len()).collect();
        for _ in 0..amount {
            let pick = self.bounded(indices.len() as u32) as usize;
            let chosen = indices.swap_remove(pick);
            result.push(&slice[chosen]);
        }
        result
    }

    /// Sample `amount` references with replacement.
    #[cfg(feature = "alloc")]
    pub fn sample_with_replacement<'a, T>(
        &mut self,
        slice: &'a [T],
        amount: usize,
    ) -> Vec<&'a T> {
        let mut result = Vec::with_capacity(amount);
        for _ in 0..amount {
            let idx = self.bounded(slice.len() as u32) as usize;
            result.push(&slice[idx]);
        }
        result
    }

    /// Returns a contiguous random subslice of `length`. Panics on
    /// degenerate arguments (empty input, zero length, or `length >
    /// slice.len()`).
    pub fn rand_slice<'a, T>(
        &mut self,
        slice: &'a [T],
        length: usize,
    ) -> Result<&'a [T], &'static str> {
        if slice.is_empty() {
            return Err("input slice is empty");
        }
        if length == 0 {
            return Err("requested length must be greater than zero");
        }
        if length > slice.len() {
            return Err("requested length exceeds slice length");
        }
        let start =
            self.bounded((slice.len() - length + 1) as u32) as usize;
        Ok(&slice[start..start + length])
    }

    // ---------------------- statistical distributions -----------------------

    /// Standard Box-Muller normal sample with parameters `(mu, sigma)`.
    pub fn normal(&mut self, mu: f64, sigma: f64) -> f64 {
        let u1 = self.f64();
        let u2 = self.f64();
        // Avoid ln(0) on the off-chance u1 == 0.
        let u1 = if u1 == 0.0 { f64::MIN_POSITIVE } else { u1 };
        let z0 = FloatExt::sqrt(-2.0 * FloatExt::ln(u1))
            * FloatExt::cos(2.0 * core::f64::consts::PI * u2);
        mu + sigma * z0
    }

    /// Exponential sample with rate `lambda`. Panics if `lambda <= 0.0`.
    pub fn exponential(&mut self, rate: f64) -> f64 {
        assert!(rate > 0.0, "rate must be positive");
        let u = 1.0 - self.f64();
        -FloatExt::ln(u) / rate
    }

    /// Poisson sample with mean `lambda`. Knuth's multiplicative
    /// algorithm — accurate for small `lambda`, slow for large.
    pub fn poisson(&mut self, mean: f64) -> u64 {
        let l = FloatExt::exp(-mean);
        let mut k = 0u64;
        let mut p = 1.0;
        loop {
            k += 1;
            p *= self.f64();
            if p < l {
                break;
            }
        }
        k - 1
    }

    // ------------------- MT-specific helpers (no-op on Xoshiro) ------------

    /// Returns the current MT index, or 0 when running on Xoshiro.
    pub fn mti(&self) -> usize {
        match &self.backend {
            #[cfg(feature = "alloc")]
            RngBackend::MersenneTwister(mt) => mt.mti(),
            _ => 0,
        }
    }

    /// Sets the MT index. No-op on Xoshiro.
    pub fn set_mti(&mut self, value: usize) {
        match &mut self.backend {
            #[cfg(feature = "alloc")]
            RngBackend::MersenneTwister(mt) => mt.set_mti(value),
            _ => {
                let _ = value;
            }
        }
    }

    /// Forces an MT twist. No-op on Xoshiro.
    pub fn twist(&mut self) {
        match &mut self.backend {
            #[cfg(feature = "alloc")]
            RngBackend::MersenneTwister(mt) => mt.twist(),
            _ => {}
        }
    }
}

#[cfg(feature = "std")]
impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for Random {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.backend {
            RngBackend::Xoshiro256PlusPlus(_) => {
                write!(f, "Random {{ backend: Xoshiro256PlusPlus }}")
            }
            #[cfg(feature = "alloc")]
            RngBackend::MersenneTwister(mt) => write!(
                f,
                "Random {{ backend: MersenneTwister, mti: {} }}",
                mt.mti
            ),
        }
    }
}

impl TryRng for Random {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.rand())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.u64())
    }

    fn try_fill_bytes(
        &mut self,
        dest: &mut [u8],
    ) -> Result<(), Self::Error> {
        match &mut self.backend {
            RngBackend::Xoshiro256PlusPlus(xs) => {
                xs.try_fill_bytes(dest)
            }
            #[cfg(feature = "alloc")]
            RngBackend::MersenneTwister(mt) => mt.try_fill_bytes(dest),
        }
    }
}

impl SeedableRng for Random {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        Random::from_seed(seed)
    }
}

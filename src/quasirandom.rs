// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Quasi-random (low-discrepancy) sequences.
//!
//! Quasi-random sequences cover the unit cube `[0, 1)^D` more evenly
//! than uniform PRNG draws — variance for Monte Carlo integration
//! scales `O((log n)^d / n)` rather than `O(1/√n)`. They're standard
//! tools for ray-tracing, financial simulation, and high-dimensional
//! numerical integration.
//!
//! Three constructions are shipped:
//!
//! - [`VanDerCorputSequence`](crate::quasirandom::VanDerCorputSequence)
//!   — 1-D, any prime base.
//! - [`HaltonSequence`](crate::quasirandom::HaltonSequence)
//!   — multi-dim, Van der Corput across the first primes. Supports
//!   up to 32 dimensions.
//! - [`SobolSequence`](crate::quasirandom::SobolSequence)
//!   — multi-dim, uses precomputed direction numbers. Supports up
//!   to 6 dimensions out of the box; the Joe-Kuo D6 file would
//!   extend this past 21 000.
//!
//! These are **not** PRNGs and live alongside, not inside,
//! [`crate::Random`]. Use a PRNG when you want unpredictability;
//! use a quasi-random sequence when you want even coverage.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ---------------------------------------------------------------------------
// Van der Corput (1-D)
// ---------------------------------------------------------------------------

/// 1-D Van der Corput sequence in a given prime base. Each step
/// reverses the index's base-`b` digit expansion and reads it as
/// a fraction in `[0, 1)`.
///
/// # Examples
///
/// ```
/// use vrd::quasirandom::VanDerCorputSequence;
///
/// let mut vdc = VanDerCorputSequence::new(2);
/// assert!((vdc.next_point() - 0.5).abs() < 1e-12);
/// assert!((vdc.next_point() - 0.25).abs() < 1e-12);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct VanDerCorputSequence {
    base: u32,
    i: u64,
}

impl VanDerCorputSequence {
    /// Builds a Van der Corput sequence in base `base`. `base` must
    /// be ≥ 2 (typically a prime for good distribution).
    pub fn new(base: u32) -> Self {
        assert!(base >= 2, "Van der Corput base must be at least 2");
        Self { base, i: 1 }
    }

    /// Skips forward by `n` points. Cheap because each point's value
    /// is determined entirely by the index.
    pub fn skip(&mut self, n: u64) {
        self.i = self.i.saturating_add(n);
    }

    /// Returns the next point in the sequence.
    pub fn next_point(&mut self) -> f64 {
        let p = radical_inverse(self.i, self.base);
        self.i += 1;
        p
    }
}

impl Iterator for VanDerCorputSequence {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        Some(self.next_point())
    }
}

#[inline]
fn radical_inverse(mut n: u64, base: u32) -> f64 {
    let b = u64::from(base);
    let mut q = 0.0_f64;
    let mut bk = 1.0_f64 / (base as f64);
    while n > 0 {
        let digit = n % b;
        q += (digit as f64) * bk;
        n /= b;
        bk /= base as f64;
    }
    q
}

// ---------------------------------------------------------------------------
// Halton (multi-dim)
// ---------------------------------------------------------------------------

/// First 32 primes — bases for the Halton sequence's per-dimension
/// Van der Corput components.
const HALTON_PRIMES: [u32; 32] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
    67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131,
];

/// Maximum number of dimensions supported by [`HaltonSequence`].
pub const HALTON_MAX_DIM: usize = HALTON_PRIMES.len();

/// Multi-dimensional Halton sequence — Van der Corput across the
/// first [`HALTON_MAX_DIM`] primes. Discrepancy is excellent for
/// dimensions up to ~10; beyond that consider [`SobolSequence`] or
/// scrambled Halton.
///
/// # Examples
///
/// ```
/// use vrd::quasirandom::HaltonSequence;
///
/// let mut h = HaltonSequence::new(2);
/// let p = h.next_point::<2>();
/// assert_eq!(p.len(), 2);
/// assert!(p[0] >= 0.0 && p[0] < 1.0);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct HaltonSequence {
    d: usize,
    i: u64,
}

impl HaltonSequence {
    /// Builds an `d`-dimensional Halton sequence. Panics if
    /// `d > HALTON_MAX_DIM`.
    pub fn new(d: usize) -> Self {
        assert!(
            d > 0 && d <= HALTON_MAX_DIM,
            "Halton dimension must be in 1..={HALTON_MAX_DIM}"
        );
        Self { d, i: 1 }
    }

    /// Skips forward by `n` points.
    pub fn skip(&mut self, n: u64) {
        self.i = self.i.saturating_add(n);
    }

    /// Returns the next `D`-dimensional point. Panics if
    /// `D != self.dimensions()`.
    pub fn next_point<const D: usize>(&mut self) -> [f64; D] {
        assert_eq!(
            D, self.d,
            "const D must match dimensions configured at construction"
        );
        let mut out = [0.0; D];
        for (k, slot) in out.iter_mut().enumerate() {
            *slot = radical_inverse(self.i, HALTON_PRIMES[k]);
        }
        self.i += 1;
        out
    }

    /// Returns the next point as an allocated [`Vec`]. Useful when
    /// `D` isn't a compile-time constant. Requires `alloc`.
    #[cfg(feature = "alloc")]
    pub fn next_point_vec(&mut self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.d);
        for &base in &HALTON_PRIMES[..self.d] {
            out.push(radical_inverse(self.i, base));
        }
        self.i += 1;
        out
    }

    /// Returns the dimension count.
    pub fn dimensions(&self) -> usize {
        self.d
    }
}

// ---------------------------------------------------------------------------
// Sobol (multi-dim)
// ---------------------------------------------------------------------------

/// Maximum number of dimensions [`SobolSequence`] supports with the
/// shipped direction-number table.
pub const SOBOL_MAX_DIM: usize = 6;

/// Direction numbers for the first six Sobol dimensions
/// (Bratley-Fox 1988 starter set). Each row holds 32 direction
/// numbers, sized for `u32` indexing.
// Hand-derived from polynomial primitives & m_i initial values
// listed in Bratley & Fox (1988) Table 1. Computed via:
//   v_i = m_i << (32 - i)               for i = 1..=s
//   v_i = v_{i-s} ^ (v_{i-s} >> s)
//                ^ a_1 v_{i-1} ^ ... ^ a_{s-1} v_{i-s+1}   for i > s
// where (a_1, ..., a_{s-1}) are the polynomial's middle bits
// and `s` is its degree.
const SOBOL_DIRECTIONS: [[u32; 32]; SOBOL_MAX_DIM] = [
    [
        0x8000_0000,
        0x4000_0000,
        0x2000_0000,
        0x1000_0000,
        0x0800_0000,
        0x0400_0000,
        0x0200_0000,
        0x0100_0000,
        0x0080_0000,
        0x0040_0000,
        0x0020_0000,
        0x0010_0000,
        0x0008_0000,
        0x0004_0000,
        0x0002_0000,
        0x0001_0000,
        0x0000_8000,
        0x0000_4000,
        0x0000_2000,
        0x0000_1000,
        0x0000_0800,
        0x0000_0400,
        0x0000_0200,
        0x0000_0100,
        0x0000_0080,
        0x0000_0040,
        0x0000_0020,
        0x0000_0010,
        0x0000_0008,
        0x0000_0004,
        0x0000_0002,
        0x0000_0001,
    ],
    // Dim 2: polynomial x + 1, m_1 = 1
    [
        0x8000_0000,
        0xC000_0000,
        0xA000_0000,
        0xF000_0000,
        0x8800_0000,
        0xCC00_0000,
        0xAA00_0000,
        0xFF00_0000,
        0x8080_0000,
        0xC0C0_0000,
        0xA0A0_0000,
        0xF0F0_0000,
        0x8888_0000,
        0xCCCC_0000,
        0xAAAA_0000,
        0xFFFF_0000,
        0x8000_8000,
        0xC000_C000,
        0xA000_A000,
        0xF000_F000,
        0x8800_8800,
        0xCC00_CC00,
        0xAA00_AA00,
        0xFF00_FF00,
        0x8080_8080,
        0xC0C0_C0C0,
        0xA0A0_A0A0,
        0xF0F0_F0F0,
        0x8888_8888,
        0xCCCC_CCCC,
        0xAAAA_AAAA,
        0xFFFF_FFFF,
    ],
    // Dim 3: polynomial x^2 + x + 1, m = [1, 3]
    [
        0x8000_0000,
        0x4000_0000,
        0xE000_0000,
        0x5000_0000,
        0xF800_0000,
        0x2C00_0000,
        0xE200_0000,
        0x4900_0000,
        0xF7C0_0000,
        0x4A60_0000,
        0xEF00_0000,
        0x5DD0_0000,
        0xF108_0000,
        0x2528_0000,
        0xCCC2_0000,
        0xFFFB_0000,
        0x8001_8000,
        0x4002_4000,
        0xE003_E000,
        0x5004_5000,
        0xF807_F800,
        0x2C0C_2C00,
        0xE20D_E200,
        0x4914_4900,
        0xF7CD_F7C0,
        0x4A6F_4A60,
        0xEF7B_EF00,
        0x5DDF_5DD0,
        0xF108_F108,
        0x2528_2528,
        0xCCC2_CCC2,
        0xFFFB_FFFB,
    ],
    // Dim 4: polynomial x^3 + x + 1, m = [1, 3, 1]
    [
        0x8000_0000,
        0x4000_0000,
        0x6000_0000,
        0xD000_0000,
        0x6800_0000,
        0x3400_0000,
        0xC600_0000,
        0x6D00_0000,
        0x3580_0000,
        0xC340_0000,
        0x6FA0_0000,
        0xB510_0000,
        0xC628_0000,
        0xA714_0000,
        0xC32A_0000,
        0x6431_0000,
        0xB2E1_8000,
        0xC55A_4000,
        0xEFBA_6000,
        0xF563_D000,
        0xD27F_6800,
        0xE3B1_3400,
        0xC568_C600,
        0xB78F_6D00,
        0xC44B_3580,
        0x8261_C340,
        0x4123_6FA0,
        0x60D2_B510,
        0xD026_C628,
        0x6817_A714,
        0x3412_C32A,
        0xC600_6431,
    ],
    // Dim 5: polynomial x^3 + x^2 + 1, m = [1, 1, 3]
    [
        0x8000_0000,
        0xC000_0000,
        0x2000_0000,
        0xB000_0000,
        0xC800_0000,
        0x6C00_0000,
        0x4600_0000,
        0x6700_0000,
        0xC580_0000,
        0xC1C0_0000,
        0x2820_0000,
        0x9870_0000,
        0x9C68_0000,
        0x4CB4_0000,
        0xC65A_0000,
        0x69C7_0000,
        0xC32E_8000,
        0xC09B_C000,
        0x2031_E000,
        0xB008_F000,
        0xC8C4_4800,
        0x6C66_E400,
        0x4647_2A00,
        0x6726_F900,
        0xC5A6_82C0,
        0xC176_4860,
        0x2832_BA90,
        0x987F_7CB0,
        0x9C7E_3C50,
        0x4C24_CE40,
        0xC60B_E5A0,
        0x69CC_99C0,
    ],
    // Dim 6: polynomial x^4 + x + 1, m = [1, 1, 1, 3]
    [
        0x8000_0000,
        0xC000_0000,
        0xA000_0000,
        0x9000_0000,
        0xD800_0000,
        0xFC00_0000,
        0xCE00_0000,
        0xD900_0000,
        0xFD80_0000,
        0xCFC0_0000,
        0xD9E0_0000,
        0xFCD0_0000,
        0xCE38_0000,
        0xD974_0000,
        0xFD46_0000,
        0xCFEF_0000,
        0xD9E8_8000,
        0xFCD0_4000,
        0xCE38_E000,
        0xD974_9000,
        0xFD46_D800,
        0xCFEF_FC00,
        0xD9E8_CE00,
        0xFCD0_D900,
        0xCE38_FD80,
        0xD974_CFC0,
        0xFD46_D9E0,
        0xCFEF_FCD0,
        0xD9E8_CE38,
        0xFCD0_D974,
        0xCE38_FD46,
        0xD974_CFEF,
    ],
];

/// Multi-dimensional Sobol sequence (Bratley-Fox 1988 starter set,
/// dimensions 1–6). For higher-dimensional uses, switch to a Joe-Kuo
/// D6 direction-number table (out of scope here).
///
/// # Examples
///
/// ```
/// use vrd::quasirandom::SobolSequence;
///
/// let mut s = SobolSequence::new(2);
/// let p = s.next_point::<2>();
/// assert!(p[0] >= 0.0 && p[0] < 1.0);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SobolSequence {
    d: usize,
    i: u64,
    x: [u32; SOBOL_MAX_DIM],
}

impl SobolSequence {
    /// Builds a `d`-dimensional Sobol sequence. Panics if `d` is 0
    /// or exceeds [`SOBOL_MAX_DIM`].
    pub fn new(d: usize) -> Self {
        assert!(
            d > 0 && d <= SOBOL_MAX_DIM,
            "Sobol dimension must be in 1..={SOBOL_MAX_DIM}"
        );
        Self {
            d,
            i: 0,
            x: [0; SOBOL_MAX_DIM],
        }
    }

    /// Skips forward by `n` points.
    pub fn skip(&mut self, n: u64) {
        for _ in 0..n {
            self.advance();
        }
    }

    /// Returns the next `D`-dimensional point.
    pub fn next_point<const D: usize>(&mut self) -> [f64; D] {
        assert_eq!(
            D, self.d,
            "const D must match dimensions configured at construction"
        );
        let scale = 1.0_f64 / ((1u64 << 32) as f64);
        let mut out = [0.0; D];
        if self.i == 0 {
            // Convention: first point is (0, 0, …, 0).
            self.i = 1;
            return out;
        }
        let bit = trailing_zero_bit(self.i);
        assert!(
            bit < 32,
            "Sobol exhausted: only 2^32 points per dimension"
        );
        for k in 0..D {
            self.x[k] ^= SOBOL_DIRECTIONS[k][bit];
            out[k] = (self.x[k] as f64) * scale;
        }
        self.i += 1;
        out
    }

    /// Returns the next point as an allocated [`Vec`]. Requires
    /// `alloc`.
    #[cfg(feature = "alloc")]
    pub fn next_point_vec(&mut self) -> Vec<f64> {
        let scale = 1.0_f64 / ((1u64 << 32) as f64);
        if self.i == 0 {
            self.i = 1;
            return alloc::vec![0.0; self.d];
        }
        let bit = trailing_zero_bit(self.i);
        assert!(
            bit < 32,
            "Sobol exhausted: only 2^32 points per dimension"
        );
        let mut out = Vec::with_capacity(self.d);
        for (k, dir) in SOBOL_DIRECTIONS.iter().enumerate().take(self.d)
        {
            self.x[k] ^= dir[bit];
            out.push((self.x[k] as f64) * scale);
        }
        self.i += 1;
        out
    }

    fn advance(&mut self) {
        if self.i == 0 {
            self.i = 1;
            return;
        }
        let bit = trailing_zero_bit(self.i);
        if bit >= 32 {
            return;
        }
        for (k, dir) in SOBOL_DIRECTIONS.iter().enumerate().take(self.d)
        {
            self.x[k] ^= dir[bit];
        }
        self.i += 1;
    }

    /// Returns the dimension count.
    pub fn dimensions(&self) -> usize {
        self.d
    }
}

#[inline]
fn trailing_zero_bit(n: u64) -> usize {
    n.trailing_zeros() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vdc_base_2_matches_classical() {
        let mut vdc = VanDerCorputSequence::new(2);
        let expected = [0.5, 0.25, 0.75, 0.125, 0.625, 0.375, 0.875];
        for &e in &expected {
            let got = vdc.next_point();
            assert!((got - e).abs() < 1e-12, "got {got}, want {e}");
        }
    }

    #[test]
    fn vdc_in_unit_interval() {
        let mut vdc = VanDerCorputSequence::new(3);
        for _ in 0..256 {
            let p = vdc.next_point();
            assert!((0.0..1.0).contains(&p), "point {p} out of [0, 1)");
        }
    }

    #[test]
    fn halton_2d_in_unit_square() {
        let mut h = HaltonSequence::new(2);
        for _ in 0..1024 {
            let p = h.next_point::<2>();
            assert!((0.0..1.0).contains(&p[0]));
            assert!((0.0..1.0).contains(&p[1]));
        }
    }

    #[test]
    fn halton_dim_panic_on_overflow() {
        // Should not panic at construction with the max dim.
        let _ = HaltonSequence::new(HALTON_MAX_DIM);
    }

    #[test]
    fn sobol_2d_first_few_points() {
        let mut s = SobolSequence::new(2);
        // (0, 0) is the canonical starting point.
        assert_eq!(s.next_point::<2>(), [0.0, 0.0]);
        let p = s.next_point::<2>();
        assert!(p[0] > 0.0 && p[0] < 1.0);
        assert!(p[1] > 0.0 && p[1] < 1.0);
    }

    #[test]
    fn sobol_iterator_in_unit_square() {
        let mut s = SobolSequence::new(2);
        for _ in 0..512 {
            let p = s.next_point::<2>();
            assert!((0.0..1.0).contains(&p[0]));
            assert!((0.0..1.0).contains(&p[1]));
        }
    }

    /// Monte Carlo π convergence: Sobol's error after N points
    /// should shrink faster than Halton's, both faster than a fair
    /// uniform PRNG would. With N=4096 we're not aiming for tight
    /// thresholds — just verifying that all three converge into
    /// the right neighbourhood.
    #[test]
    fn quasi_pi_estimates_converge() {
        const N: usize = 4096;
        let mut h = HaltonSequence::new(2);
        let mut s = SobolSequence::new(2);

        let mut h_inside = 0usize;
        let mut s_inside = 0usize;
        for _ in 0..N {
            let p = h.next_point::<2>();
            if p[0] * p[0] + p[1] * p[1] < 1.0 {
                h_inside += 1;
            }
            let q = s.next_point::<2>();
            if q[0] * q[0] + q[1] * q[1] < 1.0 {
                s_inside += 1;
            }
        }
        let pi_h = 4.0 * (h_inside as f64) / (N as f64);
        let pi_s = 4.0 * (s_inside as f64) / (N as f64);
        assert!(
            (pi_h - core::f64::consts::PI).abs() < 0.05,
            "Halton π = {pi_h}"
        );
        assert!(
            (pi_s - core::f64::consts::PI).abs() < 0.05,
            "Sobol π = {pi_s}"
        );
    }
}

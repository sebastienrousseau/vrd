// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! PCG (Permuted Congruential Generator) family — O'Neill, 2014.
//!
//! Two variants exported:
//!
//! - [`Pcg32`] — PCG-XSH-RR-64/32. 16-byte state, 32-bit output. The
//!   smallest-state member of the family; faster per `u32` than
//!   Xoshiro256++ on most CPUs.
//! - [`Pcg64`] — PCG-XSL-RR-128/64. 32-byte state, 64-bit output.
//!   Same state size as Xoshiro256++ with native 64-bit output.
//!
//! Both are statistically excellent (pass TestU01 BigCrush) but are
//! **not** CSPRNGs — same caveat as Xoshiro and MT.
//!
//! Reference: O'Neill, M. E. (2014), *PCG: A Family of Simple Fast
//! Space-Efficient Statistically Good Algorithms for Random Number
//! Generation*. <https://www.pcg-random.org/>.

use core::convert::Infallible;
use rand::rand_core::{SeedableRng, TryRng};

// ---------------------------------------------------------------------------
// PCG-XSH-RR-64/32
// ---------------------------------------------------------------------------

/// PCG-XSH-RR-64/32 generator. 16-byte state (one `u64` LCG state plus
/// a 64-bit stream-selecting increment); 32-bit output per call via
/// XOR-shift-high followed by random rotation.
///
/// # Examples
///
/// ```
/// use vrd::pcg::Pcg32;
///
/// let mut rng = Pcg32::from_u64_seed(0xCAFE_F00D);
/// let _ = rng.next_u32();
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

const PCG32_MULT: u64 = 6_364_136_223_846_793_005;
const PCG32_DEFAULT_INC: u64 = 1_442_695_040_888_963_407;

impl Pcg32 {
    /// Builds a `Pcg32` from a 64-bit seed and a 63-bit stream
    /// selector. Internally bumps the increment to the
    /// nearest odd value to ensure a full-period LCG.
    pub fn from_seed_stream(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (stream << 1) | 1,
        };
        let _ = rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        let _ = rng.next_u32();
        rng
    }

    /// Convenience constructor from a single `u64` seed; uses the
    /// canonical PCG default stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::pcg::Pcg32;
    ///
    /// let mut rng = Pcg32::from_u64_seed(42);
    /// assert_ne!(rng.next_u32(), 0);
    /// ```
    pub fn from_u64_seed(seed: u64) -> Self {
        Self::from_seed_stream(seed, PCG32_DEFAULT_INC)
    }

    /// Generates the next random `u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::pcg::Pcg32;
    ///
    /// let mut rng = Pcg32::from_u64_seed(1);
    /// let n = rng.next_u32();
    /// # let _ = n;
    /// ```
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let old = self.state;
        self.state =
            old.wrapping_mul(PCG32_MULT).wrapping_add(self.inc);
        // XSH-RR — XOR-shift-high then random-rotate.
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    /// Generates the next random `u64` by concatenating two `u32`s.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let lo = u64::from(self.next_u32());
        let hi = u64::from(self.next_u32());
        (hi << 32) | lo
    }

    /// Fills `dest` with random bytes.
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut i = 0;
        while i + 4 <= dest.len() {
            let bytes = self.next_u32().to_le_bytes();
            dest[i..i + 4].copy_from_slice(&bytes);
            i += 4;
        }
        if i < dest.len() {
            let bytes = self.next_u32().to_le_bytes();
            let remaining = dest.len() - i;
            dest[i..].copy_from_slice(&bytes[..remaining]);
        }
    }
}

impl TryRng for Pcg32 {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.next_u64())
    }

    fn try_fill_bytes(
        &mut self,
        dest: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for Pcg32 {
    type Seed = [u8; 16];

    fn from_seed(seed: [u8; 16]) -> Self {
        let state = u64::from_le_bytes([
            seed[0], seed[1], seed[2], seed[3], seed[4], seed[5],
            seed[6], seed[7],
        ]);
        let stream = u64::from_le_bytes([
            seed[8], seed[9], seed[10], seed[11], seed[12], seed[13],
            seed[14], seed[15],
        ]);
        Self::from_seed_stream(state, stream | 1)
    }
}

// ---------------------------------------------------------------------------
// PCG-XSL-RR-128/64
// ---------------------------------------------------------------------------

/// PCG-XSL-RR-128/64 generator. 32-byte state (one `u128` LCG state
/// plus a 128-bit increment); 64-bit output per call via
/// XOR-shift-low followed by random rotation.
///
/// # Examples
///
/// ```
/// use vrd::pcg::Pcg64;
///
/// let mut rng = Pcg64::from_u128_seed(0xCAFE_F00D);
/// let _ = rng.next_u64();
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Pcg64 {
    state: u128,
    inc: u128,
}

// Multiplier from O'Neill's reference C++ implementation
// (PCG-XSL-RR-128-64, default_multiplier<uint128_t>).
const PCG64_MULT: u128 = 0x2360_ED05_1FC6_5DA4_4385_DF64_9FCC_F645;
const PCG64_DEFAULT_INC: u128 =
    0x5851_F42D_4C95_7F2D_1405_7B7E_F767_814F;

impl Pcg64 {
    /// Builds a `Pcg64` from a 128-bit seed and a 127-bit stream
    /// selector. The increment is bumped to the nearest odd value
    /// to ensure a full-period LCG.
    pub fn from_seed_stream(seed: u128, stream: u128) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (stream << 1) | 1,
        };
        let _ = rng.next_u64();
        rng.state = rng.state.wrapping_add(seed);
        let _ = rng.next_u64();
        rng
    }

    /// Convenience constructor from a single `u128` seed using the
    /// canonical PCG default stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::pcg::Pcg64;
    ///
    /// let mut rng = Pcg64::from_u128_seed(42);
    /// assert_ne!(rng.next_u64(), 0);
    /// ```
    pub fn from_u128_seed(seed: u128) -> Self {
        Self::from_seed_stream(seed, PCG64_DEFAULT_INC)
    }

    /// Generates the next random `u64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::pcg::Pcg64;
    ///
    /// let mut rng = Pcg64::from_u128_seed(1);
    /// let n = rng.next_u64();
    /// # let _ = n;
    /// ```
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let old = self.state;
        self.state =
            old.wrapping_mul(PCG64_MULT).wrapping_add(self.inc);
        // XSL-RR — XOR-shift-low then random-rotate (6-bit rot).
        let half = ((old >> 64) as u64) ^ (old as u64);
        let rot = (old >> 122) as u32;
        half.rotate_right(rot)
    }

    /// Generates the next random `u32` by taking the high 32 bits.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Fills `dest` with random bytes.
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut i = 0;
        while i + 8 <= dest.len() {
            let bytes = self.next_u64().to_le_bytes();
            dest[i..i + 8].copy_from_slice(&bytes);
            i += 8;
        }
        if i < dest.len() {
            let bytes = self.next_u64().to_le_bytes();
            let remaining = dest.len() - i;
            dest[i..].copy_from_slice(&bytes[..remaining]);
        }
    }
}

impl TryRng for Pcg64 {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.next_u64())
    }

    fn try_fill_bytes(
        &mut self,
        dest: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for Pcg64 {
    type Seed = [u8; 32];

    fn from_seed(seed: [u8; 32]) -> Self {
        let state = u128::from_le_bytes([
            seed[0], seed[1], seed[2], seed[3], seed[4], seed[5],
            seed[6], seed[7], seed[8], seed[9], seed[10], seed[11],
            seed[12], seed[13], seed[14], seed[15],
        ]);
        let stream = u128::from_le_bytes([
            seed[16], seed[17], seed[18], seed[19], seed[20], seed[21],
            seed[22], seed[23], seed[24], seed[25], seed[26], seed[27],
            seed[28], seed[29], seed[30], seed[31],
        ]);
        Self::from_seed_stream(state, stream | 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pcg32_deterministic() {
        let mut a = Pcg32::from_u64_seed(42);
        let mut b = Pcg32::from_u64_seed(42);
        for _ in 0..16 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }

    #[test]
    fn pcg32_nonzero_from_zero_seed() {
        let mut rng = Pcg32::from_u64_seed(0);
        assert_ne!(rng.next_u32(), 0);
    }

    #[test]
    fn pcg32_fill_bytes_unaligned() {
        let mut rng = Pcg32::from_u64_seed(7);
        let mut buf = [0u8; 17];
        rng.fill_bytes(&mut buf);
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    fn pcg64_deterministic() {
        let mut a = Pcg64::from_u128_seed(0xDEAD_BEEF);
        let mut b = Pcg64::from_u128_seed(0xDEAD_BEEF);
        for _ in 0..16 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn pcg64_nonzero_from_zero_seed() {
        let mut rng = Pcg64::from_u128_seed(0);
        assert_ne!(rng.next_u64(), 0);
    }

    #[test]
    fn pcg64_fill_bytes_unaligned() {
        let mut rng = Pcg64::from_u128_seed(1);
        let mut buf = [0u8; 33];
        rng.fill_bytes(&mut buf);
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    fn pcg32_try_rng_impl() {
        let mut rng = Pcg32::from_u64_seed(1);
        assert!(rng.try_next_u32().is_ok());
        assert!(rng.try_next_u64().is_ok());
        let mut buf = [0u8; 8];
        assert!(rng.try_fill_bytes(&mut buf).is_ok());
    }

    #[test]
    fn pcg64_try_rng_impl() {
        let mut rng = Pcg64::from_u128_seed(1);
        assert!(rng.try_next_u32().is_ok());
        assert!(rng.try_next_u64().is_ok());
        let mut buf = [0u8; 8];
        assert!(rng.try_fill_bytes(&mut buf).is_ok());
    }

    #[test]
    fn pcg32_seedable_rng() {
        let seed = [3u8; 16];
        let mut a = <Pcg32 as SeedableRng>::from_seed(seed);
        let mut b = <Pcg32 as SeedableRng>::from_seed(seed);
        assert_eq!(a.next_u32(), b.next_u32());
    }

    #[test]
    fn pcg64_seedable_rng() {
        let seed = [3u8; 32];
        let mut a = <Pcg64 as SeedableRng>::from_seed(seed);
        let mut b = <Pcg64 as SeedableRng>::from_seed(seed);
        assert_eq!(a.next_u64(), b.next_u64());
    }
}

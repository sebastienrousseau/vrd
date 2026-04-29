// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Xoshiro256++ pseudo-random number generator.
//!
//! Small-state (32 bytes) generator with a period of 2^256 - 1, designed
//! by David Blackman and Sebastiano Vigna. See
//! <https://prng.di.unimi.it/xoshiro256plusplus.c>.
//!
//! Seeds are routed through SplitMix64 before priming the state, as
//! recommended by the original authors, so low-entropy seeds (e.g.
//! `[0u8; 32]` or `[1u8; 32]`) still yield well-distributed initial state.

use core::convert::Infallible;
use rand::rand_core::{SeedableRng, TryRng};

/// Xoshiro256++ generator state.
///
/// # Examples
///
/// ```
/// use vrd::xoshiro::Xoshiro256PlusPlus;
///
/// let rng = Xoshiro256PlusPlus::from_u64_seed(42);
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Xoshiro256PlusPlus {
    state: [u64; 4],
}

/// SplitMix64 — a fast, well-distributed 64-bit mixer used to whiten seed
/// material before priming the main generator. Constants from
/// <https://prng.di.unimi.it/splitmix64.c>.
#[inline]
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

impl Xoshiro256PlusPlus {
    /// Builds an instance from a 32-byte seed, whitened via SplitMix64.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::xoshiro::Xoshiro256PlusPlus;
    ///
    /// let seed = [0u8; 32];
    /// let mut rng = Xoshiro256PlusPlus::from_seed(seed);
    /// assert_ne!(rng.next_u64(), 0);
    /// ```
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let mut sm = u64::from_le_bytes([
            seed[0], seed[1], seed[2], seed[3], seed[4], seed[5],
            seed[6], seed[7],
        ]) ^ u64::from_le_bytes([
            seed[8], seed[9], seed[10], seed[11], seed[12], seed[13],
            seed[14], seed[15],
        ]) ^ u64::from_le_bytes([
            seed[16], seed[17], seed[18], seed[19], seed[20], seed[21],
            seed[22], seed[23],
        ]) ^ u64::from_le_bytes([
            seed[24], seed[25], seed[26], seed[27], seed[28], seed[29],
            seed[30], seed[31],
        ]);
        if sm == 0 {
            sm = 0xBAD1_DEAA_5BD1_CAFE;
        }
        let s0 = splitmix64(&mut sm);
        let s1 = splitmix64(&mut sm);
        let s2 = splitmix64(&mut sm);
        let s3 = splitmix64(&mut sm);
        Self {
            state: [s0, s1, s2, s3],
        }
    }

    /// Convenience constructor from a single `u64` seed.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::xoshiro::Xoshiro256PlusPlus;
    ///
    /// let mut rng = Xoshiro256PlusPlus::from_u64_seed(12345);
    /// assert_ne!(rng.next_u64(), 0);
    /// ```
    pub fn from_u64_seed(mut seed: u64) -> Self {
        if seed == 0 {
            seed = 0xBAD1_DEAA_5BD1_CAFE;
        }
        let s0 = splitmix64(&mut seed);
        let s1 = splitmix64(&mut seed);
        let s2 = splitmix64(&mut seed);
        let s3 = splitmix64(&mut seed);
        Self {
            state: [s0, s1, s2, s3],
        }
    }

    /// Generates the next random `u64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::xoshiro::Xoshiro256PlusPlus;
    ///
    /// let mut rng = Xoshiro256PlusPlus::from_u64_seed(42);
    /// let n = rng.next_u64();
    /// ```
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let res = self.state[0]
            .wrapping_add(self.state[3])
            .rotate_left(23)
            .wrapping_add(self.state[0]);

        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        res
    }

    /// Generates the next random `u32` by taking the high 32 bits of the
    /// next `u64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::xoshiro::Xoshiro256PlusPlus;
    ///
    /// let mut rng = Xoshiro256PlusPlus::from_u64_seed(42);
    /// let n = rng.next_u32();
    /// ```
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Fills `dest` with random bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::xoshiro::Xoshiro256PlusPlus;
    ///
    /// let mut rng = Xoshiro256PlusPlus::from_u64_seed(42);
    /// let mut buf = [0u8; 16];
    /// rng.fill_bytes(&mut buf);
    /// ```
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

    /// Advances the state by 2^128 calls to [`Self::next_u64`].
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::xoshiro::Xoshiro256PlusPlus;
    ///
    /// let mut rng = Xoshiro256PlusPlus::from_u64_seed(42);
    /// rng.jump();
    /// ```
    pub fn jump(&mut self) {
        const JUMP: [u64; 4] = [
            0x180E_C6D3_3CFD_0ABA,
            0xD5A6_1266_F0C9_392C,
            0xA958_2618_E03F_C9AA,
            0x39AB_DC45_29B1_661C,
        ];

        let mut s0 = 0u64;
        let mut s1 = 0u64;
        let mut s2 = 0u64;
        let mut s3 = 0u64;

        for &jump in &JUMP {
            for b in 0..64 {
                if (jump & (1u64 << b)) != 0 {
                    s0 ^= self.state[0];
                    s1 ^= self.state[1];
                    s2 ^= self.state[2];
                    s3 ^= self.state[3];
                }
                let _ = self.next_u64();
            }
        }

        self.state = [s0, s1, s2, s3];
    }
}

impl TryRng for Xoshiro256PlusPlus {
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

impl SeedableRng for Xoshiro256PlusPlus {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::from_seed(seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_nonzero_from_zero_seed() {
        let mut rng = Xoshiro256PlusPlus::from_seed([0u8; 32]);
        // SplitMix64 whitening must yield non-zero state.
        assert_ne!(rng.next_u64(), 0);
    }

    #[test]
    fn produces_nonzero_from_one_seed() {
        let mut rng = Xoshiro256PlusPlus::from_seed([1u8; 32]);
        assert_ne!(rng.next_u64(), 0);
    }

    #[test]
    fn deterministic_for_same_seed() {
        let seed = [42u8; 32];
        let mut a = Xoshiro256PlusPlus::from_seed(seed);
        let mut b = Xoshiro256PlusPlus::from_seed(seed);
        for _ in 0..16 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn jump_diverges_from_baseline() {
        let seed = [123u8; 32];
        let mut a = Xoshiro256PlusPlus::from_seed(seed);
        let mut b = Xoshiro256PlusPlus::from_seed(seed);
        a.jump();
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn fill_bytes_handles_unaligned_lengths() {
        let mut rng = Xoshiro256PlusPlus::from_seed([7u8; 32]);
        let mut buf = [0u8; 17]; // not a multiple of 8
        rng.fill_bytes(&mut buf);
        // At least one byte should be non-zero; the probability of all
        // zeros is 2^-136, so this is effectively a structural check.
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    fn next_u32_uses_high_bits() {
        let rng = Xoshiro256PlusPlus::from_seed([99u8; 32]);
        let high = (rng.clone().next_u64() >> 32) as u32;
        let mut copy = rng;
        assert_eq!(copy.next_u32(), high);
    }
}

#[cfg(test)]
mod additional_xoshiro_tests {
    use super::*;

    #[test]
    fn test_xoshiro_from_u64_seed() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(123);
        let mut rng2 = Xoshiro256PlusPlus::from_u64_seed(123);
        assert_eq!(rng.next_u64(), rng2.next_u64());
    }
}

#[cfg(test)]
mod xoshiro_coverage {
    use super::*;
    #[cfg(feature = "alloc")]
    use alloc::format;
    #[cfg(all(not(feature = "alloc"), feature = "std"))]
    use std::format;

    #[test]
    fn test_xoshiro_fill_bytes_large() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(42);
        let mut buf = [0u8; 100];
        rng.fill_bytes(&mut buf);
        assert!(buf.iter().any(|&x| x != 0));
    }

    #[test]
    fn test_xoshiro_next_u32() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(42);
        let _ = rng.next_u32();
    }

    #[test]
    fn test_xoshiro_debug_display() {
        let rng = Xoshiro256PlusPlus::from_u64_seed(42);
        let s = format!("{:?}", rng);
        assert!(s.contains("Xoshiro256PlusPlus"));
    }
}

#[cfg(test)]
mod xoshiro_final_coverage {
    use super::*;

    #[test]
    fn test_xoshiro_try_next() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(42);
        assert!(rng.try_next_u32().is_ok());
        assert!(rng.try_next_u64().is_ok());
    }

    #[test]
    fn test_xoshiro_seedable_rng() {
        let seed = [1u8; 32];
        let mut rng =
            <Xoshiro256PlusPlus as SeedableRng>::from_seed(seed);
        assert_ne!(rng.next_u64(), 0);
    }
}

#[cfg(test)]
mod xoshiro_zero_test {
    use super::*;

    #[test]
    fn test_xoshiro_all_zero_seed() {
        let seed = [0u8; 32];
        let mut rng = Xoshiro256PlusPlus::from_seed(seed);
        assert_ne!(rng.next_u64(), 0);
    }
}

#[cfg(test)]
mod xoshiro_edge_cases {
    use super::*;

    #[test]
    fn test_xoshiro_from_u64_seed_zero() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(0);
        assert_ne!(rng.next_u64(), 0);
    }
}

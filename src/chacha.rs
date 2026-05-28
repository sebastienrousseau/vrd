// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! ChaCha20-based CSPRNG backend (feature `crypto`).
//!
//! Thin wrapper around `rand_chacha::ChaCha20Rng` — the
//! `rand`-ecosystem reference implementation, audited via its
//! upstream maintainers. vrd does not roll its own crypto.
//!
//! ChaCha20 is a stream cipher with a 256-bit key and an
//! `2⁶⁴`-block period (sufficient for any single application).
//! It's the standard CSPRNG for security-adjacent random output:
//! session tokens, salts, UUIDs that need unpredictability.
//!
//! # CSPRNG contract
//!
//! Given an unpredictable seed (e.g. [`Random::new_secure`]'s OS
//! entropy), the output stream is computationally indistinguishable
//! from true random bytes to any polynomial-time observer that does
//! not see the seed.
//!
//! [`Random::new_secure`]: crate::Random::new_secure

use core::convert::Infallible;
use rand::rand_core::{Rng, SeedableRng, TryRng};
use rand_chacha::ChaCha20Rng;

/// ChaCha20-based CSPRNG state. Wraps `rand_chacha::ChaCha20Rng`
/// and exposes the same `TryRng` / `SeedableRng` surface as the
/// other vrd backends.
///
/// # Examples
///
/// ```
/// use vrd::chacha::ChaChaRng;
///
/// let mut rng = ChaChaRng::from_seed([0u8; 32]);
/// let _ = rng.next_u32();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ChaChaRng {
    /// Wrapped `rand_chacha::ChaCha20Rng` — vrd doesn't roll its
    /// own crypto, this is the audited reference implementation.
    inner: ChaCha20Rng,
}

impl ChaChaRng {
    /// Builds a CSPRNG from a deterministic 32-byte seed. Use this
    /// for reproducible tests; use [`Self::from_os_rng`] for real
    /// crypto-quality randomness.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::chacha::ChaChaRng;
    ///
    /// let mut rng = ChaChaRng::from_seed([1u8; 32]);
    /// assert_ne!(rng.next_u64(), 0);
    /// ```
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            inner: ChaCha20Rng::from_seed(seed),
        }
    }

    /// Seeds the CSPRNG from the operating system entropy source.
    /// Requires `std`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::chacha::ChaChaRng;
    ///
    /// # #[cfg(feature = "std")]
    /// # {
    /// let mut rng = ChaChaRng::from_os_rng();
    /// let _ = rng.next_u32();
    /// # }
    /// ```
    #[cfg(feature = "std")]
    pub fn from_os_rng() -> Self {
        let mut seed = [0u8; 32];
        for byte in &mut seed {
            *byte = rand::random::<u8>();
        }
        Self::from_seed(seed)
    }

    /// Generates the next random `u32`.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    /// Generates the next random `u64`.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    /// Fills `dest` with CSPRNG-grade random bytes.
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest);
    }
}

impl TryRng for ChaChaRng {
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

impl SeedableRng for ChaChaRng {
    type Seed = [u8; 32];

    fn from_seed(seed: [u8; 32]) -> Self {
        ChaChaRng::from_seed(seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_from_seed() {
        let mut a = ChaChaRng::from_seed([42u8; 32]);
        let mut b = ChaChaRng::from_seed([42u8; 32]);
        for _ in 0..16 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn fill_bytes_unaligned() {
        let mut rng = ChaChaRng::from_seed([7u8; 32]);
        let mut buf = [0u8; 65];
        rng.fill_bytes(&mut buf);
        assert!(buf.iter().any(|&b| b != 0));
    }

    /// Bit-exact compatibility with the reference
    /// `rand_chacha::ChaCha20Rng::from_seed` output. If this test
    /// fails the wrapper has somehow diverged from the upstream
    /// implementation.
    #[test]
    fn matches_reference_rand_chacha() {
        let seed = [9u8; 32];
        let mut ours = ChaChaRng::from_seed(seed);
        let mut reference = ChaCha20Rng::from_seed(seed);
        for _ in 0..32 {
            assert_eq!(ours.next_u64(), reference.next_u64());
        }
    }

    /// Exercises the `TryRng` and `SeedableRng` impls (covers the
    /// trait dispatch lines that the inherent-method tests miss).
    #[test]
    fn try_rng_and_seedable_impls() {
        let mut rng = <ChaChaRng as SeedableRng>::from_seed([1u8; 32]);
        assert!(TryRng::try_next_u32(&mut rng).is_ok());
        assert!(TryRng::try_next_u64(&mut rng).is_ok());
        let mut buf = [0u8; 16];
        assert!(TryRng::try_fill_bytes(&mut rng, &mut buf).is_ok());
        assert!(buf.iter().any(|&b| b != 0));
    }
}

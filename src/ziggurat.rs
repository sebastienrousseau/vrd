// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Ziggurat sampler for the standard normal distribution.
//!
//! Implements the 256-strip variant of Marsaglia & Tsang (2000),
//! *The Ziggurat Method for Generating Random Variables*,
//! *J. Stat. Software* **5**(8). The tables are pre-computed at build
//! time (see `build.rs`), so this module is a small, branch-light
//! sampler that costs **one `u32` draw, one table lookup, and one
//! `f64` multiply** in the ~99% fast-path case.
//!
//! Layout (matches Marsaglia's reference C, generalised from N=128 to
//! N=256, with the 2³¹ signed-int scale):
//!
//! - `ZIG_NORM_K[256]` — `u32` thresholds. Fast-accept when
//!   `hz.unsigned_abs() < K[i]`.
//! - `ZIG_NORM_W[256]` — `f64` per-bin x-scale; `x = hz as f64 * W[i]`.
//! - `ZIG_NORM_F[256]` — `f64` heights `exp(-X[i]²/2)`. Used by the
//!   overhang test for bins ≥ 1.
//! - Bin 0 (`i == 0`) handles the unbounded tail (`|x| > R`) by
//!   exponential rejection.

use crate::FloatExt;

include!(concat!(env!("OUT_DIR"), "/ziggurat_tables.rs"));

/// Minimal trait the Ziggurat sampler needs from a generator: one `u32`
/// for the bin draw and one uniform `[0, 1)` `f64` for the overhang /
/// tail branches.
pub(crate) trait NormalSource {
    /// Draws an arbitrary `u32`. Used for the bin index +
    /// signed magnitude in the Ziggurat fast path.
    fn next_u32(&mut self) -> u32;
    /// Draws a uniform `f64` in `[0, 1)`. Used by the overhang
    /// and tail rejection branches.
    fn next_f64(&mut self) -> f64;
}

/// Samples one standard normal `N(0, 1)` deviate via the Ziggurat
/// method. Multiply by `sigma` and add `mu` for the general case.
///
/// The fast path is one `u32` from the underlying generator plus two
/// table lookups; the overhang branch additionally draws an `f64` and
/// evaluates one `exp`; the tail branch falls back to Marsaglia's
/// exponential-rejection sampler.
#[inline]
pub(crate) fn sample_normal<S: NormalSource>(rng: &mut S) -> f64 {
    loop {
        let raw = rng.next_u32();
        let hz = raw as i32;
        let i = (raw & 0xff) as usize;
        let abs_hz = hz.unsigned_abs();

        if abs_hz < ZIG_NORM_K[i] {
            return f64::from(hz) * ZIG_NORM_W[i];
        }

        let x = f64::from(hz) * ZIG_NORM_W[i];

        if i == 0 {
            return tail(rng, hz < 0);
        }

        let u = rng.next_f64();
        if ZIG_NORM_F[i] + u * (ZIG_NORM_F[i - 1] - ZIG_NORM_F[i])
            < FloatExt::exp(-0.5 * x * x)
        {
            return x;
        }
    }
}

/// Marsaglia's tail sampler for `|x| > R`. Loops with ~1.27 iterations
/// on average and returns a sample with magnitude ≥ `ZIG_NORM_R`.
#[inline]
fn tail<S: NormalSource>(rng: &mut S, negative: bool) -> f64 {
    let r = ZIG_NORM_R;
    let inv_r = 1.0 / r;
    loop {
        let mut u1 = rng.next_f64();
        let mut u2 = rng.next_f64();
        if u1 == 0.0 {
            u1 = f64::MIN_POSITIVE;
        }
        if u2 == 0.0 {
            u2 = f64::MIN_POSITIVE;
        }
        let x = -FloatExt::ln(u1) * inv_r;
        let y = -FloatExt::ln(u2);
        if y + y >= x * x {
            let v = r + x;
            return if negative { -v } else { v };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xoshiro::Xoshiro256PlusPlus;

    // Golden vector — 16 bit-exact normal samples for seed
    // `Xoshiro256PlusPlus::from_u64_seed(0xCAFE_F00D)`. Drift here
    // means either the lookup tables changed or the sampler logic
    // did. Regenerate with the `print_golden` ignored test if either
    // change was intentional.
    const GOLDEN_NORMAL_BITS: [u64; 16] = [
        0x3FE317455E8CFE27,
        0x3FB8A63B77941D0B,
        0xBFCD571D78F62591,
        0x3FF0AFE4C8B994C8,
        0xBFC55A2D3DD7C8D4,
        0x3FE3430210FFA568,
        0xBFE085DC62FDAEB1,
        0x3FA0BC3D479A5311,
        0xBFF0D05B816ECDE6,
        0x3FE412E50088A80C,
        0x3FF52153C3D85878,
        0xBFE77CDEDD03AC33,
        0xBFEE7A6041517CA6,
        0xBFF1060A1F216922,
        0xC00116516984F9B6,
        0xBFD89369A117C5E1,
    ];

    impl NormalSource for Xoshiro256PlusPlus {
        fn next_u32(&mut self) -> u32 {
            Xoshiro256PlusPlus::next_u32(self)
        }
        fn next_f64(&mut self) -> f64 {
            let bits = self.next_u64() >> 11;
            (bits as f64) * (1.0 / ((1u64 << 53) as f64))
        }
    }

    #[test]
    fn finite_over_many_calls() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(0xC0FF_EE42);
        for _ in 0..200_000 {
            assert!(sample_normal(&mut rng).is_finite());
        }
    }

    #[test]
    fn tail_returns_beyond_r() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(7);
        let z = tail(&mut rng, false);
        assert!(z.abs() >= ZIG_NORM_R);
        let z = tail(&mut rng, true);
        assert!(z.abs() >= ZIG_NORM_R);
    }

    /// Forces both `u1 == 0.0` and `u2 == 0.0` guard branches in
    /// `tail()` — these are dead-code paths in practice (probability
    /// 2⁻⁵³) but the guards exist to avoid `ln(0) = -inf`. A custom
    /// NormalSource that returns 0.0 once then non-zero exercises
    /// the substitution to `f64::MIN_POSITIVE`.
    #[test]
    fn tail_handles_zero_uniforms() {
        struct ZeroFirstSource {
            calls: usize,
            xs: Xoshiro256PlusPlus,
        }
        impl NormalSource for ZeroFirstSource {
            fn next_u32(&mut self) -> u32 {
                self.xs.next_u32()
            }
            fn next_f64(&mut self) -> f64 {
                self.calls += 1;
                // First two calls return 0.0 to trip both guards;
                // subsequent calls fall back to the real Xoshiro
                // path so the rejection loop terminates.
                if self.calls <= 2 {
                    0.0
                } else {
                    let bits = self.xs.next_u64() >> 11;
                    (bits as f64) * (1.0 / ((1u64 << 53) as f64))
                }
            }
        }
        let mut rng = ZeroFirstSource {
            calls: 0,
            xs: Xoshiro256PlusPlus::from_u64_seed(5),
        };
        let z = tail(&mut rng, false);
        assert!(z.is_finite() && z.abs() >= ZIG_NORM_R);
    }

    /// Helper: emits 16 samples from a fixed seed in bit-exact form.
    /// Run with `cargo test --lib --release -- --ignored print_golden
    /// --nocapture` after intentional table changes to regenerate the
    /// golden vector below.
    #[test]
    #[ignore = "regenerator — run manually after intentional table changes"]
    fn print_golden() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(0xCAFE_F00D);
        std::println!("GOLDEN_NORMAL_BITS = [");
        for _ in 0..16 {
            let z = sample_normal(&mut rng);
            std::println!("    0x{:016X}u64, // {z}", z.to_bits());
        }
        std::println!("];");
    }

    /// Catches accidental drift in the Ziggurat lookup tables or
    /// sampler. The bits are bit-for-bit deterministic given a fixed
    /// seed because the algorithm never branches on data that
    /// wouldn't be re-derivable from the same RNG state.
    #[test]
    #[cfg(feature = "std")]
    fn golden_normal_vector_stable() {
        const GOLDEN: [u64; 16] = GOLDEN_NORMAL_BITS;
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(0xCAFE_F00D);
        for (i, &expected) in GOLDEN.iter().enumerate() {
            let got = sample_normal(&mut rng).to_bits();
            assert_eq!(
                got, expected,
                "sample {i} drifted: got 0x{got:016X}, want 0x{expected:016X}"
            );
        }
    }

    /// Distribution shape check — mean, variance, skewness, excess
    /// kurtosis over 200k samples must be close to 0/1/0/0 for a
    /// standard normal.
    #[test]
    fn moments_match_standard_normal() {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(0xDEAD_BEEF);
        let n = 200_000usize;

        let mut sum = 0.0_f64;
        let mut sum2 = 0.0_f64;
        let mut sum3 = 0.0_f64;
        let mut sum4 = 0.0_f64;
        for _ in 0..n {
            let z = sample_normal(&mut rng);
            sum += z;
            sum2 += z * z;
            sum3 += z * z * z;
            sum4 += z * z * z * z;
        }
        let inv_n = 1.0 / n as f64;
        let m1 = sum * inv_n;
        let m2 = sum2 * inv_n - m1 * m1;
        let m3 =
            sum3 * inv_n - 3.0 * m1 * sum2 * inv_n + 2.0 * m1 * m1 * m1;
        let m4 = sum4 * inv_n - 4.0 * m1 * sum3 * inv_n
            + 6.0 * m1 * m1 * sum2 * inv_n
            - 3.0 * m1 * m1 * m1 * m1;

        let std = m2.sqrt();
        let skew = m3 / (std * std * std);
        let kurt = m4 / (m2 * m2) - 3.0;

        assert!(m1.abs() < 0.02, "mean drifted: {m1}");
        assert!((std - 1.0).abs() < 0.02, "stddev drifted: {std}");
        assert!(skew.abs() < 0.05, "skewness drifted: {skew}");
        assert!(kurt.abs() < 0.1, "excess kurtosis drifted: {kurt}");
    }
}

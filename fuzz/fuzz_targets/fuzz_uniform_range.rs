//! Fuzz target: `Random::uniform(low, high)` on arbitrary
//! finite `f64` pairs. Asserts the result is finite and in
//! `[low, high)`. Skips inputs where `low >= high` (documented
//! programmer-error case that panics).

// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2023-2026 vrd contributors.

#![no_main]

use libfuzzer_sys::fuzz_target;
use vrd::Random;

fuzz_target!(|data: &[u8]| {
    if data.len() < 48 {
        return;
    }
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&data[..32]);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    a.copy_from_slice(&data[32..40]);
    b.copy_from_slice(&data[40..48]);
    let low = f64::from_le_bytes(a);
    let high = f64::from_le_bytes(b);

    // Skip pathological inputs: NaN, infinities, low >= high.
    if !low.is_finite() || !high.is_finite() || low >= high {
        return;
    }

    let mut rng = Random::from_seed(seed);
    for _ in 0..32 {
        let v = rng.uniform(low, high);
        assert!(v.is_finite(), "uniform produced non-finite");
        assert!(v >= low, "uniform produced {v} < low {low}");
        assert!(v < high, "uniform produced {v} >= high {high}");
    }
});

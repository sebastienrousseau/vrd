//! Fuzz target: Halton / Sobol with arbitrary dim + skip
//! counts. Asserts every emitted point lands in `[0, 1)^D`.

// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2023-2026 vrd contributors.

#![no_main]

use libfuzzer_sys::fuzz_target;
use vrd::quasirandom::{
    HaltonSequence, SobolSequence, VanDerCorputSequence,
    HALTON_MAX_DIM, SOBOL_MAX_DIM,
};

fuzz_target!(|data: &[u8]| {
    if data.len() < 4 {
        return;
    }
    let halton_dim = (data[0] as usize % HALTON_MAX_DIM) + 1;
    let sobol_dim = (data[1] as usize % SOBOL_MAX_DIM) + 1;
    let skip = data[2] as u64;
    let n = (data[3] as usize % 64) + 1;

    // Van der Corput.
    let mut vdc = VanDerCorputSequence::new(2);
    VanDerCorputSequence::skip(&mut vdc, skip);
    for _ in 0..n {
        let p = vdc.next_point();
        assert!((0.0..1.0).contains(&p), "VdC produced {p}");
    }

    // Halton — drop into next_point_vec for runtime-dim.
    let mut h = HaltonSequence::new(halton_dim);
    h.skip(skip);
    for _ in 0..n {
        let v = h.next_point_vec();
        assert_eq!(v.len(), halton_dim);
        for &x in &v {
            assert!((0.0..1.0).contains(&x), "Halton produced {x}");
        }
    }

    // Sobol — same pattern, runtime dim.
    let mut s = SobolSequence::new(sobol_dim);
    s.skip(skip);
    for _ in 0..n {
        let v = s.next_point_vec();
        assert_eq!(v.len(), sobol_dim);
        for &x in &v {
            assert!((0.0..1.0).contains(&x), "Sobol produced {x}");
        }
    }
});

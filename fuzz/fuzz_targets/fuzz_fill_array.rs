//! Fuzz target: drive `Random::try_fill_bytes` with arbitrary
//! length × seed. Asserts every returned slice has the
//! requested length (mostly tests the scalar bulk path; the
//! SIMD path is only built when the `simd` feature is on).

// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2023-2026 vrd contributors.

#![no_main]

use libfuzzer_sys::fuzz_target;
use vrd::Random;

fuzz_target!(|data: &[u8]| {
    if data.len() < 33 {
        return;
    }
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&data[..32]);
    // Cap fuzz buffer length at 2 KiB to keep iteration fast.
    let len = (data[32] as usize) * 8;

    let mut rng = Random::from_seed(seed);
    // Exercises vrd's heap bulk-byte path (alloc-only); the
    // returned Vec must always match the requested length.
    let buf = rng.bytes(len);
    assert_eq!(buf.len(), len);

    // Also exercise the const-generic stack path for a fixed
    // size — catches alignment / init regressions independent
    // of the alloc path.
    let _arr: [u8; 64] = rng.fill_array();
});

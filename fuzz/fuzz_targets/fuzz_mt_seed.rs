//! Fuzz target: drive MersenneTwister with arbitrary seed input.

// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2023-2026 vrd contributors.

#![no_main]

use libfuzzer_sys::fuzz_target;
use vrd::random::MersenneTwister;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&data[..32]);

    let mut mt = MersenneTwister::from_seed(seed);
    for _ in 0..64 {
        let _ = mt.rand();
        let _ = mt.next_u64();
    }
});

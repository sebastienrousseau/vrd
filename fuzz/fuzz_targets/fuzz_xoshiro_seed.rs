//! Fuzz target: drive Xoshiro256++ with an arbitrary 32-byte
//! seed, then 64 draws. Panics are bugs; the public API must
//! tolerate any input.

// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2023-2026 vrd contributors.

#![no_main]

use libfuzzer_sys::fuzz_target;
use vrd::xoshiro::Xoshiro256PlusPlus;

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&data[..32]);

    let mut rng = Xoshiro256PlusPlus::from_seed(seed);
    for _ in 0..64 {
        let _ = rng.next_u32();
        let _ = rng.next_u64();
    }

    let mut buf = [0u8; 128];
    rng.fill_bytes(&mut buf);
});

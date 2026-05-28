//! Fuzz target: drive both PCG variants with arbitrary seed
//! and stream-selector bytes.

// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2023-2026 vrd contributors.

#![no_main]

use libfuzzer_sys::fuzz_target;
use vrd::pcg::{Pcg32, Pcg64};

fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }

    // PCG32: 8 bytes seed + 8 bytes stream
    let mut s = [0u8; 8];
    let mut x = [0u8; 8];
    s.copy_from_slice(&data[0..8]);
    x.copy_from_slice(&data[8..16]);
    let mut p32 = Pcg32::from_seed_stream(u64::from_le_bytes(s), u64::from_le_bytes(x));
    for _ in 0..64 {
        let _ = p32.next_u32();
        let _ = p32.next_u64();
    }
    let mut buf = [0u8; 64];
    p32.fill_bytes(&mut buf);

    // PCG64: 16 bytes seed + 16 bytes stream — re-using data
    let mut s = [0u8; 16];
    let mut x = [0u8; 16];
    s.copy_from_slice(&data[0..16]);
    x.copy_from_slice(&data[16..32]);
    let mut p64 = Pcg64::from_seed_stream(u128::from_le_bytes(s), u128::from_le_bytes(x));
    for _ in 0..64 {
        let _ = p64.next_u32();
        let _ = p64.next_u64();
    }
    let mut buf = [0u8; 64];
    p64.fill_bytes(&mut buf);
});

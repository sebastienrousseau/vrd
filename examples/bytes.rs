// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Byte generation via `bytes()` and `try_fill_bytes`.
//!
//! Run: `cargo run --example bytes`

#[path = "support.rs"]
mod support;

use rand::rand_core::TryRng;
use vrd::Random;

fn main() {
    support::header("vrd -- bytes");

    support::task_with_output(
        "Random::bytes(n) returns a fresh Vec<u8>",
        || {
            let mut rng = Random::from_u64_seed(0xB17E5);
            let buf = rng.bytes(16);
            vec![
                format!("len = {}", buf.len()),
                format!("hex = {}", to_hex(&buf)),
            ]
        },
    );

    support::task_with_output(
        "try_fill_bytes fills any &mut [u8]",
        || {
            let mut rng = Random::from_u64_seed(0xB17E5);
            let mut buf = [0u8; 24];
            rng.try_fill_bytes(&mut buf).unwrap();
            vec![
                format!("len (unaligned to 8) = {}", buf.len()),
                format!("hex = {}", to_hex(&buf)),
            ]
        },
    );

    support::task_with_output("Same seed -> same bytes", || {
        let mut a = Random::from_u64_seed(7);
        let mut b = Random::from_u64_seed(7);
        let ba = a.bytes(8);
        let bb = b.bytes(8);
        vec![
            format!("a     = {}", to_hex(&ba)),
            format!("b     = {}", to_hex(&bb)),
            format!("equal = {}", ba == bb),
        ]
    });

    support::summary(3);
}

fn to_hex(b: &[u8]) -> String {
    b.iter()
        .map(|x| format!("{x:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

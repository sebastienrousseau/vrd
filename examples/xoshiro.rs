// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Direct use of the Xoshiro256++ generator (without the `Random`
//! facade), including the 2^128-step `jump` function for splitting
//! sequences across parallel workers.
//!
//! Run: `cargo run --example xoshiro`

#[path = "support.rs"]
mod support;

use vrd::xoshiro::Xoshiro256PlusPlus;

fn main() {
    support::header("vrd -- xoshiro");

    support::task_with_output("Direct construction and draws", || {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(1);
        vec![
            format!("next_u64 = {}", rng.next_u64()),
            format!("next_u32 = {}", rng.next_u32()),
        ]
    });

    support::task_with_output(
        "SplitMix64 whitens low-entropy seeds",
        || {
            let mut zeros = Xoshiro256PlusPlus::from_seed([0u8; 32]);
            let mut ones = Xoshiro256PlusPlus::from_seed([1u8; 32]);
            vec![
                format!(
                    "seed [0;32]: first u64 = {}",
                    zeros.next_u64()
                ),
                format!("seed [1;32]: first u64 = {}", ones.next_u64()),
                "Both non-zero — proves SplitMix64 whitening works."
                    .into(),
            ]
        },
    );

    support::task_with_output(
        "jump() advances by 2^128, partitioning sequences",
        || {
            let mut a = Xoshiro256PlusPlus::from_u64_seed(2024);
            let mut b = Xoshiro256PlusPlus::from_u64_seed(2024);
            b.jump();
            vec![
                format!("a (no jump):     {}", a.next_u64()),
                format!("b (jumped 2^128): {}", b.next_u64()),
                "Use jump() to give each thread its own subsequence."
                    .into(),
            ]
        },
    );

    support::task_with_output(
        "fill_bytes handles unaligned lengths",
        || {
            let mut rng = Xoshiro256PlusPlus::from_u64_seed(7);
            let mut buf = [0u8; 17];
            rng.fill_bytes(&mut buf);
            vec![
                format!("len = {}", buf.len()),
                format!("hex = {}", hex(&buf)),
            ]
        },
    );

    support::summary(4);
}

fn hex(b: &[u8]) -> String {
    b.iter()
        .map(|x| format!("{x:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

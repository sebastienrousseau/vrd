// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Iterator-style adapters: `iter_u32`, `iter_u64`, `iter_bytes`.
//!
//! Run: `cargo run --example iterators`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- iterators");

    support::task_with_output("iter_u32().take(5)", || {
        let mut rng = Random::from_u64_seed(1);
        let xs: Vec<u32> = rng.iter_u32().take(5).collect();
        vec![format!("{xs:?}")]
    });

    support::task_with_output("iter_u64().take(5)", || {
        let mut rng = Random::from_u64_seed(1);
        let xs: Vec<u64> = rng.iter_u64().take(5).collect();
        vec![format!("{xs:?}")]
    });

    support::task_with_output("iter_bytes().take(16)", || {
        let mut rng = Random::from_u64_seed(1);
        let bytes: Vec<u8> = rng.iter_bytes().take(16).collect();
        let hex: String = bytes
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join("");
        vec![format!("hex = {hex}")]
    });

    support::task_with_output(
        "Iterator combinators (take, filter, sum)",
        || {
            let mut rng = Random::from_u64_seed(1);
            let evens: Vec<u32> = rng
                .iter_u32()
                .take(50)
                .filter(|n| n % 2 == 0)
                .take(5)
                .collect();
            vec![
                format!("first 5 even draws = {evens:?}"),
                format!("count               = {}", evens.len()),
            ]
        },
    );

    support::summary(4);
}

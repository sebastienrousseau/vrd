// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Smallest possible vrd program.
//!
//! Run: `cargo run --example hello`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- hello");

    support::task_with_output(
        "Random::new() — entropy-seeded Xoshiro",
        || {
            let mut rng = Random::new();
            vec![
                format!("u32 = {}", rng.rand()),
                format!("u64 = {}", rng.u64()),
                format!("f64 = {:.6}", rng.double()),
            ]
        },
    );

    support::task_with_output("Default backend display", || {
        let rng = Random::new();
        vec![format!("{rng}")]
    });

    support::summary(2);
}

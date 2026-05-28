// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! PCG32 / PCG64 backends - small-state, fast, statistically
//! excellent (TestU01 BigCrush clean). Not a CSPRNG.
//!
//! Run: `cargo run --example pcg --features pcg`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- pcg");

    support::task_with_output(
        "PCG32 - 16-byte state, 32-bit output",
        || {
            let mut rng = Random::new_pcg32_with_seed(42);
            (0..4)
                .map(|i| format!("draw {i}: {:>12}", rng.rand()))
                .collect()
        },
    );

    support::task_with_output(
        "PCG64 - 32-byte state, native 64-bit output",
        || {
            let mut rng = Random::new_pcg64_with_seed(0xCAFE_F00D);
            (0..4)
                .map(|i| format!("draw {i}: {:>20}", rng.u64()))
                .collect()
        },
    );

    support::task_with_output(
        "Deterministic across the dispatch facade",
        || {
            let mut a = Random::new_pcg32_with_seed(7);
            let mut b = Random::new_pcg32_with_seed(7);
            let av = a.rand();
            let bv = b.rand();
            vec![
                format!("a = {av}"),
                format!("b = {bv}"),
                format!("equal = {}", av == bv),
            ]
        },
    );

    support::summary(3);
}

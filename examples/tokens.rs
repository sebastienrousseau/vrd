// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Hex and URL-safe base64 token generation.
//!
//! **Not a CSPRNG.** For security-sensitive tokens (session IDs,
//! credentials), use `rand::rngs::OsRng` or `getrandom`. This example
//! covers non-security tokens — log correlation IDs, debug trace
//! markers, ephemeral test fixtures.
//!
//! Run: `cargo run --example tokens`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- tokens");

    support::task_with_output(
        "hex_token(16) — 32 lowercase hex chars",
        || {
            let mut rng = Random::from_u64_seed(0x70CE);
            (0..5).map(|_| rng.hex_token(16)).collect()
        },
    );

    support::task_with_output(
        "base64_token(15) — 20 URL-safe base64 chars (no padding)",
        || {
            let mut rng = Random::from_u64_seed(0x70CE);
            (0..5).map(|_| rng.base64_token(15)).collect()
        },
    );

    support::task_with_output(
        "Variable lengths via base64_token",
        || {
            let mut rng = Random::from_u64_seed(0x70CE);
            [8usize, 16, 24, 32, 64]
                .iter()
                .map(|&n| {
                    let t = rng.base64_token(n);
                    format!("{n:>3} bytes -> {} chars: {t}", t.len())
                })
                .collect()
        },
    );

    support::summary(3);
}

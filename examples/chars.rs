// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Random characters: lowercase via `char()`, alphanumeric via macro.
//!
//! Run: `cargo run --example chars`

#[path = "support.rs"]
mod support;

use vrd::{rand_alphanumeric, Random};

fn main() {
    support::header("vrd -- chars");

    support::task_with_output("char() is always lowercase a-z", || {
        let mut rng = Random::from_u64_seed(2024);
        let s: String = (0..32).map(|_| rng.char()).collect();
        vec![
            format!("32 chars = {s}"),
            format!(
                "all lowercase = {}",
                s.chars().all(|c| c.is_ascii_lowercase())
            ),
        ]
    });

    support::task_with_output(
        "rand_alphanumeric! covers upper, lower, digits",
        || {
            let mut rng = Random::from_u64_seed(2024);
            let s: String =
                (0..32).map(|_| rand_alphanumeric!(rng)).collect();
            vec![
                format!("32 chars = {s}"),
                format!(
                    "all alphanumeric = {}",
                    s.chars().all(|c| c.is_ascii_alphanumeric())
                ),
            ]
        },
    );

    support::summary(2);
}

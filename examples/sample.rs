// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Sample without replacement (partial Fisher-Yates) and with
//! replacement.
//!
//! Run: `cargo run --example sample`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- sample");

    support::task_with_output(
        "sample without replacement: all unique",
        || {
            let mut rng = Random::from_u64_seed(0xDEC);
            let pool: Vec<u32> = (1..=20).collect();
            let picks = rng.sample(&pool, 5);
            let mut as_vals: Vec<u32> =
                picks.iter().map(|r| **r).collect();
            as_vals.sort_unstable();
            let unique = {
                let mut sorted = as_vals.clone();
                sorted.dedup();
                sorted.len() == as_vals.len()
            };
            vec![
                format!("picks = {as_vals:?}"),
                format!("all unique = {unique}"),
            ]
        },
    );

    support::task_with_output(
        "sample_with_replacement allows repeats",
        || {
            let mut rng = Random::from_u64_seed(0xDEC);
            let pool = ["alpha", "beta", "gamma"];
            let picks = rng.sample_with_replacement(&pool, 8);
            let names: Vec<_> = picks.iter().map(|r| **r).collect();
            vec![format!("picks = {names:?}")]
        },
    );

    support::summary(2);
}

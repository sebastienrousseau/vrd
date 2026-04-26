// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Exponential distribution sampling.
//!
//! Run: `cargo run --example exponential`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- exponential");

    support::task_with_output(
        "exponential(1.0) — first 8 samples",
        || {
            let mut rng = Random::from_u64_seed(0xEFFE);
            (0..8)
                .map(|_| format!("{:.6}", rng.exponential(1.0)))
                .collect()
        },
    );

    support::task_with_output("Mean approaches 1/rate", || {
        let n = 50_000usize;
        let mut lines = Vec::new();
        for &rate in &[0.5, 1.0, 2.0, 4.0] {
            let mut rng = Random::from_u64_seed(0xEFFE);
            let mean: f64 =
                (0..n).map(|_| rng.exponential(rate)).sum::<f64>()
                    / n as f64;
            let expected = 1.0 / rate;
            lines.push(format!(
                "rate = {rate:.1} -> mean = {mean:.4} (expected {expected:.4})",
            ));
        }
        lines
    });

    support::summary(2);
}

// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Random bools with arbitrary `true`-probability.
//!
//! Run: `cargo run --example bools`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- bools");

    support::task_with_output(
        "Edge cases: 0.0 and 1.0 are deterministic",
        || {
            let mut rng = Random::from_u64_seed(99);
            vec![
                format!(
                    "bool(0.0) x 5 = {:?}",
                    (0..5).map(|_| rng.bool(0.0)).collect::<Vec<_>>()
                ),
                format!(
                    "bool(1.0) x 5 = {:?}",
                    (0..5).map(|_| rng.bool(1.0)).collect::<Vec<_>>()
                ),
            ]
        },
    );

    support::task_with_output(
        "Empirical rate matches probability",
        || {
            let n = 20_000usize;
            let mut lines = Vec::new();
            for &p in &[0.10, 0.25, 0.50, 0.75, 0.90] {
                let mut rng = Random::from_u64_seed(42);
                let trues = (0..n).filter(|_| rng.bool(p)).count();
                let observed = trues as f64 / n as f64;
                lines.push(format!(
                "p = {p:.2} -> observed = {observed:.4} (|err| = {:.4})",
                (observed - p).abs()
            ));
            }
            lines
        },
    );

    support::summary(2);
}

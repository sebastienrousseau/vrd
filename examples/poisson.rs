// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Poisson distribution sampling via Knuth's multiplicative algorithm.
//!
//! Run: `cargo run --example poisson`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- poisson");

    support::task_with_output("poisson(3.0) — first 12 samples", || {
        let mut rng = Random::from_u64_seed(0xF15);
        (0..12)
            .map(|_| format!("{}", rng.poisson(3.0)))
            .collect()
    });

    support::task_with_output("Mean approaches lambda", || {
        let n = 30_000usize;
        let mut lines = Vec::new();
        for &lambda in &[0.5, 1.0, 3.0, 7.0] {
            let mut rng = Random::from_u64_seed(0xF15);
            let mean: f64 = (0..n)
                .map(|_| rng.poisson(lambda) as f64)
                .sum::<f64>()
                / n as f64;
            lines.push(format!(
                "lambda = {lambda:.1} -> mean = {mean:.4}",
            ));
        }
        lines
    });

    support::task_with_output("Histogram for lambda = 4.0", || {
        let mut rng = Random::from_u64_seed(7);
        let mut bins = [0u64; 16];
        for _ in 0..5_000 {
            let k = rng.poisson(4.0) as usize;
            if k < bins.len() {
                bins[k] += 1;
            }
        }
        let max = *bins.iter().max().unwrap_or(&1);
        bins.iter()
            .enumerate()
            .map(|(k, &c)| {
                let bar = "\u{2588}"
                    .repeat((c as f64 / max as f64 * 40.0) as usize);
                format!("k = {k:>2}: {bar}")
            })
            .collect()
    });

    support::summary(3);
}

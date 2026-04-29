// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Normal (Gaussian) distribution sampling via Box-Muller.
//!
//! Run: `cargo run --example normal`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- normal");

    support::task_with_output(
        "normal(0, 1) — first 8 samples",
        || {
            let mut rng = Random::from_u64_seed(0xBE11);
            (0..8)
                .map(|_| format!("{:.6}", rng.normal(0.0, 1.0)))
                .collect()
        },
    );

    support::task_with_output(
        "Bulk mean and stddev approach the parameters",
        || {
            let mut rng = Random::from_u64_seed(0xBE11);
            let n = 50_000usize;
            let mu = 1.5;
            let sigma = 2.0;
            let samples: Vec<f64> =
                (0..n).map(|_| rng.normal(mu, sigma)).collect();
            let mean: f64 = samples.iter().sum::<f64>() / n as f64;
            let var: f64 =
                samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
                    / n as f64;
            let std = var.sqrt();
            vec![
                format!("samples = {n}"),
                format!("mean    = {mean:.4} (expected {mu})"),
                format!("stddev  = {std:.4} (expected {sigma})"),
            ]
        },
    );

    support::task_with_output(
        "Tiny ASCII histogram of N(0, 1)",
        || {
            let mut rng = Random::from_u64_seed(7);
            let mut bins = [0u32; 21];
            for _ in 0..10_000 {
                let x = rng.normal(0.0, 1.0);
                let idx =
                    ((x + 5.0) * 2.0).round().clamp(0.0, 20.0) as usize;
                bins[idx] += 1;
            }
            let max = *bins.iter().max().unwrap_or(&1);
            bins.iter()
                .enumerate()
                .map(|(i, &c)| {
                    let width = (c as f64 / max as f64 * 40.0) as usize;
                    let bar = "\u{2588}".repeat(width);
                    let z = (i as f64) / 2.0 - 5.0;
                    format!("{z:>+5.1} {bar}")
                })
                .collect()
        },
    );

    support::summary(3);
}

// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Monte Carlo estimate of π via dart-throwing inside a unit square.
//!
//! Run: `cargo run --example monte`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn estimate_pi(rng: &mut Random, n: usize) -> f64 {
    let mut inside = 0u64;
    for _ in 0..n {
        let x = rng.f64();
        let y = rng.f64();
        if x * x + y * y < 1.0 {
            inside += 1;
        }
    }
    4.0 * inside as f64 / n as f64
}

fn main() {
    support::header("vrd -- monte");

    support::task_with_output(
        "Convergence as sample count grows",
        || {
            let truth = core::f64::consts::PI;
            [10_000usize, 100_000, 1_000_000]
                .iter()
                .map(|&n| {
                    let mut rng =
                        Random::from_u64_seed(0x3141_5926_5358_9793);
                    let est = estimate_pi(&mut rng, n);
                    format!(
                        "n = {n:>8}: π ≈ {est:.6} (|err| = {:.6})",
                        (est - truth).abs()
                    )
                })
                .collect()
        },
    );

    support::task_with_output(
        "Independent runs for variance check",
        || {
            (0..5)
                .map(|i| {
                    let mut rng = Random::from_u64_seed(i as u64 + 1);
                    let est = estimate_pi(&mut rng, 200_000);
                    format!("run {i}: π ≈ {est:.6}")
                })
                .collect()
        },
    );

    support::summary(2);
}

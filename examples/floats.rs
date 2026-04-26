// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Float generation: 24-bit `f32` and 53-bit `f64`, both in `[0.0, 1.0)`.
//!
//! Run: `cargo run --example floats`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- floats");

    support::task_with_output(
        "float() — 24-bit f32 in [0.0, 1.0)",
        || {
            let mut rng = Random::from_u64_seed(11);
            let samples: Vec<f32> =
                (0..6).map(|_| rng.float()).collect();
            let mut lines: Vec<_> =
                samples.iter().map(|f| format!("{f:.10}")).collect();
            lines.push(format!(
                "all in [0,1) = {}",
                samples.iter().all(|&f| (0.0..1.0).contains(&f))
            ));
            lines
        },
    );

    support::task_with_output(
        "double() — 53-bit f64 in [0.0, 1.0)",
        || {
            let mut rng = Random::from_u64_seed(11);
            let samples: Vec<f64> =
                (0..6).map(|_| rng.double()).collect();
            let mut lines: Vec<_> =
                samples.iter().map(|f| format!("{f:.16}")).collect();
            lines.push(format!(
                "all in [0,1) = {}",
                samples.iter().all(|&f| (0.0..1.0).contains(&f))
            ));
            lines
        },
    );

    support::task_with_output("Bulk mean approaches 0.5", || {
        let mut rng = Random::from_u64_seed(11);
        let n = 50_000usize;
        let mean: f64 =
            (0..n).map(|_| rng.f64()).sum::<f64>() / n as f64;
        vec![
            format!("samples = {n}"),
            format!("mean    = {mean:.5} (expected ~0.5)"),
            format!("|err|   = {:.5}", (mean - 0.5).abs()),
        ]
    });

    support::summary(3);
}

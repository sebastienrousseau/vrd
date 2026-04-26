// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Bounded sampling: `bounded`, `random_range`, `int`, `uint`, `range`.
//!
//! All paths use Lemire's nearly-divisionless method, so output is
//! uniform even when the range doesn't divide `2^32` cleanly.
//! See `unbiased` for a statistical demonstration.
//!
//! Run: `cargo run --example bounded`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- bounded");

    support::task_with_output(
        "bounded(range) — half-open [0, range)",
        || {
            let mut rng = Random::from_u64_seed(1);
            let s: Vec<u32> =
                (0..8).map(|_| rng.bounded(100)).collect();
            vec![
                format!("samples = {s:?}"),
                format!("all < 100 = {}", s.iter().all(|&v| v < 100)),
            ]
        },
    );

    support::task_with_output(
        "random_range(min, max) — [min, max)",
        || {
            let mut rng = Random::from_u64_seed(2);
            let s: Vec<u32> =
                (0..8).map(|_| rng.random_range(50, 60)).collect();
            vec![
                format!("samples = {s:?}"),
                format!(
                    "all in [50, 60) = {}",
                    s.iter().all(|&v| (50..60).contains(&v))
                ),
            ]
        },
    );

    support::task_with_output(
        "int(min, max) — i32 inclusive",
        || {
            let mut rng = Random::from_u64_seed(3);
            let s: Vec<i32> = (0..8).map(|_| rng.int(-5, 5)).collect();
            vec![
                format!("samples = {s:?}"),
                format!(
                    "all in [-5, 5] = {}",
                    s.iter().all(|&v| (-5..=5).contains(&v))
                ),
            ]
        },
    );

    support::task_with_output(
        "uint(min, max) — u32 inclusive",
        || {
            let mut rng = Random::from_u64_seed(4);
            let s: Vec<u32> =
                (0..8).map(|_| rng.uint(1000, 1100)).collect();
            vec![
                format!("samples = {s:?}"),
                format!(
                    "all in [1000, 1100] = {}",
                    s.iter().all(|&v| (1000..=1100).contains(&v))
                ),
            ]
        },
    );

    support::summary(4);
}

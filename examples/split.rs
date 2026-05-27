// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `Random::split()` — derive a parallel-safe sibling stream that
//! starts 2¹²⁸ calls ahead of the parent. Each half is independent;
//! both remain valid.
//!
//! Run: `cargo run --example split`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- split");

    support::task_with_output(
        "split() returns an independent sibling stream",
        || {
            let mut parent = Random::from_u64_seed(0x00C0_FFEE);
            let mut child = parent.split().expect("Xoshiro backend");
            (0..4)
                .map(|i| {
                    format!(
                        "draw {i}: parent = {:>20}, child = {:>20}",
                        parent.u64(),
                        child.u64()
                    )
                })
                .collect()
        },
    );

    support::task_with_output(
        "Fan-out: derive N siblings for N workers",
        || {
            let mut master = Random::from_u64_seed(42);
            let workers: Vec<Random> = (0..4)
                .map(|_| {
                    master
                        .split()
                        .expect("Xoshiro split always succeeds")
                })
                .collect();
            // Imagine each `workers[i]` going to a thread; here we
            // just show the first sample from each.
            workers
                .into_iter()
                .enumerate()
                .map(|(i, mut w)| {
                    format!("worker {i}: first u64 = {}", w.u64())
                })
                .collect()
        },
    );

    support::task_with_output(
        "split() on Mersenne Twister returns None",
        || {
            let mut rng = Random::new_mersenne_twister_with_seed(1);
            vec![format!("split() = {:?}", rng.split().is_some())]
        },
    );

    support::summary(3);
}

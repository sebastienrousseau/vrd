// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Empirical proof that bounded sampling is unbiased (Lemire's method).
//!
//! For a range that doesn't divide `2^32` cleanly, naive `rand() % range`
//! produces visibly skewed bucket counts. `Random::bounded` does not.
//!
//! Run: `cargo run --example unbiased`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- unbiased");

    let n = 1_000_000usize;
    let range = 7u32;

    support::task_with_output("bounded(7) — bucket counts", || {
        let mut rng = Random::from_u64_seed(0xD15EA5E);
        let mut counts = [0u64; 7];
        for _ in 0..n {
            counts[rng.bounded(range) as usize] += 1;
        }
        report(n, &counts)
    });

    support::task_with_output(
        "Naive `rand() % 7` for comparison",
        || {
            let mut rng = Random::from_u64_seed(0xD15EA5E);
            let mut counts = [0u64; 7];
            for _ in 0..n {
                counts[(rng.rand() % range) as usize] += 1;
            }
            report(n, &counts)
        },
    );

    support::summary(2);
}

fn report(n: usize, counts: &[u64; 7]) -> Vec<String> {
    let expected = n as f64 / 7.0;
    let mut max_dev = 0.0f64;
    let mut lines: Vec<_> = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let dev = (c as f64 - expected).abs() / expected;
            if dev > max_dev {
                max_dev = dev;
            }
            format!("bucket {i}: {c:>7} (deviation {dev:.4})")
        })
        .collect();
    lines.push(format!("max deviation = {max_dev:.4}"));
    lines
}

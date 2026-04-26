// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Pick a random element from a slice.
//!
//! Run: `cargo run --example choose`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- choose");

    support::task_with_output(
        "choose(&slice) returns Some(&T)",
        || {
            let mut rng = Random::from_u64_seed(99);
            let colors = ["red", "green", "blue", "yellow", "magenta"];
            (0..6)
                .map(|i| {
                    let pick = rng.choose(&colors).unwrap();
                    format!("draw {i}: {pick}")
                })
                .collect()
        },
    );

    support::task_with_output("Empty slice yields None", || {
        let mut rng = Random::from_u64_seed(1);
        let empty: [i32; 0] = [];
        vec![format!("choose(&[]) = {:?}", rng.choose(&empty))]
    });

    support::task_with_output(
        "Distribution is roughly uniform",
        || {
            let mut rng = Random::from_u64_seed(2024);
            let pool = ["A", "B", "C", "D"];
            let mut counts = [0u32; 4];
            for _ in 0..10_000 {
                let p = rng.choose(&pool).unwrap();
                let i = pool.iter().position(|x| x == p).unwrap();
                counts[i] += 1;
            }
            counts
                .iter()
                .enumerate()
                .map(|(i, &c)| {
                    format!(
                        "{} -> {c} ({:.2}%)",
                        pool[i],
                        c as f64 / 100.0
                    )
                })
                .collect()
        },
    );

    support::summary(3);
}

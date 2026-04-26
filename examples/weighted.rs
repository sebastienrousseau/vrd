// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Weighted sampling via the `rand_weighted_choice!` macro.
//!
//! Run: `cargo run --example weighted`

#[path = "support.rs"]
mod support;

use vrd::{rand_weighted_choice, Random};

fn main() {
    support::header("vrd -- weighted");

    support::task_with_output("Weighted picks converge to the weights", || {
        let mut rng = Random::from_u64_seed(0xCAFE);
        let labels = ["common", "uncommon", "rare", "legendary"];
        let weights: [u32; 4] = [60, 25, 10, 5];

        let n = 20_000usize;
        let mut counts = [0u32; 4];
        for _ in 0..n {
            let pick: &str =
                rand_weighted_choice!(rng, &labels, &weights);
            let i = labels
                .iter()
                .position(|x| *x == pick)
                .expect("known label");
            counts[i] += 1;
        }

        counts
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let observed = c as f64 / n as f64 * 100.0;
                format!(
                    "{:<10} expected {:>3}%  observed {:>5.2}%",
                    labels[i], weights[i], observed
                )
            })
            .collect()
    });

    support::summary(1);
}

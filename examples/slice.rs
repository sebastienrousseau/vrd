// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Random contiguous subslice via `rand_slice`.
//!
//! Run: `cargo run --example slice`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- slice");

    support::task_with_output(
        "rand_slice picks a contiguous window",
        || {
            let mut rng = Random::from_u64_seed(0xC1FF);
            let pool: Vec<u32> = (1..=12).collect();
            (0..5)
                .map(|i| {
                    let win = rng.rand_slice(&pool, 4).unwrap();
                    format!("draw {i}: {win:?}")
                })
                .collect()
        },
    );

    support::task_with_output("Edge cases return Err", || {
        let mut rng = Random::from_u64_seed(0);
        let pool: [i32; 0] = [];
        let pool_4 = [1, 2, 3, 4];
        vec![
            format!(
                "rand_slice(&[], 1) = {:?}",
                rng.rand_slice(&pool, 1)
            ),
            format!(
                "rand_slice(&[1..4], 0) = {:?}",
                rng.rand_slice(&pool_4, 0)
            ),
            format!(
                "rand_slice(&[1..4], 5) = {:?}",
                rng.rand_slice(&pool_4, 5)
            ),
        ]
    });

    support::summary(2);
}

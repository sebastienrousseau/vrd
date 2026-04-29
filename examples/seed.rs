// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Deterministic seeding: same seed in, same sequence out.
//!
//! Run: `cargo run --example seed`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- seed");

    support::task_with_output(
        "from_u64_seed reproduces the sequence",
        || {
            let mut a = Random::from_u64_seed(0xCAFE_BABE);
            let mut b = Random::from_u64_seed(0xCAFE_BABE);
            (0..4)
            .map(|i| {
                let av = a.rand();
                let bv = b.rand();
                format!("draw {i}: a = {av:>10}, b = {bv:>10}, equal = {}", av == bv)
            })
            .collect()
        },
    );

    support::task_with_output(
        "from_seed accepts a 32-byte seed",
        || {
            let seed = *b"vrd-deterministic-32-byte-seed!!";
            let mut rng = Random::from_seed(seed);
            vec![
                format!("seed = {} bytes", seed.len()),
                format!("first  = {}", rng.rand()),
                format!("second = {}", rng.rand()),
            ]
        },
    );

    support::task_with_output("Re-seeding rewinds the stream", || {
        let mut rng = Random::from_u64_seed(1);
        let baseline = (rng.rand(), rng.rand());
        rng.seed(1u32 ^ 0x9E37_79B9);
        let _ = rng.rand();
        rng.seed(1);
        let after = (rng.rand(), rng.rand());
        vec![
            format!("baseline      = ({}, {})", baseline.0, baseline.1),
            format!("after re-seed = ({}, {})", after.0, after.1),
        ]
    });

    support::summary(3);
}

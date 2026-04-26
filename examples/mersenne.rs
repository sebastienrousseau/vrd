// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Mersenne Twister (MT19937) backend usage.
//!
//! Reach for MT only when you need MT-specific reproducibility (e.g.,
//! matching output from another tool). For new code, the default
//! Xoshiro256++ backend is faster, smaller, and statistically stronger.
//!
//! Run: `cargo run --example mersenne`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- mersenne");

    support::task_with_output(
        "new_mersenne_twister() returns an MT-backed Random",
        || {
            let mut rng = Random::new_mersenne_twister();
            vec![
                format!("display     = {rng}"),
                format!("first u32   = {}", rng.rand()),
                format!("after seed  = mti = {}", rng.mti()),
            ]
        },
    );

    support::task_with_output(
        "Deterministic MT via new_mersenne_twister_with_seed",
        || {
            let mut a = Random::new_mersenne_twister_with_seed(12345);
            let mut b = Random::new_mersenne_twister_with_seed(12345);
            (0..4)
                .map(|i| {
                    let av = a.rand();
                    let bv = b.rand();
                    format!(
                        "draw {i}: a = {av:>10}, b = {bv:>10}, equal = {}",
                        av == bv
                    )
                })
                .collect()
        },
    );

    support::task_with_output(
        "Manual twist refreshes the state vector",
        || {
            let mut rng = Random::new_mersenne_twister_with_seed(0);
            let before = rng.mti();
            rng.twist();
            let after = rng.mti();
            vec![
                format!("mti before twist = {before}"),
                format!("mti after twist  = {after}"),
            ]
        },
    );

    support::summary(3);
}

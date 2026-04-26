// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Side-by-side comparison of the two `Random` backends.
//!
//! Run: `cargo run --example backends`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- backends");

    support::task_with_output("Same seed yields different streams per backend", || {
        let mut xs = Random::from_u64_seed(42);
        let mut mt = Random::new_mersenne_twister_with_seed(42);
        vec![
            format!("Xoshiro256++  : {}", xs.rand()),
            format!("MersenneTwister: {}", mt.rand()),
            "(streams differ — they are different algorithms)".into(),
        ]
    });

    support::task_with_output("State sizes", || {
        vec![
            "Xoshiro256++   : 32 bytes (inline)".into(),
            "MersenneTwister: 2496 bytes (heap, behind alloc feature)"
                .into(),
        ]
    });

    support::task_with_output("Backend introspection", || {
        let xs = Random::from_u64_seed(0);
        let mt = Random::new_mersenne_twister_with_seed(0);
        vec![
            format!("xs.backend() = {:?}", xs.backend()),
            format!("mt.backend() = {:?}", mt.backend()),
        ]
    });

    support::summary(3);
}

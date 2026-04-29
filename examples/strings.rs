// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Random strings of arbitrary length.
//!
//! Run: `cargo run --example strings`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- strings");

    support::task_with_output(
        "string(n) generates lowercase ASCII",
        || {
            let mut rng = Random::from_u64_seed(0xABCD);
            let s = rng.string(20);
            vec![format!("len = {}", s.len()), format!("\"{s}\"")]
        },
    );

    support::task_with_output("Length 0 is an empty string", || {
        let mut rng = Random::from_u64_seed(0);
        let s = rng.string(0);
        vec![format!("\"{s}\" (len = {})", s.len())]
    });

    support::task_with_output("Same seed -> same string", || {
        let mut a = Random::from_u64_seed(13);
        let mut b = Random::from_u64_seed(13);
        let sa = a.string(8);
        let sb = b.string(8);
        vec![
            format!("a = {sa}"),
            format!("b = {sb}"),
            format!("equal = {}", sa == sb),
        ]
    });

    support::summary(3);
}

// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tour of the most-used `Random` methods in one place.
//!
//! Run: `cargo run --example basics`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- basics");

    support::task_with_output("Integer outputs", || {
        let mut rng = Random::from_u64_seed(7);
        vec![
            format!("rand()  u32 = {}", rng.rand()),
            format!("u64()   u64 = {}", rng.u64()),
            format!("i64()   i64 = {}", rng.i64()),
        ]
    });

    support::task_with_output("Float outputs in [0.0, 1.0)", || {
        let mut rng = Random::from_u64_seed(7);
        vec![
            format!("float()  f32 = {:.10}", rng.float()),
            format!("double() f64 = {:.16}", rng.double()),
            format!("f64()    f64 = {:.16}", rng.f64()),
        ]
    });

    support::task_with_output("Bounded integer ranges", || {
        let mut rng = Random::from_u64_seed(7);
        vec![
            format!("int(-10, 10) inclusive   = {}", rng.int(-10, 10)),
            format!("uint(1, 100) inclusive   = {}", rng.uint(1, 100)),
            format!("range(1, 6) inclusive    = {}", rng.range(1, 6)),
            format!(
                "random_range(0, 1000)    = {}",
                rng.random_range(0, 1000)
            ),
            format!("bounded(42)              = {}", rng.bounded(42)),
        ]
    });

    support::task_with_output("Bools, chars, and strings", || {
        let mut rng = Random::from_u64_seed(7);
        vec![
            format!("bool(0.5)  = {}", rng.bool(0.5)),
            format!("char()     = '{}'", rng.char()),
            format!("string(8)  = {}", rng.string(8)),
        ]
    });

    support::task_with_output("Bytes via Random::bytes", || {
        let mut rng = Random::from_u64_seed(7);
        let bytes = rng.bytes(16);
        vec![format!("16 random bytes = {:02x?}", bytes)]
    });

    support::summary(5);
}

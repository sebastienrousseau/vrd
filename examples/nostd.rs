// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `no_std` usage notes.
//!
//! Even though this example is itself a `std` binary (cargo runs
//! examples against the default feature set), every API it calls is
//! `no_std`-safe: it uses only `Random::from_seed` / `Random::rand`
//! / `Random::float` / `Random::bounded` — all of which compile under
//! `--no-default-features` against `thumbv7em-none-eabihf`.
//!
//! Run: `cargo run --example nostd`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- nostd");

    support::task_with_output(
        "From a 32-byte seed, no allocator required",
        || {
            let seed = [0x42u8; 32];
            let mut rng = Random::from_seed(seed);
            vec![
                format!("rand()        = {}", rng.rand()),
                format!("bounded(100)  = {}", rng.bounded(100)),
                format!("float()       = {:.6}", rng.float()),
            ]
        },
    );

    support::task_with_output("Configuring vrd for `no_std` projects", || {
        vec![
            "[dependencies]".into(),
            "vrd = { version = \"0.0.10\", default-features = false }".into(),
            "".into(),
            "// In your lib.rs:".into(),
            "#![no_std]".into(),
            "use vrd::Random;".into(),
            "let mut rng = Random::from_seed([0u8; 32]);".into(),
        ]
    });

    support::task_with_output(
        "When you need heap APIs (bytes, sample, MT backend)",
        || {
            vec![
                "Add the `alloc` feature:".into(),
                "vrd = { version = \"0.0.10\", default-features = false, features = [\"alloc\"] }"
                    .into(),
                "".into(),
                "Then `Random::bytes`, `Random::string`,".into(),
                "`Random::sample`, and the Mersenne Twister backend".into(),
                "all become available.".into(),
            ]
        },
    );

    support::summary(3);
}

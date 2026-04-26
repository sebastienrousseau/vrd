// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `Random` implements `rand_core::TryRng` (with `Error = Infallible`),
//! so the blanket-implemented `Rng` trait works on it too.
//!
//! Run: `cargo run --example traits`

#[path = "support.rs"]
mod support;

use rand::rand_core::{Rng, TryRng};
use vrd::Random;

fn main() {
    support::header("vrd -- traits");

    support::task_with_output("Rng methods (next_u32, next_u64, fill_bytes)", || {
        let mut rng = Random::from_u64_seed(1);
        let u32_v = rng.next_u32();
        let u64_v = rng.next_u64();
        let mut buf = [0u8; 8];
        rng.fill_bytes(&mut buf);
        vec![
            format!("next_u32  = {u32_v}"),
            format!("next_u64  = {u64_v}"),
            format!("fill_bytes = {buf:02x?}"),
        ]
    });

    support::task_with_output("TryRng methods (try_next_u32, try_fill_bytes)", || {
        let mut rng = Random::from_u64_seed(1);
        let v = rng.try_next_u32().expect("Infallible");
        let mut buf = [0u8; 8];
        rng.try_fill_bytes(&mut buf).expect("Infallible");
        vec![
            format!("try_next_u32 = {v}"),
            format!("try_fill_bytes -> {buf:02x?}"),
        ]
    });

    support::task_with_output(
        "SeedableRng plugs Random into the wider rand ecosystem",
        || {
            let seed = [9u8; 32];
            let mut rng = Random::from_seed(seed);
            vec![format!(
                "from_seed(seed[..32]) -> first u32 = {}",
                rng.next_u32()
            )]
        },
    );

    support::summary(3);
}

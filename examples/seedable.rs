// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `SeedableRng::from_seed` on each generator type.
//!
//! Run: `cargo run --example seedable`

#[path = "support.rs"]
mod support;

use rand::rand_core::SeedableRng;
use vrd::random::MersenneTwister;
use vrd::xoshiro::Xoshiro256PlusPlus;
use vrd::Random;

fn main() {
    support::header("vrd -- seedable");

    support::task_with_output(
        "Random impls SeedableRng with [u8; 32]",
        || {
            let seed = [42u8; 32];
            let mut rng = <Random as SeedableRng>::from_seed(seed);
            vec![
                format!("first u32 = {}", rng.rand()),
                format!("next  u32 = {}", rng.rand()),
            ]
        },
    );

    support::task_with_output(
        "Xoshiro256PlusPlus impls SeedableRng",
        || {
            let seed = [42u8; 32];
            let mut rng =
                <Xoshiro256PlusPlus as SeedableRng>::from_seed(seed);
            vec![format!("first u64 = {}", rng.next_u64())]
        },
    );

    support::task_with_output(
        "MersenneTwister impls SeedableRng with [u8; 32]",
        || {
            let seed = [42u8; 32];
            let mut rng =
                <MersenneTwister as SeedableRng>::from_seed(seed);
            vec![format!("first u32 = {}", rng.rand())]
        },
    );

    support::summary(3);
}

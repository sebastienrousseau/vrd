// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Pick K winners from N tickets without replacement.
//!
//! Run: `cargo run --example lottery`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- lottery");

    support::task_with_output("Draw 6/49", || {
        let mut rng = Random::from_u64_seed(0x649);
        let pool: Vec<u32> = (1..=49).collect();
        let mut winners: Vec<u32> = rng
            .sample(&pool, 6)
            .into_iter()
            .copied()
            .collect();
        winners.sort_unstable();
        vec![format!("winners = {winners:?}")]
    });

    support::task_with_output("Pick 3 finalists from 100 entries", || {
        let mut rng = Random::from_u64_seed(0xF1A1);
        let entries: Vec<String> = (1..=100)
            .map(|i| format!("entrant-{i:03}"))
            .collect();
        let finalists = rng.sample(&entries, 3);
        finalists
            .iter()
            .enumerate()
            .map(|(i, name)| format!("#{i}: {name}"))
            .collect()
    });

    support::task_with_output("Same draw is reproducible from a seed", || {
        let pool: Vec<u32> = (1..=20).collect();
        let mut a = Random::from_u64_seed(7);
        let mut b = Random::from_u64_seed(7);
        let pa: Vec<u32> =
            a.sample(&pool, 5).into_iter().copied().collect();
        let pb: Vec<u32> =
            b.sample(&pool, 5).into_iter().copied().collect();
        vec![
            format!("a = {pa:?}"),
            format!("b = {pb:?}"),
            format!("equal = {}", pa == pb),
        ]
    });

    support::summary(3);
}

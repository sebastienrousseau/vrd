// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Dice-roller built on top of `Random::int`.
//!
//! Run: `cargo run --example dice`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn roll(rng: &mut Random, sides: i32) -> i32 {
    rng.int(1, sides)
}

fn roll_n(rng: &mut Random, n: usize, sides: i32) -> Vec<i32> {
    (0..n).map(|_| roll(rng, sides)).collect()
}

fn main() {
    support::header("vrd -- dice");

    support::task_with_output("Single d20", || {
        let mut rng = Random::from_u64_seed(0xD20);
        vec![format!("d20 = {}", roll(&mut rng, 20))]
    });

    support::task_with_output("4d6 — drop the lowest (D&D ability rolls)", || {
        let mut rng = Random::from_u64_seed(0xD6D6);
        (0..6)
            .map(|i| {
                let mut rolls = roll_n(&mut rng, 4, 6);
                rolls.sort_unstable();
                let total: i32 = rolls.iter().skip(1).sum();
                format!("ability {i}: {rolls:?} -> {total}")
            })
            .collect()
    });

    support::task_with_output("Histogram of 10,000 d6 rolls", || {
        let mut rng = Random::from_u64_seed(7);
        let mut bins = [0u32; 6];
        for _ in 0..10_000 {
            bins[(roll(&mut rng, 6) - 1) as usize] += 1;
        }
        bins.iter()
            .enumerate()
            .map(|(face, &c)| {
                let bar = "\u{2588}".repeat(c as usize / 50);
                format!("{}: {bar} {c}", face + 1)
            })
            .collect()
    });

    support::summary(3);
}

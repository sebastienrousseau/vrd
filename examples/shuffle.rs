// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Fisher-Yates shuffle in place.
//!
//! Run: `cargo run --example shuffle`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- shuffle");

    support::task_with_output("shuffle reorders without dropping elements", || {
        let mut rng = Random::from_u64_seed(7);
        let mut deck: Vec<u32> = (1..=10).collect();
        let original: Vec<_> = deck.to_vec();
        rng.shuffle(&mut deck);
        let mut sorted = deck.clone();
        sorted.sort_unstable();
        vec![
            format!("original = {original:?}"),
            format!("shuffled = {deck:?}"),
            format!("multiset preserved = {}", sorted == original),
        ]
    });

    support::task_with_output("Same seed -> same shuffle", || {
        let mut a = Random::from_u64_seed(42);
        let mut b = Random::from_u64_seed(42);
        let mut da: Vec<u32> = (1..=8).collect();
        let mut db: Vec<u32> = (1..=8).collect();
        a.shuffle(&mut da);
        b.shuffle(&mut db);
        vec![
            format!("a = {da:?}"),
            format!("b = {db:?}"),
            format!("equal = {}", da == db),
        ]
    });

    support::summary(2);
}

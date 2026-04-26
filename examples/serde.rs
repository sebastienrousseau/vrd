// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Serialize and deserialize `Random` (and the underlying generator
//! state) via `serde_json`. Requires the `serde` feature.
//!
//! Run: `cargo run --example serde --features serde`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- serde");

    support::task_with_output("Serialize Random, then deserialize and continue", || {
        let mut rng = Random::from_u64_seed(123);
        // Burn a few draws so the state is non-initial.
        let _ = rng.rand();
        let _ = rng.rand();

        let json = serde_json::to_string(&rng).expect("serialize");
        let mut restored: Random =
            serde_json::from_str(&json).expect("deserialize");

        // Both copies should now produce identical sequences.
        let a: Vec<u32> = (0..4).map(|_| rng.rand()).collect();
        let b: Vec<u32> = (0..4).map(|_| restored.rand()).collect();

        let mut lines = Vec::new();
        lines.push(format!("json size  = {} bytes", json.len()));
        lines.push(format!("original   = {a:?}"));
        lines.push(format!("restored   = {b:?}"));
        lines.push(format!("equal      = {}", a == b));
        lines
    });

    support::summary(1);
}

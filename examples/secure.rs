// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! ChaCha20 CSPRNG backend - `Random::new_secure()` /
//! `Random::from_secure_seed`. Use for tokens, salts, session IDs:
//! anything an attacker would benefit from predicting.
//!
//! Run: `cargo run --example secure --features crypto`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- secure (ChaCha20 CSPRNG)");

    support::task_with_output(
        "from_secure_seed - deterministic CSPRNG output",
        || {
            let mut rng = Random::from_secure_seed([42u8; 32]);
            (0..4)
                .map(|i| format!("draw {i}: {:>20}", rng.u64()))
                .collect()
        },
    );

    support::task_with_output(
        "Crypto-grade tokens / UUIDs from the same RNG",
        || {
            let mut rng = Random::from_secure_seed([0xAB; 32]);
            vec![
                format!("hex_token(16)    = {}", rng.hex_token(16)),
                format!("base64_token(24) = {}", rng.base64_token(24)),
                format!("uuid_v4          = {}", rng.uuid_v4()),
            ]
        },
    );

    support::task_with_output(
        "new_secure() seeds from the OS entropy source",
        || {
            let mut rng = Random::new_secure();
            vec![format!("first u64 = {}", rng.u64())]
        },
    );

    support::summary(3);
}

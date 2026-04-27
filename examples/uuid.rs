// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! UUID v4 generation: `uuid_v4()` (string) and `uuid_v4_bytes()`
//! (16-byte allocation-free).
//!
//! `vrd` is **not** a CSPRNG — for security-sensitive UUIDs (session
//! tokens, identity), reach for `getrandom` or `rand::rngs::OsRng`.
//! This example is for non-security identifiers (database keys,
//! correlation IDs, log markers).
//!
//! Run: `cargo run --example uuid`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn main() {
    support::header("vrd -- uuid");

    support::task_with_output(
        "uuid_v4_bytes() — 16 bytes, no_std",
        || {
            let mut rng = Random::from_u64_seed(0xCAFE);
            let b = rng.uuid_v4_bytes();
            vec![
                format!("bytes      = {b:02x?}"),
                format!("version    = {} (must be 4)", b[6] >> 4),
                format!(
                    "variant    = 0b{:02b} (must be 10)",
                    b[8] >> 6
                ),
            ]
        },
    );

    support::task_with_output(
        "uuid_v4() — RFC 4122 hyphenated",
        || {
            let mut rng = Random::from_u64_seed(0xCAFE);
            (0..6).map(|_| rng.uuid_v4()).collect()
        },
    );

    support::task_with_output("Same seed yields same UUID", || {
        let mut a = Random::from_u64_seed(99);
        let mut b = Random::from_u64_seed(99);
        let ua = a.uuid_v4();
        let ub = b.uuid_v4();
        vec![
            format!("a = {ua}"),
            format!("b = {ub}"),
            format!("equal = {}", ua == ub),
        ]
    });

    support::summary(3);
}

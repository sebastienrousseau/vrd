// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Generate passwords from arbitrary character sets.
//!
//! Note: `Random` (Xoshiro256++) is **not** a cryptographic RNG. Use
//! a CSPRNG for credentials in production. This example illustrates
//! the construction pattern only.
//!
//! Run: `cargo run --example passwords`

#[path = "support.rs"]
mod support;

use vrd::Random;

fn make(rng: &mut Random, alphabet: &[u8], len: usize) -> String {
    (0..len)
        .map(|_| {
            let idx = rng.bounded(alphabet.len() as u32) as usize;
            alphabet[idx] as char
        })
        .collect()
}

fn main() {
    support::header("vrd -- passwords");

    let lower = b"abcdefghijklmnopqrstuvwxyz";
    let upper = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = b"0123456789";
    let symbols = b"!@#$%^&*()-_=+[]{}";

    support::task_with_output("16-char alphanumeric", || {
        let mut rng = Random::from_u64_seed(0xA1B2);
        let mut alphabet = Vec::new();
        alphabet.extend_from_slice(lower);
        alphabet.extend_from_slice(upper);
        alphabet.extend_from_slice(digits);
        vec![format!("password = {}", make(&mut rng, &alphabet, 16))]
    });

    support::task_with_output("24-char with symbols", || {
        let mut rng = Random::from_u64_seed(0xA1B2);
        let mut alphabet = Vec::new();
        alphabet.extend_from_slice(lower);
        alphabet.extend_from_slice(upper);
        alphabet.extend_from_slice(digits);
        alphabet.extend_from_slice(symbols);
        vec![format!("password = {}", make(&mut rng, &alphabet, 24))]
    });

    support::task_with_output("Five 8-char tokens", || {
        let mut rng = Random::from_u64_seed(0xCAFE);
        let mut alphabet = Vec::new();
        alphabet.extend_from_slice(lower);
        alphabet.extend_from_slice(digits);
        (0..5)
            .map(|i| {
                format!("token-{i}: {}", make(&mut rng, &alphabet, 8))
            })
            .collect()
    });

    support::summary(3);
}

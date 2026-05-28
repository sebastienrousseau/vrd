// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Integration tests for the `crypto` feature: `Random::new_secure`,
//! `Random::from_secure_seed`, ChaCha20-backed UUID / token paths.

#![cfg(feature = "crypto")]

use vrd::{Random, RngBackend};

#[test]
fn from_secure_seed_uses_chacha_backend() {
    let rng = Random::from_secure_seed([0u8; 32]);
    assert!(matches!(rng.backend(), RngBackend::ChaCha20(_)));
}

#[test]
fn from_secure_seed_is_deterministic() {
    let mut a = Random::from_secure_seed([42u8; 32]);
    let mut b = Random::from_secure_seed([42u8; 32]);
    for _ in 0..16 {
        assert_eq!(a.u64(), b.u64());
    }
}

#[test]
fn distributions_dispatch_through_chacha() {
    let mut rng = Random::from_secure_seed([7u8; 32]);
    // The Ziggurat normal() sampler is backend-agnostic — confirm
    // it still produces finite samples on ChaCha20.
    for _ in 0..1024 {
        assert!(rng.normal(0.0, 1.0).is_finite());
    }
}

#[test]
#[cfg(feature = "alloc")]
fn uuid_v4_via_secure_backend() {
    let mut rng = Random::from_secure_seed([1u8; 32]);
    let id = rng.uuid_v4();
    // Canonical 8-4-4-4-12 layout.
    assert_eq!(id.len(), 36);
    assert_eq!(id.as_bytes()[8], b'-');
    assert_eq!(id.as_bytes()[13], b'-');
    assert_eq!(id.as_bytes()[18], b'-');
    assert_eq!(id.as_bytes()[23], b'-');
    // Version 4 marker.
    assert_eq!(id.as_bytes()[14], b'4');
}

#[test]
fn fill_bytes_unaligned() {
    let mut rng = Random::from_secure_seed([9u8; 32]);
    let mut buf = [0u8; 99];
    use rand::rand_core::TryRng;
    rng.try_fill_bytes(&mut buf).unwrap();
    assert!(buf.iter().any(|&b| b != 0));
}

#[test]
fn split_returns_none_for_chacha() {
    let mut rng = Random::from_secure_seed([1u8; 32]);
    assert!(rng.split().is_none());
}

#[test]
#[cfg(feature = "std")]
fn new_secure_produces_different_streams() {
    let mut a = Random::new_secure();
    let mut b = Random::new_secure();
    // Two OS-seeded instances should differ on the very first draw
    // with overwhelming probability.
    assert_ne!(a.u64(), b.u64());
}

// Copyright © 2023 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate vrd;
use vrd::Random;
use vrd::*;

#[cfg(test)]
#[test]
fn test_random_range_macro() {
    let mut rng = Random::new();
    let min = 10;
    let max = 20;
    let num = random_range!(rng, min, max);
    assert!(num >= min && num < max);
}

#[test]
fn test_rand_bool_macro() {
    let mut rng = Random::new();
    let p = 1.0; // Set p to 1.0 to always generate true
    let b = rand_bool!(rng, p);
    assert!(b);
}

#[test]
fn test_rand_bytes_macro() {
    let mut rng = Random::new();
    let len = 10;
    let bytes = rand_bytes!(rng, len);
    assert_eq!(bytes.len(), len);
}

#[test]
fn test_rand_char_macro() {
    let mut rng = Random::new();
    let c = rand_char!(rng);
    assert!(
        c.is_ascii_lowercase()
            || c.is_ascii_uppercase()
            || c.is_ascii_digit()
    );
}

#[test]
fn test_rand_choose_macro() {
    let mut rng = Random::new();
    let values = vec![1, 2, 3, 4, 5];
    let chosen = rand_choose!(rng, &values).unwrap();
    assert!(values.contains(chosen));
}

#[test]
fn test_rand_float_macro() {
    let mut rng = Random::new();
    let f = rand_float!(rng);
    assert!((0.0..1.0).contains(&f));
}

#[test]
fn test_rand_int_macro() {
    let mut rng = Random::new();
    let min = 10;
    let max = 20;
    let num = rand_int!(rng, min, max);
    assert!(num >= min && num <= max);
}

#[test]
fn test_rand_pseudo_macro() {
    let mut rng = Random::new();
    let p = rand_pseudo!(rng);
    assert!(p < 4294967295);
}

#[test]
fn test_rand_range_macro() {
    let mut rng = Random::new();
    let min = 10;
    let max = 20;
    let num = rand_range!(rng, min, max);
    assert!(num >= min && num <= max);
}

#[test]
fn test_rand_seed_macro() {
    let mut rng = Random::new();
    rand_seed!(rng, 42);
    let num = rng.rand();
    assert!(num < 4294967295);
}

#[test]
fn test_rand_twist_macro() {
    let mut rng = Random::new();
    rand_twist!(rng);
    let num = rng.rand();
    assert!(num < 4294967295);
}

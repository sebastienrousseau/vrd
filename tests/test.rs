// Copyright © 2023 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

extern crate vrd;
use vrd::Random;
use vrd::*;

#[cfg(test)]
const N: usize = 624;

#[test]
fn test_bool() {
    let mut rng = Random::new();
    let mut count_of_true = 0;
    for _ in 0..1000 {
        let b = Random::bool(&mut rng, 0.5);
        if b {
            count_of_true += 1;
        }
    }
    assert!((count_of_true as f64 / 1000.0 - 0.5).abs() <= 0.05);
}
#[test]
fn test_bytes() {
    let mut rng = Random::new();
    let bytes = Random::bytes(&mut rng, 0);
    assert_eq!(bytes.len(), 0);

    let bytes = Random::bytes(&mut rng, 10);
    assert_eq!(bytes.len(), 10);

    let bytes = Random::bytes(&mut rng, 100);
    assert_eq!(bytes.len(), 100);
}
#[test]
fn test_char() {
    let mut rng = Random::new();
    let c = Random::char(&mut rng);
    assert!(('a'..='z').contains(&c));
}
#[test]
fn test_choose() {
    let mut rng = Random::new();
    let values = vec![1, 2, 3, 4, 5];
    let value = Random::choose(&mut rng, &values);
    assert!(value.is_some());
    assert!(value.unwrap() >= &1 && value.unwrap() <= &5);
    if values.is_empty() {
        assert!(Random::choose(&mut rng, &values).is_none());
    }
}
#[test]
fn test_float() {
    let mut rng = Random::new();
    let f = Random::float(&mut rng);
    assert!((0.0..=1.0).contains(&f));
}
#[test]
fn test_int() {
    let mut rng = Random::new();
    let i = rng.int(0, 10);
    assert!((0..=10).contains(&i));
}
#[test]
fn test_uint() {
    let mut rng = Random::new();
    let u = rng.uint(0, 10);
    assert!((0..=10).contains(&u));
}
#[test]
fn test_double() {
    let mut rng = Random::new();
    let d = rng.double();
    assert!((0.0..=1.0).contains(&d));
}
#[test]
fn test_pseudo() {
    let mut rng = Random::new();
    let p = Random::pseudo(&mut rng);
    assert!(p < 4294967295);
}
#[test]
fn test_range() {
    let mut rng = Random::new();
    let r = Random::range(&mut rng, 0, 10);
    assert!((0..=10).contains(&r));
}
#[test]
pub fn test_new() {
    let rng = Random::new();
    assert!(rng.mti <= N);
    assert!(rng.mt[0] > 0);
}
#[test]
fn test_rand() {
    let mut rng = Random::new();
    let r = Random::rand(&mut rng);
    assert!(r < 4294967295);
}
#[test]
fn test_random_range() {
    let mut rng = Random::new();
    let r = Random::random_range(&mut rng, 0, 10);
    assert!(r <= 10);
}
#[test]
fn test_seed() {
    let mut rng = Random::new();
    Random::seed(&mut rng, 0);
    assert!(rng.mti <= N);
    assert!(rng.mt.iter().any(|&x| x != 0));
    assert!(rng.mti == N);
}
#[test]
fn test_twist() {
    let mut rng = Random::new();
    Random::seed(&mut rng, 0);
    Random::twist(&mut rng);
    assert!(rng.mti <= N);
    assert!(rng.mt.iter().any(|&x| x != 0));
}
#[test]
fn test_fmt() {
    let rng = Random::new();
    let s = format!("{rng}");
    assert!(!s.is_empty());
}
#[test]
fn test_default() {
    let rng = Random::default();
    assert!(rng.mti <= N);
    assert!(rng.mt[0] > 0);
}
#[test]
fn test_vrd() {
    let mut vrd = Random::new();
    assert!(vrd.rand() < 4294967295);
}

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
    let p = 0.5;
    let b = rand_bool!(rng, p);
    assert!(b == true || b == false);
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
    assert!((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9'));
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
    assert!(f >= 0.0 && f < 1.0);
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

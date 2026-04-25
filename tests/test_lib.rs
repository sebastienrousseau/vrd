// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! Integration tests for the public `vrd` library surface.

#[cfg(test)]
mod tests {

    use vrd::Random;

    const N: usize = 624;

    #[test]
    fn test_bool() {
        let mut rng = Random::new();
        let mut count_of_true = 0;
        for _ in 0..1000 {
            let b = rng.bool(0.5);
            if b {
                count_of_true += 1;
            }
        }
        assert!((count_of_true as f64 / 1000.0 - 0.5).abs() <= 0.1);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bytes() {
        let mut rng = Random::new();
        let bytes = rng.bytes(0);
        assert_eq!(bytes.len(), 0);

        let bytes = rng.bytes(10);
        assert_eq!(bytes.len(), 10);

        let bytes = rng.bytes(100);
        assert_eq!(bytes.len(), 100);
    }

    #[test]
    fn test_char() {
        let mut rng = Random::new();
        let c = rng.char();
        assert!(c.is_ascii_lowercase());
    }

    #[test]
    fn test_choose() {
        let mut rng = Random::new();
        let values = [1, 2, 3, 4, 5];
        let value = rng.choose(&values);
        assert!(value.is_some());
        assert!(value.unwrap() >= &1 && value.unwrap() <= &5);

        let empty_values: [i32; 0] = [];
        assert!(rng.choose(&empty_values).is_none());
    }

    #[test]
    fn test_float() {
        let mut rng = Random::new();
        let f = rng.float();
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
    fn test_mti() {
        let mut rng = Random::new_mersenne_twister();
        rng.seed(0);
        assert_eq!(rng.mti(), N);
    }

    #[test]
    fn test_range() {
        let mut rng = Random::new();
        let r = rng.range(0, 10);
        assert!((0..=10).contains(&r));
    }

    #[test]
    fn test_new() {
        let mut rng = Random::new();
        assert_ne!(rng.rand(), rng.rand());
    }

    #[test]
    fn test_rand() {
        let mut rng = Random::new();
        let r = rng.rand();
        assert!(r < 4294967295);
    }

    #[test]
    fn test_random_range() {
        let mut rng = Random::new();
        let r = rng.random_range(0, 10);
        assert!(r < 10);
    }

    #[test]
    fn test_seed() {
        let mut rng = Random::new();
        rng.seed(123);
        let v1 = rng.rand();
        rng.seed(123);
        let v2 = rng.rand();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_twist() {
        let mut rng = Random::new_mersenne_twister();
        rng.seed(0);
        rng.twist();
        assert!(rng.mti() <= N);
    }

    #[test]
    fn test_fmt() {
        let rng = Random::new();
        let s = format!("{rng}");
        assert!(!s.is_empty());
    }

    #[test]
    fn test_default() {
        let mut rng = Random::default();
        assert_ne!(rng.rand(), rng.rand());
    }

    #[test]
    fn test_i64() {
        let mut rng = Random::new();
        let i = rng.i64();
        assert!((i64::MIN..=i64::MAX).contains(&i));
    }

    #[test]
    fn test_u64() {
        let mut rng = Random::new();
        let u = rng.u64();
        assert!((u64::MIN..=u64::MAX).contains(&u));
    }

    #[test]
    fn test_f64() {
        let mut rng = Random::new();
        let f = rng.f64();
        assert!((0.0..=1.0).contains(&f));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_string() {
        let mut rng = Random::new();
        let s1 = rng.string(0);
        assert_eq!(s1.len(), 0);

        let s2 = rng.string(10);
        assert_eq!(s2.len(), 10);
    }

    #[test]
    fn test_normal() {
        let mut rng = Random::new();
        let mu = 0.0;
        let sigma = 1.0;
        let sample = rng.normal(mu, sigma);
        assert!(sample.is_finite());
    }

    #[test]
    fn test_exponential() {
        let mut rng = Random::new();
        let rate = 1.5;
        let sample = rng.exponential(rate);
        assert!(sample >= 0.0);
    }

    #[test]
    fn test_poisson() {
        let mut rng = Random::new();
        let mean = 3.0;
        let _sample = rng.poisson(mean);
    }
}

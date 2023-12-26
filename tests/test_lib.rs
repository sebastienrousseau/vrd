// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {

    extern crate vrd;
    use vrd::Random;
    use std::convert::TryInto;

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
        assert!(c.is_ascii_lowercase());
    }
    #[test]
    fn test_choose() {
        let mut rng = Random::new();
        let values = vec![1, 2, 3, 4, 5];
        let value = Random::choose(&mut rng, &values);
        assert!(value.is_some());
        assert!(value.unwrap() >= &1 && value.unwrap() <= &5);

        let empty_values: Vec<i32> = vec![];
        assert!(Random::choose(&mut rng, &empty_values).is_none());
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
    fn test_mti() {
        let mut rng = Random::new();

        // Call the seed method to set the mti field to N
        rng.seed(0);

        // Verify that mti() returns N
        assert_eq!(rng.mti(), N);
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
        rng.mti = N + 1;
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

        rng.set_mti(0); // Change `mti` to 0
        assert_eq!(rng.mti, 0); // Assert `mti` is indeed 0

        rng.seed(0); // Call the seed method to set the mti field

        // Verify that `mti` is set to `N` after seeding
        assert_eq!(rng.mti, N);
        assert!(rng.mt.iter().any(|&x| x != N.try_into().unwrap()));
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
}

// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    use vrd::random::Random;

    // Initialization tests
    #[test]
    fn test_new() {
        let rng = Random::new();
        // Check that the internal state (mt, mti) is initialized correctly.
        // You might want to compare a few elements of rng.mt against expected values
        // after its generation, based on the algorithm's specifications.
        assert_eq!(rng.mti, 624);
    }

    #[test]
    fn test_seed() {
        let mut rng = Random::new();

        rng.seed(42); // Seed with a specific value

        // Test that subsequent random numbers are deterministic based on the seed
        assert_eq!(rng.rand(), 848288234);
    }

    // Integer tests
    #[test]
    fn test_int() {
        let mut rng = Random::new();
        rng.seed(20);

        assert_eq!(rng.int(1, 10), 5);
        assert_eq!(rng.int(5, 10), 9);
        assert_eq!(rng.int(10, 20), 18);
        assert_eq!(rng.int(15, 20), 19);
        assert_eq!(rng.int(20, 30), 28);
        assert_eq!(rng.int(25, 30), 29);
        assert_eq!(rng.int(30, 40), 38);
    }

    #[test]
    fn test_int_edge_cases() {
        let mut rng = Random::new();
        rng.seed(42);

        // Test with minimum and maximum possible integers
        assert_eq!(rng.int(i32::MIN, i32::MIN + 1), i32::MIN);
        assert_eq!(rng.int(i32::MAX - 1, i32::MAX), i32::MAX - 1);
    }

    // Floating-point tests
    #[test]
    fn test_float() {
        let mut rng = Random::new();
        rng.seed(42);

        // Test generating floating-point numbers within the range [0.0, 1.0)
        let result = rng.float();
        assert!((0.0..1.0).contains(&result));

        // Test generating floating-point numbers within the range [-1.0, 0.0)
        let result = rng.float() * -1.0; // Adjust the sign to check the negative range
        assert!((-1.0..0.0).contains(&result));
    }

    #[test]
    fn test_double() {
        let mut rng = Random::new();
        rng.seed(42);

        let result = rng.double();
        assert!((0.0..1.0).contains(&result));
    }

    #[test]
    fn test_f64() {
        let mut rng = Random::new();
        rng.seed(50);

        let result = rng.f64();
        assert!((0.0..1.0).contains(&result));
    }

    // Other types tests
    #[test]
    fn test_bytes() {
        let mut rng = Random::new();
        rng.seed(5); // Fixed seed for predictability

        let expected_bytes = vec![234, 232, 232, 232, 232, 232, 232];
        let random_bytes = rng.bytes(expected_bytes.len());

        assert_eq!(random_bytes, expected_bytes);
    }

    #[test]
    fn test_bool() {
        let mut rng = Random::new();
        rng.seed(42);

        // With a probability of 0.5, true and false should have roughly equal occurrence
        let mut true_count: i32 = 0;
        let mut false_count: i32 = 0;

        for _ in 0..1000 {
            if rng.bool(0.5) {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }

        // Assert that the counts are approximately balanced (adjust tolerance as needed)
        let difference = (true_count - false_count).abs();
        assert!(difference < 100);
    }

    #[test]
    fn test_char() {
        let mut rng = Random::new();
        rng.seed(60);

        let result = rng.char();
        assert!(result.is_ascii_lowercase());
    }

    // Method tests
    #[test]
    #[should_panic(
        expected = "max must be greater than min for random_range"
    )]
    fn test_random_range_invalid() {
        let mut rng = Random::new();
        rng.random_range(20, 10); // Should panic
    }

    #[test]
    fn test_random_range() {
        let mut rng = Random::new();
        rng.seed(40);

        assert_ne!(rng.random_range(1, 10), 0);
    }

    #[test]
    fn test_clone() {
        let mut rng = Random::new();
        rng.seed(42);

        let mut cloned_rng = rng.clone();

        // Ensure that the cloned RNG produces the same sequence of random numbers
        for _ in 0..100 {
            assert_eq!(rng.rand(), cloned_rng.rand());
        }
    }

    #[test]
    fn test_shuffle() {
        let mut rng = Random::new();
        rng.seed(42);

        let mut data = vec![1, 2, 3, 4, 5];
        let original_data = data.clone();

        rng.shuffle(&mut data);

        // Ensure that the shuffle operation produces a different permutation
        assert_ne!(data, original_data);

        // Ensure that all elements are still present in the shuffled vector
        original_data.iter().for_each(|x| assert!(data.contains(x)));
        data.iter().for_each(|x| assert!(original_data.contains(x)));
    }

    #[test]
    fn test_choose() {
        let mut rng = Random::new();
        rng.seed(42);

        let data = vec![1, 2, 3, 4, 5];

        // Choose a random element from the data vector
        let chosen_element = rng.choose(&data).unwrap();

        // Ensure that the chosen element is present in the original data
        assert!(data.contains(chosen_element));
    }
}

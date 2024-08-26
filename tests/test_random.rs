// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    use rand::RngCore;
    use vrd::random::Random;

    // Initialization tests
    /// Tests the `new` method to ensure that the RNG is initialized correctly.
    #[test]
    fn test_new() {
        let rng = Random::new();
        assert_eq!(rng.mti, 624);
    }

    /// Tests the `seed` method to ensure that seeding produces consistent random numbers.
    #[test]
    fn test_seed() {
        let mut rng = Random::new();
        rng.seed(42);
        assert_eq!(rng.rand(), 848288234);
    }

    // Integer generation tests
    /// Tests the `int` method to ensure it generates integers within the specified range.
    #[test]
    fn test_int() {
        let mut rng = Random::new();
        rng.seed(20);
        assert_eq!(rng.int(1, 10), 5);
    }

    /// Tests edge cases for the `int` method with minimum and maximum integer values.
    #[test]
    fn test_int_edge_cases() {
        let mut rng = Random::new();
        rng.seed(42);
        assert_eq!(rng.int(i32::MIN, i32::MIN + 1), i32::MIN);
        assert_eq!(rng.int(i32::MAX - 1, i32::MAX), i32::MAX - 1);
    }

    /// Tests the `int` method to ensure it handles cases where min and max are equal.
    #[test]
    fn test_int_min_max_equal() {
        let mut rng = Random::new();
        assert_eq!(rng.int(5, 5), 5);
    }

    /// Tests the `int` method to ensure it panics when min is greater than max.
    #[test]
    #[should_panic(
        expected = "min must be less than or equal to max for int"
    )]
    fn test_int_min_greater_than_max() {
        let mut rng = Random::new();
        rng.int(10, 5);
    }

    /// Tests the `uint` method to ensure it handles cases where min and max are equal.
    #[test]
    fn test_uint_min_max_equal() {
        let mut rng = Random::new();
        assert_eq!(rng.uint(5, 5), 5);
    }

    // Floating-point generation tests
    /// Tests the `float` method to ensure it generates floating-point numbers within the correct range.
    #[test]
    fn test_float() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.float();
        assert!((0.0..1.0).contains(&result));
    }

    /// Tests edge cases for the `float` method to ensure it generates small floats correctly.
    #[test]
    fn test_float_edge_cases() {
        let mut rng = Random::new();
        rng.seed(42);

        for _ in 0..1000 {
            let result = rng.float();
            assert!((0.0..1.0).contains(&result));
            assert!(result.is_finite());
        }
    }

    /// Tests the `double` method to ensure it generates double-precision floating-point numbers within the correct range.
    #[test]
    fn test_double() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.double();
        assert!((0.0..1.0).contains(&result));
    }

    /// Tests edge cases for the `double` method to ensure it generates small doubles correctly.
    #[test]
    fn test_double_edge_cases() {
        let mut rng = Random::new();
        rng.seed(42);

        for _ in 0..1000 {
            let result = rng.double();
            assert!((0.0..1.0).contains(&result));
            assert!(result.is_finite());
        }
    }

    /// Tests the `f64` method to ensure it generates 64-bit floating-point numbers within the correct range.
    #[test]
    fn test_f64() {
        let mut rng = Random::new();
        rng.seed(50);
        let result = rng.f64();
        assert!((0.0..1.0).contains(&result));
    }

    // Byte and character generation tests
    /// Tests the `bytes` method to ensure it generates the correct sequence of bytes.
    #[test]
    fn test_bytes() {
        let mut rng = Random::new();
        rng.seed(5);
        let expected_bytes = vec![234, 232, 232, 232, 232, 232, 232];
        let random_bytes = rng.bytes(expected_bytes.len());
        assert_eq!(random_bytes, expected_bytes);
    }

    /// Tests the `bool` method to ensure it generates booleans with the correct probability distribution.
    #[test]
    fn test_bool() {
        let mut rng = Random::new();
        rng.seed(42);

        let mut true_count: i32 = 0;
        let mut false_count: i32 = 0;

        for _ in 0..10_000 {
            if rng.bool(0.5) {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }

        let difference = (true_count - false_count).abs();
        assert!(difference < 500);
    }

    /// Tests the `char` method to ensure it generates lowercase characters.
    #[test]
    fn test_char() {
        let mut rng = Random::new();
        rng.seed(60);
        let result = rng.char();
        assert!(result.is_ascii_lowercase());
    }

    // String generation tests
    /// Tests the `string` method to ensure it generates a string of the specified length.
    #[test]
    fn test_string() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.string(10);
        assert_eq!(result.len(), 10);
        assert!(result.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    /// Tests the `string` method to ensure it handles zero length input correctly.
    #[test]
    fn test_string_zero_length() {
        let mut rng = Random::new();
        assert_eq!(rng.string(0), "");
    }

    // Random range tests
    /// Tests the `random_range` method to ensure it generates numbers within the specified range.
    #[test]
    fn test_random_range() {
        let mut rng = Random::new();
        rng.seed(40);
        assert_ne!(rng.random_range(1, 10), 0);
    }

    /// Tests the `random_range` method to ensure it panics when given invalid input.
    #[test]
    #[should_panic(
        expected = "max must be greater than min for random_range"
    )]
    fn test_random_range_invalid() {
        let mut rng = Random::new();
        rng.random_range(20, 10);
    }

    /// Tests the `random_range` method to ensure it panics when min equals max.
    #[test]
    #[should_panic(
        expected = "max must be greater than min for random_range"
    )]
    fn test_random_range_min_equal_max() {
        let mut rng = Random::new();
        rng.random_range(10, 10);
    }

    // RNG state tests
    /// Tests the `mti` method to ensure it returns the correct internal index.
    #[test]
    fn test_mti() {
        let rng = Random::new();
        assert_eq!(rng.mti(), 624);
    }

    /// Tests the `set_mti` method to ensure it sets the internal index correctly.
    #[test]
    fn test_set_mti() {
        let mut rng = Random::new();
        rng.set_mti(100);
        assert_eq!(rng.mti(), 100);
    }

    /// Tests the `twist` method directly to ensure it updates the internal state as expected.
    #[test]
    fn test_twist_directly() {
        let mut rng = Random::new();
        rng.seed(42);

        let mti_before = rng.mti();
        rng.twist();
        let mti_after = rng.mti();

        assert!(mti_after < mti_before);
    }

    // Cloning tests
    /// Tests the `clone` method to ensure that cloned RNGs produce the same sequence of numbers.
    #[test]
    fn test_clone() {
        let mut rng = Random::new();
        rng.seed(42);
        let mut cloned_rng = rng.clone();
        for _ in 0..100 {
            assert_eq!(rng.rand(), cloned_rng.rand());
        }
    }

    /// Tests the `clone` method after performing operations to ensure that cloned RNGs continue the same sequence.
    #[test]
    fn test_clone_after_operations() {
        let mut rng = Random::new();
        rng.seed(42);

        // Perform some operations
        rng.int(1, 10);
        rng.float();
        rng.double();

        // Clone after operations
        let mut cloned_rng = rng.clone();

        // Ensure that the cloned RNG continues the same sequence
        assert_eq!(rng.rand(), cloned_rng.rand());
        assert_eq!(rng.int(1, 100), cloned_rng.int(1, 100));
    }

    // Random selection and sampling tests
    /// Tests the `choose` method to ensure it correctly selects an element from a slice.
    #[test]
    fn test_choose() {
        let mut rng = Random::new();
        rng.seed(42);
        let data = vec![1, 2, 3, 4, 5];
        let chosen_element = rng.choose(&data).unwrap();
        assert!(data.contains(chosen_element));
    }

    /// Tests the `choose` method with an empty slice to ensure it returns `None`.
    #[test]
    fn test_choose_empty_slice() {
        let mut rng = Random::new();
        let empty_slice: &[i32] = &[];
        assert!(rng.choose(empty_slice).is_none());
    }

    /// Tests the `shuffle` method to ensure it shuffles a slice correctly.
    #[test]
    fn test_shuffle() {
        let mut rng = Random::new();
        rng.seed(42);
        let mut data = vec![1, 2, 3, 4, 5];
        let original_data = data.clone();
        rng.shuffle(&mut data);
        assert_ne!(data, original_data);
        original_data.iter().for_each(|x| assert!(data.contains(x)));
    }

    /// Tests the `rand_slice` method to ensure it generates a subslice of the specified length.
    #[test]
    fn test_rand_slice() {
        let mut rng = Random::new();
        rng.seed(42);
        let slice = &[1, 2, 3, 4, 5];
        let result = rng.rand_slice(slice, 3);
        assert!(result.is_ok());
        let subslice = result.unwrap();
        assert_eq!(subslice.len(), 3);
        assert!(subslice.iter().all(|&x| slice.contains(&x)));
    }

    /// Tests the `rand_slice` method with an empty slice to ensure it returns an error.
    #[test]
    fn test_rand_slice_empty() {
        let mut rng = Random::new();
        let empty_slice: &[i32] = &[];
        let result = rng.rand_slice(empty_slice, 1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input slice is empty");
    }

    /// Tests the `rand_slice` method with a zero length to ensure it returns an error.
    #[test]
    fn test_rand_slice_zero_length() {
        let mut rng = Random::new();
        let slice = &[1, 2, 3];
        let result = rng.rand_slice(slice, 0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Requested length must be greater than zero"
        );
    }

    /// Tests the `rand_slice` method with a length that exceeds the slice length to ensure it returns an error.
    #[test]
    fn test_rand_slice_length_exceeds() {
        let mut rng = Random::new();
        let slice = &[1, 2, 3];
        let result = rng.rand_slice(slice, 4);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Requested length exceeds slice length"
        );
    }

    /// Tests the `rand_slice` method with a length equal to the slice length to ensure it returns the full slice.
    #[test]
    fn test_rand_slice_full_length() {
        let mut rng = Random::new();
        let slice = &[1, 2, 3];
        let result = rng.rand_slice(slice, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), slice);
    }

    /// Tests the `sample` method to ensure it samples elements without replacement correctly.
    #[test]
    fn test_sample() {
        let mut rng = Random::new();
        rng.seed(42);
        let slice = &[1, 2, 3, 4, 5];
        let samples = rng.sample(slice, 3);
        assert_eq!(samples.len(), 3);
        samples.iter().for_each(|&s| assert!(slice.contains(s)));
    }

    /// Tests the `sample_with_replacement` method to ensure it samples elements with replacement correctly.
    #[test]
    fn test_sample_with_replacement() {
        let mut rng = Random::new();
        rng.seed(42);
        let slice = &[1, 2, 3, 4, 5];
        let samples = rng.sample_with_replacement(slice, 3);
        assert_eq!(samples.len(), 3);
        samples.iter().for_each(|&s| assert!(slice.contains(s)));
    }

    // Special distribution tests
    /// Tests the `pseudo` method to ensure it generates a pseudo-random number.
    #[test]
    fn test_pseudo() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.pseudo();
        assert_ne!(result, 0);
    }

    /// Tests the `normal` method to ensure it generates numbers from a normal distribution.
    #[test]
    fn test_normal() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.normal(0.0, 1.0);
        assert!(result.is_finite());
    }

    /// Tests the `exponential` method to ensure it generates numbers from an exponential distribution.
    #[test]
    fn test_exponential() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.exponential(1.5);
        assert!(result >= 0.0);
    }

    /// Tests the `exponential` method to ensure it handles a zero rate correctly.
    #[test]
    fn test_exponential_zero_rate() {
        let mut rng = Random::new();
        let result = rng.exponential(0.0);
        assert!(result.is_infinite() && result.is_sign_positive());
    }

    /// Tests the `poisson` method to ensure it generates numbers from a Poisson distribution.
    #[test]
    fn test_poisson() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.poisson(3.0);

        // Ensure that the result is within a reasonable range given the mean
        // For a mean of 3.0, values are likely to be between 0 and some reasonable upper bound.
        assert!(result < 20);
    }

    /// Tests the `poisson` method to ensure it handles a zero mean correctly.
    #[test]
    fn test_poisson_zero_mean() {
        let mut rng = Random::new();
        assert_eq!(rng.poisson(0.0), 0);
    }

    // Buffer fill and display tests
    /// Tests the `fill` method to ensure it fills a buffer with non-zero values.
    #[test]
    fn test_fill() {
        let mut rng = Random::new();
        let mut buffer = [0u32; 10];
        rng.fill(&mut buffer);
        assert!(buffer.iter().any(|&x| x != 0));
    }

    /// Tests the `Display` implementation for the `Random` struct to ensure it formats correctly.
    #[test]
    fn test_display() {
        let rng = Random::new();
        let display = format!("{}", rng);
        assert!(display.contains("mt"));
        assert!(display.contains("mti"));
    }

    // RngCore trait implementation tests
    /// Tests the `next_u32` method from `RngCore` to ensure it generates non-zero random `u32` values.
    #[test]
    fn test_next_u32() {
        let mut rng = Random::new();
        let value = rng.next_u32();
        assert!(value != 0);
    }

    /// Tests the `next_u64` method from `RngCore` to ensure it generates non-zero random `u64` values.
    #[test]
    fn test_next_u64() {
        let mut rng = Random::new();
        let value = rng.next_u64();
        assert!(value != 0);
    }

    /// Tests the `fill_bytes` method from `RngCore` to ensure it fills a byte slice with random data.
    #[test]
    fn test_fill_bytes() {
        let mut rng = Random::new();
        let mut buffer = [0u8; 8];
        rng.fill_bytes(&mut buffer);
        assert!(buffer.iter().any(|&x| x != 0));
    }

    /// Tests the `try_fill_bytes` method from `RngCore` to ensure it fills a byte slice and returns `Ok(())`.
    #[test]
    fn test_try_fill_bytes() {
        let mut rng = Random::new();
        let mut buffer = [0u8; 8];
        let result = rng.try_fill_bytes(&mut buffer);
        assert!(result.is_ok());
        assert!(buffer.iter().any(|&x| x != 0));
    }

    // Clone trait test
    /// Tests that the `Clone` trait creates an exact copy of the `Random` struct.
    #[test]
    fn test_clone_trait() {
        let rng1 = Random::new();
        let rng2 = rng1.clone();
        assert_eq!(rng1.mt, rng2.mt);
        assert_eq!(rng1.mti, rng2.mti);
    }

    // Debug trait test
    /// Tests that the `Debug` trait formats the `Random` struct correctly.
    #[test]
    fn test_debug_trait() {
        let rng = Random::new();
        let debug_str = format!("{:?}", rng);
        assert!(debug_str.contains("Random"));
        assert!(debug_str.contains("mt"));
        assert!(debug_str.contains("mti"));
    }

    // Eq and PartialEq trait test
    /// Tests the `Eq` and `PartialEq` traits for equality between two `Random` structs.
    #[test]
    fn test_eq_partial_eq_trait() {
        let mut rng1 = Random::new();
        let mut rng2 = Random::new();

        // Ensure they are not equal initially due to random initialization
        assert_ne!(rng1, rng2);

        // Seed both RNGs identically
        rng1.seed(42);
        rng2.seed(42);

        // Now they should be equal
        assert_eq!(rng1, rng2);

        // Modify one RNG and check inequality
        rng2.mti = 500;
        assert_ne!(rng1, rng2);
    }

    // Hash trait test
    /// Tests that the `Hash` trait produces consistent hash values for the `Random` struct.
    #[test]
    fn test_hash_trait() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let rng1 = Random::new();
        let rng2 = rng1.clone();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        rng1.hash(&mut hasher1);
        rng2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    // Ord and PartialOrd trait test
    /// Tests the `Ord` and `PartialOrd` traits for ordering between two `Random` structs.
    #[test]
    fn test_ord_partial_ord_trait() {
        let mut rng1 = Random::new();
        let mut rng2 = Random::new();

        // Seed both RNGs identically
        rng1.seed(42);
        rng2.seed(42);

        // Initially, they should be equal
        assert_eq!(rng1.cmp(&rng2), std::cmp::Ordering::Equal);

        // Modify rng2's mti to make rng1 less than rng2
        rng2.mti = rng1.mti + 100;
        assert!(rng1 < rng2);

        // Modify rng1's mti to make rng1 greater than rng2
        rng1.mti = rng2.mti + 100;
        assert!(rng1 > rng2);
    }

    // Serialize and Deserialize trait test
    /// Tests that the `Serialize` trait serializes the `Random` struct correctly.
    #[test]
    fn test_serialize_trait() {
        let rng = Random::new();
        let serialized =
            serde_json::to_string(&rng).expect("Serialization failed");
        assert!(serialized.contains("mt"));
        assert!(serialized.contains("mti"));
    }

    /// Tests that the `Deserialize` trait deserializes the `Random` struct correctly.
    #[test]
    fn test_deserialize_trait() {
        let rng = Random::new();
        let serialized =
            serde_json::to_string(&rng).expect("Serialization failed");
        let deserialized: Random = serde_json::from_str(&serialized)
            .expect("Deserialization failed");
        assert_eq!(rng, deserialized);
    }
}

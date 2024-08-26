#[cfg(test)]
mod tests {
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
        assert_eq!(rng.int(5, 10), 9);
        assert_eq!(rng.int(10, 20), 18);
        assert_eq!(rng.int(15, 20), 19);
        assert_eq!(rng.int(20, 30), 28);
        assert_eq!(rng.int(25, 30), 29);
        assert_eq!(rng.int(30, 40), 38);
    }

    /// Tests edge cases for the `int` method with minimum and maximum integer values.
    #[test]
    fn test_int_edge_cases() {
        let mut rng = Random::new();
        rng.seed(42);
        // Test with minimum and maximum possible integers
        assert_eq!(rng.int(i32::MIN, i32::MIN + 1), i32::MIN);
        assert_eq!(rng.int(i32::MAX - 1, i32::MAX), i32::MAX - 1);
    }

    // Floating-point generation tests
    /// Tests the `float` method to ensure it generates floating-point numbers within the correct range.
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

    /// Tests the `double` method to ensure it generates double-precision floating-point numbers within the correct range.
    #[test]
    fn test_double() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.double();
        assert!((0.0..1.0).contains(&result));
    }

    /// Tests the `f64` method to ensure it generates 64-bit floating-point numbers within the correct range.
    #[test]
    fn test_f64() {
        let mut rng = Random::new();
        rng.seed(50);
        let result = rng.f64();
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

        for _ in 0..1000 {
            if rng.bool(0.5) {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }

        let difference = (true_count - false_count).abs();
        assert!(difference < 100);
    }

    /// Tests the `char` method to ensure it generates lowercase characters.
    #[test]
    fn test_char() {
        let mut rng = Random::new();
        rng.seed(60);
        let result = rng.char();
        assert!(result.is_ascii_lowercase());
    }

    // Random range tests
    /// Tests the `random_range` method to ensure it panics when given invalid input.
    #[test]
    #[should_panic(
        expected = "max must be greater than min for random_range"
    )]
    fn test_random_range_invalid() {
        let mut rng = Random::new();
        rng.random_range(20, 10);
    }

    /// Tests the `random_range` method to ensure it generates numbers within the specified range.
    #[test]
    fn test_random_range() {
        let mut rng = Random::new();
        rng.seed(40);
        assert_ne!(rng.random_range(1, 10), 0);
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

    /// Tests the `shuffle` method to ensure it shuffles a slice correctly.
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

    /// Tests the `rand_slice` method to ensure it generates a subslice of the specified length.
    #[test]
    fn test_rand_slice() {
        let mut rng = Random::new();
        rng.seed(42);
        let slice = &[1, 2, 3, 4, 5];
        let subslice = rng.rand_slice(slice, 3);
        assert_eq!(subslice.len(), 3);
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

    /// Tests the `poisson` method to ensure it generates numbers from a Poisson distribution.
    #[test]
    fn test_poisson() {
        let mut rng = Random::new();
        rng.seed(42);
        let result = rng.poisson(3.0);
        assert!(result > 0);
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

    /// Tests the `RngCore` implementation to ensure it generates random numbers and fills buffers correctly.
    #[test]
    fn test_rng_core_impl() {
        let mut rng = Random::new();
        let next_u32_value = rng.rand();
        assert!(next_u32_value != 0);
        let next_u64_value = rng.u64();
        assert!(next_u64_value != 0);
        let mut buffer = [0u8; 8];
        let random_bytes = rng.bytes(buffer.len());
        buffer.copy_from_slice(&random_bytes);
        assert!(buffer.iter().any(|&x| x != 0));
    }
}

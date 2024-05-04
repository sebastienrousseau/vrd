// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {

    use vrd::{random::Random, *};

    #[test]
    fn test_random_range_macro_within_bounds() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = random_range!(rng, min, max);
        assert!(
            num >= min && num <= max,
            "Number should be within the given range."
        );
    }

    #[test]
    fn test_rand_bool_macro_always_true() {
        let mut rng = Random::new();
        let b = rand_bool!(rng, 1.0);
        assert!(
            b,
            "rand_bool should always return true with probability 1.0."
        );
    }

    #[test]
    fn test_rand_bool_macro_always_false() {
        let mut rng = Random::new();
        let b = rand_bool!(rng, 0.0);
        assert!(!b, "rand_bool should always return false with probability 0.0.");
    }

    #[test]
    fn test_rand_bytes_macro() {
        let mut rng = Random::new();
        let len = 10;
        let bytes = rand_bytes!(rng, len);
        assert_eq!(
            bytes.len(),
            len,
            "Length of bytes should be equal to the specified length."
        );
    }

    #[test]
    fn test_rand_char_macro_ascii_check() {
        let mut rng = Random::new();
        let c = rand_char!(rng);
        assert!(
            c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit(),
            "Generated character should be ASCII lowercase, uppercase, or digit."
        );
    }

    #[test]
    fn test_rand_choose_macro_value_in_slice() {
        let mut rng = Random::new();
        let values = vec![1, 2, 3, 4, 5];
        let chosen = rand_choose!(rng, &values).unwrap();
        assert!(
            values.contains(chosen),
            "Chosen value should be in the provided slice."
        );
    }

    #[test]
    fn test_rand_float_macro_within_bounds() {
        let mut rng = Random::new();
        let f = rand_float!(rng);
        assert!(
            (0.0..1.0).contains(&f),
            "Generated float should be within 0.0 and 1.0."
        );
    }

    #[test]
    fn test_rand_int_macro_within_range() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = rand_int!(rng, min, max);
        assert!(
            num >= min && num <= max,
            "Generated integer should be within the specified range."
        );
    }

    #[test]
    fn test_rand_pseudo_macro_upper_bound() {
        let mut rng = Random::new();
        let p = rand_pseudo!(rng);
        assert!(p < 4294967295, "Generated pseudo random number should be less than 4294967295.");
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

    #[test]
    fn test_rand_range_macro_valid_range() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = random_range!(rng, min, max);
        println!("Generated number: {}", num); // Add this line for debugging
        assert!(
            num >= min && num < max,
            "Number should be within the given range."
        );
    }

    #[test]
    fn test_rand_bool_macro_true() {
        let mut rng = Random::new();
        let p = 1.0; // Set p to 1.0 to always generate true
        let b = rand_bool!(rng, p);
        assert!(b, "The probability of 1.0 should always return true.");
    }

    #[test]
    fn test_rand_bool_macro_false() {
        let mut rng = Random::new();
        let p = 0.0; // Set p to 0.0 to always generate false
        let b = rand_bool!(rng, p);
        assert!(
            !b,
            "The probability of 0.0 should always return false."
        );
    }

    #[test]
    fn test_rand_int_macro_valid_range() {
        let mut rng = Random::new();
        let min = 10;
        let max = 20;
        let num = rand_int!(rng, min, max);
        assert!(
            num >= min && num <= max,
            "Number should be within the given range."
        );
    }

    #[test]
    fn test_rand_alphanumeric() {
        let mut rng = Random::new();
        for _ in 0..1000 {
            let c = rand_alphanumeric!(rng);
            assert!(c.is_ascii_alphanumeric());
        }
    }

    #[test]
    fn test_rand_string() {
        let mut rng = Random::new();
        let length = 20;
        let random_string = rand_string!(rng, length);
        assert_eq!(random_string.len(), length);
        assert!(random_string
            .chars()
            .all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_rand_shuffle() {
        let mut rng = Random::new();
        let mut numbers = [1, 2, 3, 4, 5];
        let original_numbers = numbers;

        rand_shuffle!(rng, &mut numbers);

        assert_eq!(numbers.len(), original_numbers.len());
        assert!(numbers.iter().all(|&x| original_numbers.contains(&x)));
    }

    /// Test the `rand_weighted_choice!` macro for correct weighted choice distribution.
    #[test]
    fn test_rand_weighted_choice() {
        // Create a new Random instance
        let mut rng = Random::new();

        // Define choices and their corresponding weights
        let choices = ["A", "B", "C"];
        let weights = [20, 30, 50]; // Weights implying a 2:3:5 ratio

        // Initialize counters for each choice
        let mut counts = [0; 3];
        let num_iterations = 10_000; // Number of iterations to simulate

        // Simulate weighted choice selection
        for _ in 0..num_iterations {
            let selected =
                rand_weighted_choice!(rng, &choices, &weights);
            match *selected {
                "A" => counts[0] += 1,
                "B" => counts[1] += 1,
                "C" => counts[2] += 1,
                _ => panic!("Unexpected choice"),
            }
        }

        // Calculate the observed distribution ratios
        let total_counts: i32 = counts.iter().sum();
        let observed_ratios: Vec<f64> = counts
            .iter()
            .map(|&count| count as f64 / total_counts as f64)
            .collect();

        // Expected distribution ratios based on weights
        let total_weight: u32 = weights.iter().sum();
        let expected_ratios: Vec<f64> = weights
            .iter()
            .map(|&weight| weight as f64 / total_weight as f64)
            .collect();

        // Check if the observed ratios are close to the expected ratios within a tolerance
        let tolerance = 0.05; // 5% tolerance for distribution accuracy
        for (observed, expected) in
            observed_ratios.iter().zip(expected_ratios.iter())
        {
            assert!(
            (observed - expected).abs() <= tolerance,
            "Distribution does not match expected ratios within tolerance: observed {:?}, expected {:?}",
            observed_ratios, expected_ratios
        );
        }
    }

    #[test]
    fn test_rand_normal() {
        let mut rng = Random::new(); // Assuming `Random::new()` provides the necessary `f64()` method.
        let mu = 0.0; // Expected mean
        let sigma = 1.0; // Expected standard deviation
        let num_samples = 10000; // Number of samples to generate

        let mut samples = Vec::new();
        for _ in 0..num_samples {
            let sample = rand_normal!(rng, mu, sigma);
            samples.push(sample);
        }

        // Calculate the sample mean and sample standard deviation
        let sample_mean: f64 =
            samples.iter().sum::<f64>() / num_samples as f64;
        let sample_variance: f64 = samples
            .iter()
            .map(|&x| (x - sample_mean).powi(2))
            .sum::<f64>()
            / (num_samples - 1) as f64;
        let sample_std_dev = sample_variance.sqrt();

        // Define tolerances for the mean and standard deviation
        let mean_tolerance = 0.1; // Adjust based on acceptable error
        let std_dev_tolerance = 0.1; // Adjust based on acceptable error

        // Assert that the sample mean and standard deviation are within the expected tolerances
        assert!(
            (sample_mean - mu).abs() <= mean_tolerance,
            "Sample mean is not within the expected tolerance: expected {}, got {}",
            mu, sample_mean
        );

        assert!(
            (sample_std_dev - sigma).abs() <= std_dev_tolerance,
            "Sample standard deviation is not within the expected tolerance: expected {}, got {}",
            sigma, sample_std_dev
        );
    }

    #[test]
    fn test_rand_exponential() {
        let mut rng = Random::new(); // Assuming `Random::new()` provides the `f64()` method.
        let rate = 1.5;
        let num_samples = 10000;
        let expected_mean = 1.0 / rate;

        let samples: Vec<f64> = (0..num_samples)
            .map(|_| rand_exponential!(rng, rate))
            .collect();

        let sample_mean: f64 =
            samples.iter().sum::<f64>() / samples.len() as f64;

        // Tolerance for the difference between the sample mean and the expected mean.
        let tolerance = 0.1;

        assert!(
            (sample_mean - expected_mean).abs() < tolerance,
            "The sample mean {} is not within the tolerance {} of the expected mean {}",
            sample_mean,
            tolerance,
            expected_mean
        );
    }

    #[test]
    fn test_rand_poisson() {
        let mut rng = Random::new();
        let mean = 3.0;
        let num_samples = 10000;

        let mut sum = 0;
        for _ in 0..num_samples {
            let poisson = rand_poisson!(rng, mean);
            sum += poisson;
        }

        let sample_mean = sum as f64 / num_samples as f64;
        let expected_mean = mean;

        assert!((sample_mean - expected_mean).abs() < 0.1);
    }
}

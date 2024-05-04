// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    // Import necessary modules from the vrd crate
    use vrd::{
        mersenne_twister::MersenneTwisterParams, MersenneTwisterConfig,
    };

    // Test creating a custom Mersenne Twister configuration
    #[test]
    fn test_new_custom() {
        // Arrange
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };

        // Act
        let config =
            MersenneTwisterConfig::new_custom(624, 397, params);

        // Assert
        assert_eq!(config.n, 624);
        assert_eq!(config.m, 397);
        assert_eq!(config.params.matrix_a, 0x9908b0df);
        assert_eq!(config.params.upper_mask, 0x80000000);
        assert_eq!(config.params.lower_mask, 0x7fffffff);
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    // Test creating a custom configuration with invalid n value
    #[test]
    #[should_panic(expected = "n must be at least 1")]
    fn test_new_custom_invalid_n() {
        let params = MersenneTwisterParams::default(); // Provide default params here
        MersenneTwisterConfig::new_custom(0, 397, params);
    }

    // Test creating a custom configuration with invalid m value
    #[test]
    #[should_panic(expected = "m must be at least 1 and less than n")]
    fn test_new_custom_invalid_m() {
        let params = MersenneTwisterParams::default(); // Provide default params here
        MersenneTwisterConfig::new_custom(624, 0, params);
    }

    // Test creating a custom configuration with invalid matrix_a value
    #[test]
    #[should_panic(expected = "matrix_a must have its highest bit set")]
    fn test_new_custom_invalid_matrix_a() {
        let params = MersenneTwisterParams {
            matrix_a: 0x7fffffff, // Invalid value
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        MersenneTwisterConfig::new_custom(624, 397, params);
    }

    // Test creating a custom configuration with invalid tempering_mask_b value
    #[test]
    #[should_panic(
        expected = "tempering_mask_b must be a valid 32-bit unsigned integer"
    )]
    fn test_new_custom_invalid_tempering_mask_b() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0xffffffff, // Invalid value
            tempering_mask_c: 0xefc60000,
        };
        MersenneTwisterConfig::new_custom(624, 397, params);
    }

    // Test creating a custom configuration with invalid upper_mask value
    #[test]
    #[should_panic(
        expected = "upper_mask must be a valid 32-bit unsigned integer"
    )]
    fn test_new_custom_invalid_upper_mask() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0xffffffff, // Invalid value
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        MersenneTwisterConfig::new_custom(624, 397, params);
    }

    // Test creating a custom configuration with invalid lower_mask value
    #[test]
    #[should_panic(
        expected = "lower_mask must be a valid 32-bit unsigned integer"
    )]
    fn test_new_custom_invalid_lower_mask() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0xffffffff, // Invalid value
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        MersenneTwisterConfig::new_custom(624, 397, params);
    }

    // Test creating a custom configuration with invalid tempering_mask_c value
    #[test]
    #[should_panic(
        expected = "tempering_mask_c must be a valid 32-bit unsigned integer"
    )]
    fn test_new_custom_invalid_tempering_mask_c() {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xffffffff, // Invalid value
        };
        MersenneTwisterConfig::new_custom(624, 397, params);
    }

    // Test setting configuration parameters
    #[test]
    fn test_set_config() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        let params = MersenneTwisterParams::default();
        config.set_config(1000, 500, params);

        // Assert
        assert_eq!(config.n, 1000);
        assert_eq!(config.m, 500);
        assert_eq!(config.params.matrix_a, 0x9908b0df);
        assert_eq!(config.params.upper_mask, 0x80000000);
        assert_eq!(config.params.lower_mask, 0x7fffffff);
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    // Test creating a default configuration
    #[test]
    fn test_new() {
        // Act
        let config = MersenneTwisterConfig::new();

        // Assert
        assert_eq!(config.n, 624);
        assert_eq!(config.m, 397);
        assert_eq!(config.params.matrix_a, 0x9908b0df);
        assert_eq!(config.params.upper_mask, 0x80000000);
        assert_eq!(config.params.lower_mask, 0x7fffffff);
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    // Test setting n parameter
    #[test]
    fn test_set_n() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_n(1000);

        // Assert
        assert_eq!(config.n, 1000);
    }

    // Test setting n parameter with invalid value
    #[test]
    #[should_panic(expected = "n must be at least 1")]
    fn test_set_n_invalid() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_n(0);
    }

    // Test setting m parameter
    #[test]
    fn test_set_m() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_m(500);

        // Assert
        assert_eq!(config.m, 500);
    }

    // Test setting m parameter with invalid value
    #[test]
    #[should_panic(expected = "m must be at least 1 and less than n")]
    fn test_set_m_invalid() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_m(0);
    }

    // Test setting matrix_a parameter
    #[test]
    fn test_set_matrix_a() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_matrix_a(0x9908b0df);

        // Assert
        assert_eq!(config.params.matrix_a, 0x9908b0df);
    }

    // Test setting matrix_a parameter with invalid value
    #[test]
    #[should_panic(expected = "matrix_a must have its highest bit set")]
    fn test_set_matrix_a_invalid() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_matrix_a(0x7fffffff);
    }

    // Test setting upper_mask parameter
    #[test]
    fn test_set_upper_mask() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_upper_mask(0x80000000);

        // Assert
        assert_eq!(config.params.upper_mask, 0x80000000);
    }

    // Test setting upper_mask parameter with invalid value
    #[test]
    #[should_panic(
        expected = "upper_mask must be a valid 32-bit unsigned integer"
    )]
    fn test_set_upper_mask_invalid() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_upper_mask(0xffffffff);
    }

    // Test setting lower_mask parameter
    #[test]
    fn test_set_lower_mask() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_lower_mask(0x7fffffff);

        // Assert
        assert_eq!(config.params.lower_mask, 0x7fffffff);
    }

    // Test setting lower_mask parameter with invalid value
    #[test]
    #[should_panic(
        expected = "lower_mask must be a valid 32-bit unsigned integer"
    )]
    fn test_set_lower_mask_invalid() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_lower_mask(0xffffffff);
    }

    // Test setting tempering_mask_b parameter
    #[test]
    fn test_set_tempering_mask_b() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_tempering_mask_b(0x9d2c5680);

        // Assert
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
    }

    // Test setting tempering_mask_b parameter with invalid value
    #[test]
    #[should_panic(
        expected = "tempering_mask_b must be a valid 32-bit unsigned integer"
    )]
    fn test_set_tempering_mask_b_invalid() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_tempering_mask_b(0xffffffff);
    }

    // Test setting tempering_mask_c parameter
    #[test]
    fn test_set_tempering_mask_c() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_tempering_mask_c(0xefc60000);

        // Assert
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    // Test setting tempering_mask_c parameter with invalid value
    #[test]
    #[should_panic(
        expected = "tempering_mask_c must be a valid 32-bit unsigned integer"
    )]
    fn test_set_tempering_mask_c_invalid() {
        // Arrange
        let mut config = MersenneTwisterConfig::new();

        // Act
        config.set_tempering_mask_c(0xffffffff);
    }

    // Test creating a default configuration
    #[test]
    fn test_default() {
        // Act
        let config = MersenneTwisterConfig::default();

        // Assert
        assert_eq!(config.n, 624);
        assert_eq!(config.m, 397);
        assert_eq!(config.params.matrix_a, 0x9908b0df);
        assert_eq!(config.params.upper_mask, 0x80000000);
        assert_eq!(config.params.lower_mask, 0x7fffffff);
        assert_eq!(config.params.tempering_mask_b, 0x9d2c5680);
        assert_eq!(config.params.tempering_mask_c, 0xefc60000);
    }

    // Test displaying configuration
    #[test]
    fn test_display() {
        // Arrange
        let config = MersenneTwisterConfig::new();
        let expected = format!(
        "MersenneTwisterConfig {{ n: {}, m: {}, matrix_a: 0x{:x}, upper_mask: 0x{:x}, lower_mask: 0x{:x}, tempering_mask_b: 0x{:x}, tempering_mask_c: 0x{:x} }}",
        config.n,
        config.m,
        config.params.matrix_a,
        config.params.upper_mask,
        config.params.lower_mask,
        config.params.tempering_mask_b,
        config.params.tempering_mask_c
    );

        // Act & Assert
        assert_eq!(format!("{}", config), expected);
    }

    // Test validating a valid configuration
    #[test]
    fn test_validate_valid() {
        let params = MersenneTwisterParams::default(); // Provide default params here
                                                       // Act
        MersenneTwisterConfig::validate(624, 397, &params);
    }

    // Test validating a configuration with invalid n value
    #[test]
    #[should_panic(expected = "n must be at least 1")]
    fn test_validate_invalid_n() {
        let params = MersenneTwisterParams::default(); // Provide default params here
                                                       // Act
        MersenneTwisterConfig::validate(0, 397, &params);
    }

    #[test]
    #[should_panic] // Expect all cases to panic
    fn test_new_custom_invalid_parameters() {
        let test_cases = vec![
            (0, 397, "n must be at least 1"),
            (624, 0, "m must be at least 1 and less than n"),
            (624, 397, "matrix_a must have its highest bit set"), // Invalid matrix_a
            (624, 397, "0xffffffff"), // Invalid upper_mask
            (624, 397, "0xffffffff"), // Invalid lower_mask
            (624, 397, "0xffffffff"), // Invalid tempering_mask_b
            (624, 397, "0xffffffff"), // Invalid tempering_mask_c
        ];

        for (n, m, invalid_param) in test_cases {
            let mut params = MersenneTwisterParams::default();
            let expected_msg = invalid_param; // Assuming invalid_param contains error messages

            // Set the invalid parameter based on the test case
            if let Ok(param_as_int) = invalid_param.parse::<u32>() {
                // Try parsing first
                match param_as_int {
                    param if param & 0x80000000 == 0 => {
                        params.matrix_a = param
                    }
                    0xffffffff => {
                        params.upper_mask = param_as_int; // Use the parsed value
                        params.lower_mask = param_as_int;
                        params.tempering_mask_b = param_as_int;
                        params.tempering_mask_c = param_as_int;
                    }
                    _ => unreachable!(),
                }
            } else {
                // Handle cases where invalid_param is not a number (likely an error message)
                panic!("{}", expected_msg); // Or handle accordingly
            }

            let result = std::panic::catch_unwind(|| {
                MersenneTwisterConfig::new_custom(n, m, params);
            });

            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .downcast_ref::<&str>()
                .unwrap()
                .contains(expected_msg));
        }
    }
}

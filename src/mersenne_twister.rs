// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{BufReader, BufWriter},
};

/// Configuration parameters for the Mersenne Twister algorithm.
///
/// This struct contains the constant values required for the Mersenne Twister algorithm.
///
/// # Fields
///
/// - `matrix_a`: A constant value used in the Mersenne Twister algorithm. It must have its highest bit set (0x80000000).
/// - `upper_mask`: A constant value used for masking the upper bits of the generated values (0x80000000).
/// - `lower_mask`: A constant value used for masking the lower bits of the generated values (0x7fffffff).
/// - `tempering_mask_b`: A constant value used for tempering the generated values (0x9d2c5680).
/// - `tempering_mask_c`: A constant value used for tempering the generated values (0xefc60000).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct MersenneTwisterParams {
    /// A constant value used in the Mersenne Twister algorithm. It must have its highest bit set (0x80000000).
    pub matrix_a: u32,
    /// A constant value used for masking the upper bits of the generated values (0x80000000).
    pub upper_mask: u32,
    /// A constant value used for masking the lower bits of the generated values (0x7fffffff).
    pub lower_mask: u32,
    /// A constant value used for tempering the generated values (0x9d2c5680).
    pub tempering_mask_b: u32,
    /// A constant value used for tempering the generated values (0xefc60000).
    pub tempering_mask_c: u32,
}

/// Configuration for the Mersenne Twister algorithm.
///
/// This struct contains the configurable parameters for the Mersenne Twister algorithm.
///
/// # Fields
///
/// - `n`: The number of elements in the array used for the Mersenne Twister algorithm. Its value is set to 624 for optimal performance. Must be at least 1.
/// - `m`: The number of elements to skip in the array used for the Mersenne Twister algorithm. Its value is set to 397 for optimal performance. Must be at least 1 and less than `n`.
/// - `params`: An instance of `MersenneTwisterParams` containing the constant values for the Mersenne Twister algorithm.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct MersenneTwisterConfig {
    /// The number of elements in the array used for the Mersenne Twister algorithm.
    /// Its value is set to 624 for optimal performance.
    /// Must be at least 1.
    pub n: usize,

    /// The number of elements to skip in the array used for the Mersenne Twister algorithm.
    /// Its value is set to 397 for optimal performance.
    /// Must be at least 1 and less than `n`.
    pub m: usize,

    /// Configuration parameters for the Mersenne Twister algorithm.
    pub params: MersenneTwisterParams,
}

/// Implementation of the `MersenneTwisterConfig` struct.
impl MersenneTwisterConfig {
    /// Creates a new `MersenneTwisterConfig` with customizable values.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of elements in the array.
    /// * `m` - The number of elements to skip.
    /// * `params` - Configuration parameters for the Mersenne Twister algorithm.
    ///
    /// # Panics
    ///
    /// This function panics if any of the provided parameters are outside of their valid range.
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::{MersenneTwisterConfig, MersenneTwisterParams};
    ///
    /// let params = MersenneTwisterParams {
    ///     matrix_a: 0x9908b0df,
    ///     upper_mask: 0x80000000,
    ///     lower_mask: 0x7fffffff,
    ///     tempering_mask_b: 0x9d2c5680,
    ///     tempering_mask_c: 0xefc60000,
    /// };
    ///
    /// let config = MersenneTwisterConfig::new_custom(624, 397, params);
    /// ```
    pub fn new_custom(
        n: usize,
        m: usize,
        params: MersenneTwisterParams,
    ) -> MersenneTwisterConfig {
        MersenneTwisterConfig::validate(n, m, &params);
        MersenneTwisterConfig { n, m, params }
    }

    /// Sets all the fields of the `MersenneTwisterConfig` struct at once.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of elements in the array.
    /// * `m` - The number of elements to skip.
    /// * `params` - Configuration parameters for the Mersenne Twister algorithm.
    ///
    /// # Panics
    ///
    /// This function panics if any of the provided parameters are outside of their valid range.
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::{MersenneTwisterConfig, MersenneTwisterParams};
    ///
    /// let mut config = MersenneTwisterConfig::new();
    ///
    /// let params = MersenneTwisterParams {
    ///     matrix_a: 0x9908b0df,
    ///     upper_mask: 0x80000000,
    ///     lower_mask: 0x7fffffff,
    ///     tempering_mask_b: 0x9d2c5680,
    ///     tempering_mask_c: 0xefc60000,
    /// };
    ///
    /// config.set_config(1000, 500, params);
    /// ```
    pub fn set_config(
        &mut self,
        n: usize,
        m: usize,
        params: MersenneTwisterParams,
    ) {
        MersenneTwisterConfig::validate(n, m, &params);
        self.n = n;
        self.m = m;
        self.params = params;
    }

    /// Validates the parameters for a MersenneTwisterConfig.
    ///
    /// # Panics
    ///
    /// This function panics if any of the provided parameters are outside of their valid range.
    pub fn validate(
        n: usize,
        m: usize,
        params: &MersenneTwisterParams,
    ) {
        assert!(n >= 1, "n must be at least 1");
        assert!(
            m >= 1 && m < n,
            "m must be at least 1 and less than n"
        );
        assert_eq!(
            params.matrix_a & 0x80000000,
            0x80000000,
            "matrix_a must have its highest bit set"
        );
        assert_eq!(
            params.upper_mask, 0x80000000,
            "upper_mask must be a valid 32-bit unsigned integer"
        );
        assert_eq!(
            params.lower_mask, 0x7fffffff,
            "lower_mask must be a valid 32-bit unsigned integer"
        );
        assert_eq!(
            params.tempering_mask_b, 0x9d2c5680,
            "tempering_mask_b must be a valid 32-bit unsigned integer"
        );
        assert_eq!(
            params.tempering_mask_c, 0xefc60000,
            "tempering_mask_c must be a valid 32-bit unsigned integer"
        );
    }

    /// Creates a new `MersenneTwisterConfig` with default values.
    ///
    /// Default values are as follows:
    ///
    /// - `n`: 624
    /// - `m`: 397
    /// - `matrix_a`: 0x9908b0df
    /// - `upper_mask`: 0x80000000
    /// - `lower_mask`: 0x7fffffff
    /// - `tempering_mask_b`: 0x9d2c5680
    /// - `tempering_mask_c`: 0xefc60000
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::MersenneTwisterConfig;
    ///
    /// let config = MersenneTwisterConfig::new();
    /// ```
    pub fn new() -> MersenneTwisterConfig {
        let params = MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        };
        MersenneTwisterConfig::new_custom(624, 397, params)
    }

    /// Sets the number of elements in the array.
    ///
    /// # Panics
    ///
    /// This function panics if the provided parameter is outside of its valid range.
    pub fn set_n(&mut self, n: usize) {
        assert!(n >= 1, "n must be at least 1");
        self.n = n;
    }

    /// Sets the number of elements to skip.
    ///
    /// # Panics
    ///
    /// This function panics if the provided parameter is outside of its valid range.
    pub fn set_m(&mut self, m: usize) {
        assert!(
            m >= 1 && m < self.n,
            "m must be at least 1 and less than n"
        );
        self.m = m;
    }

    /// Sets the matrix_a constant value.
    ///
    /// # Panics
    ///
    /// This function panics if the provided parameter is not valid.
    pub fn set_matrix_a(&mut self, matrix_a: u32) {
        assert_eq!(
            matrix_a & 0x80000000,
            0x80000000,
            "matrix_a must have its highest bit set"
        );
        self.params.matrix_a = matrix_a;
    }

    /// Sets the upper_mask constant value.
    ///
    /// # Panics
    ///
    /// This function panics if the provided parameter is not valid.
    pub fn set_upper_mask(&mut self, upper_mask: u32) {
        assert_eq!(
            upper_mask, 0x80000000,
            "upper_mask must be a valid 32-bit unsigned integer"
        );
        self.params.upper_mask = upper_mask;
    }

    /// Sets the lower_mask constant value.
    ///
    /// # Panics
    ///
    /// This function panics if the provided parameter is not valid.
    pub fn set_lower_mask(&mut self, lower_mask: u32) {
        assert_eq!(
            lower_mask, 0x7fffffff,
            "lower_mask must be a valid 32-bit unsigned integer"
        );
        self.params.lower_mask = lower_mask;
    }

    /// Sets the tempering_mask_b constant value.
    ///
    /// # Panics
    ///
    /// This function panics if the provided parameter is not valid.
    pub fn set_tempering_mask_b(&mut self, tempering_mask_b: u32) {
        assert_eq!(
            tempering_mask_b, 0x9d2c5680,
            "tempering_mask_b must be a valid 32-bit unsigned integer"
        );
        self.params.tempering_mask_b = tempering_mask_b;
    }

    /// Sets the tempering_mask_c constant value.
    ///
    /// # Panics
    ///
    /// This function panics if the provided parameter is not valid.
    pub fn set_tempering_mask_c(&mut self, tempering_mask_c: u32) {
        assert_eq!(
            tempering_mask_c, 0xefc60000,
            "tempering_mask_c must be a valid 32-bit unsigned integer"
        );
        self.params.tempering_mask_c = tempering_mask_c;
    }

    /// Serialize a MersenneTwisterConfig instance to a JSON file
    ///
    /// # Arguments
    /// * `config` - A reference to a MersenneTwisterConfig instance
    /// * `filename` - A string slice containing the filename
    ///
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - A result indicating success or failure
    ///
    pub fn serialize_to_file(
        config: &MersenneTwisterConfig,
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, config)?;
        Ok(())
    }

    /// Deserialize a MersenneTwisterConfig instance from a JSON file
    ///
    /// # Arguments
    /// * `filename` - A string slice containing the filename
    ///
    /// # Returns
    /// * `Result<MersenneTwisterConfig, Box<dyn std::error::Error>>` - A result containing the deserialized MersenneTwisterConfig instance
    ///
    pub fn deserialize_from_file(
        filename: &str,
    ) -> Result<MersenneTwisterConfig, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}

impl Default for MersenneTwisterConfig {
    fn default() -> Self {
        MersenneTwisterConfig::new()
    }
}

impl fmt::Display for MersenneTwisterConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MersenneTwisterConfig {{ n: {}, m: {}, matrix_a: 0x{:x}, upper_mask: 0x{:x}, lower_mask: 0x{:x}, tempering_mask_b: 0x{:x}, tempering_mask_c: 0x{:x} }}",
            self.n,
            self.m,
            self.params.matrix_a,
            self.params.upper_mask,
            self.params.lower_mask,
            self.params.tempering_mask_b,
            self.params.tempering_mask_c,
        )
    }
}

impl Default for MersenneTwisterParams {
    fn default() -> Self {
        MersenneTwisterParams {
            matrix_a: 0x9908b0df,
            upper_mask: 0x80000000,
            lower_mask: 0x7fffffff,
            tempering_mask_b: 0x9d2c5680,
            tempering_mask_c: 0xefc60000,
        }
    }
}

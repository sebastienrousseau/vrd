// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Configuration for the Mersenne Twister algorithm.
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

    /// A constant value used in the Mersenne Twister algorithm.
    /// Must have its highest bit set.
    pub matrix_a: u32,

    /// A constant value used in the Mersenne Twister algorithm.
    /// Must be a valid 32-bit unsigned integer.
    pub upper_mask: u32,

    /// A constant value used in the Mersenne Twister algorithm.
    /// Must be a valid 32-bit unsigned integer.
    pub lower_mask: u32,

    /// A constant value used in the Mersenne Twister algorithm.
    /// Must be a valid 32-bit unsigned integer.
    pub tempering_mask_b: u32,

    /// A constant value used in the Mersenne Twister algorithm.
    /// Must be a valid 32-bit unsigned integer.
    pub tempering_mask_c: u32,
}

/// Implementation of the `MersenneTwisterConfig` struct.
impl MersenneTwisterConfig {
    /// Creates a new `MersenneTwisterConfig` with customizable values.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of elements in the array.
    /// * `m` - The number of elements to skip.
    /// * `matrix_a` - Constant value used in the algorithm.
    /// * `upper_mask` - Constant value used in the algorithm.
    /// * `lower_mask` - Constant value used in the algorithm.
    /// * `tempering_mask_b` - Constant value used in the algorithm.
    /// * `tempering_mask_c` - Constant value used in the algorithm.
    ///
    /// # Panics
    ///
    /// This function panics if any of the provided parameters are outside of their valid range.
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::MersenneTwisterConfig;
    ///
    /// let config = MersenneTwisterConfig::new_custom(
    ///     624,                // n
    ///     397,                // m
    ///     0x9908b0df,         // matrix_a
    ///     0x80000000,         // upper_mask
    ///     0x7fffffff,         // lower_mask
    ///     0x9d2c5680,         // tempering_mask_b
    ///     0xefc60000          // tempering_mask_c
    /// );
    /// ```
    pub fn new_custom(
        n: usize,
        m: usize,
        matrix_a: u32,
        upper_mask: u32,
        lower_mask: u32,
        tempering_mask_b: u32,
        tempering_mask_c: u32,
    ) -> MersenneTwisterConfig {
        MersenneTwisterConfig::validate(
            n,
            m,
            matrix_a,
            upper_mask,
            lower_mask,
            tempering_mask_b,
            tempering_mask_c,
        );
        MersenneTwisterConfig {
            n,
            m,
            matrix_a,
            upper_mask,
            lower_mask,
            tempering_mask_b,
            tempering_mask_c,
        }
    }

    /// Sets all the fields of the `MersenneTwisterConfig` struct at once.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of elements in the array.
    /// * `m` - The number of elements to skip.
    /// * `matrix_a` - Constant value used in the algorithm.
    /// * `upper_mask` - Constant value used in the algorithm.
    /// * `lower_mask` - Constant value used in the algorithm.
    /// * `tempering_mask_b` - Constant value used in the algorithm.
    /// * `tempering_mask_c` - Constant value used in the algorithm.
    ///
    /// # Panics
    ///
    /// This function panics if any of the provided parameters are outside of their valid range.
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::MersenneTwisterConfig;
    ///
    /// let mut config = MersenneTwisterConfig::new();
    /// config.set_config(
    ///     1000,               // n
    ///     500,                // m
    ///     0x9908b0df,         // matrix_a
    ///     0x80000000,         // upper_mask
    ///     0x7fffffff,         // lower_mask
    ///     0x9d2c5680,         // tempering_mask_b
    ///     0xefc60000          // tempering_mask_c
    /// );
    /// ```
    pub fn set_config(
        &mut self,
        n: usize,
        m: usize,
        matrix_a: u32,
        upper_mask: u32,
        lower_mask: u32,
        tempering_mask_b: u32,
        tempering_mask_c: u32,
    ) {
        MersenneTwisterConfig::validate(
            n,
            m,
            matrix_a,
            upper_mask,
            lower_mask,
            tempering_mask_b,
            tempering_mask_c,
        );
        self.n = n;
        self.m = m;
        self.matrix_a = matrix_a;
        self.upper_mask = upper_mask;
        self.lower_mask = lower_mask;
        self.tempering_mask_b = tempering_mask_b;
        self.tempering_mask_c = tempering_mask_c;
    }

    /// Validates the parameters for a MersenneTwisterConfig.
    ///
    /// # Panics
    ///
    /// This function panics if any of the provided parameters are outside of their valid range.
    pub fn validate(
        n: usize,
        m: usize,
        matrix_a: u32,
        upper_mask: u32,
        lower_mask: u32,
        tempering_mask_b: u32,
        tempering_mask_c: u32,
    ) {
        assert!(n >= 1, "n must be at least 1");
        assert!(
            m >= 1 && m < n,
            "m must be at least 1 and less than n"
        );
        assert_eq!(
            matrix_a & 0x80000000,
            0x80000000,
            "matrix_a must have its highest bit set"
        );
        assert_eq!(
            upper_mask, 0x80000000,
            "upper_mask must be a valid 32-bit unsigned integer"
        );
        assert_eq!(
            lower_mask, 0x7fffffff,
            "lower_mask must be a valid 32-bit unsigned integer"
        );
        assert_eq!(
            tempering_mask_b, 0x9d2c5680,
            "tempering_mask_b must be a valid 32-bit unsigned integer"
        );
        assert_eq!(
            tempering_mask_c, 0xefc60000,
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
        MersenneTwisterConfig::new_custom(
            624, 397, 0x9908b0df, 0x80000000, 0x7fffffff, 0x9d2c5680,
            0xefc60000,
        )
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
        self.matrix_a = matrix_a;
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
        self.upper_mask = upper_mask;
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
        self.lower_mask = lower_mask;
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
        self.tempering_mask_b = tempering_mask_b;
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
        self.tempering_mask_c = tempering_mask_c;
    }
}

impl Default for MersenneTwisterConfig {
    fn default() -> Self {
        MersenneTwisterConfig::new()
    }
}

impl fmt::Display for MersenneTwisterConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MersenneTwisterConfig {{ n: {}, m: {}, matrix_a: 0x{:x}, upper_mask: 0x{:x}, lower_mask: 0x{:x}, tempering_mask_b: 0x{:x}, tempering_mask_c: 0x{:x} }}", 
            self.n, self.m, self.matrix_a, self.upper_mask, self.lower_mask, self.tempering_mask_b, self.tempering_mask_c)
    }
}

// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
/// Configuration for the Mersenne Twister algorithm.
pub struct MersenneTwisterConfig {
    /// The number of elements in the array used for the Mersenne Twister algorithm.
    /// Its value is set to 624 for optimal performance.
    pub n: usize,

    /// The number of elements to skip in the array used for the Mersenne Twister algorithm.
    /// Its value is set to 397 for optimal performance.
    pub m: usize,

    /// A constant value used in the Mersenne Twister algorithm.
    pub matrix_a: u32,

    /// A constant value used in the Mersenne Twister algorithm.
    pub upper_mask: u32,

    /// A constant value used in the Mersenne Twister algorithm.
    pub lower_mask: u32,

    /// A constant value used in the Mersenne Twister algorithm.
    pub tempering_mask_b: u32,

    /// A constant value used in the Mersenne Twister algorithm.
    pub tempering_mask_c: u32,
}

/// Implementation of the `MersenneTwisterConfig` struct.
impl MersenneTwisterConfig {
    /// Creates a new `MersenneTwisterConfig` with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use vrd::MersenneTwisterConfig;
    /// let config = MersenneTwisterConfig::new();
    /// ```
    pub fn new() -> MersenneTwisterConfig {
        MersenneTwisterConfig {
            // n is the number of elements in the array used for the Mersenne
            // Twister algorithm. Its value is set to 624 for optimal performance.
            n: 624,
            // m is the number of elements to skip in the array used for the
            // Mersenne Twister algorithm. Its value is set to 397 for optimal
            // performance.
            m: 397, // Default value for m
            // matrix_a is a constant value used in the Mersenne Twister algorithm.
            matrix_a: 0x9908b0df,
            // upper_mask is a constant value used in the Mersenne Twister algorithm.
            upper_mask: 0x80000000, // Default value for upper_mask
            // lower_mask is a constant value used in the Mersenne Twister algorithm.
            lower_mask: 0x7fffffff,
            // tempering_mask_b is a constant value used in the Mersenne Twister algorithm.
            tempering_mask_b: 0x9d2c5680,
            // tempering_mask_c is a constant value used in the Mersenne Twister algorithm.
            tempering_mask_c: 0xefc60000, // Default value for tempering_mask_c
        }
    }
}

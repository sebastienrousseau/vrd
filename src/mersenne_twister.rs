// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{self, BufReader, BufWriter},
};

/// Custom error type for `MersenneTwisterConfig`.
///
/// This enum defines various errors that can occur when using the `MersenneTwisterConfig` struct.
#[derive(Debug)]
pub enum MersenneTwisterError {
    /// An error indicating invalid configuration parameters.
    InvalidConfig(String),
    /// An error indicating an issue with I/O operations.
    IoError(io::Error),
    /// An error indicating a problem with serialization or deserialization.
    SerializationError(String),
}

impl fmt::Display for MersenneTwisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MersenneTwisterError::InvalidConfig(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            MersenneTwisterError::IoError(err) => {
                write!(f, "I/O Error: {}", err)
            }
            MersenneTwisterError::SerializationError(msg) => {
                write!(f, "Serialization Error: {}", msg)
            }
        }
    }
}

impl std::error::Error for MersenneTwisterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MersenneTwisterError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for MersenneTwisterError {
    fn from(err: io::Error) -> MersenneTwisterError {
        MersenneTwisterError::IoError(err)
    }
}

/// Configuration parameters for the Mersenne Twister algorithm.
///
/// This struct contains the constant values required for the Mersenne Twister algorithm.
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

/// Configuration for the Mersenne Twister algorithm.
///
/// This struct contains the configurable parameters for the Mersenne Twister algorithm.
///
/// # Type Parameters
///
/// - `N`: The number of elements in the array used for the Mersenne Twister algorithm.
/// - `M`: The number of elements to skip in the array used for the Mersenne Twister algorithm.
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
pub struct MersenneTwisterConfig<const N: usize, const M: usize> {
    /// Configuration parameters for the Mersenne Twister algorithm.
    pub params: MersenneTwisterParams,
}

impl<const N: usize, const M: usize> MersenneTwisterConfig<N, M> {
    /// Creates a new `MersenneTwisterConfig` with customizable values.
    ///
    /// # Arguments
    ///
    /// * `params` - Configuration parameters for the Mersenne Twister algorithm.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `MersenneTwisterConfig` instance or a `MersenneTwisterError` if validation fails.
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
    /// let config = MersenneTwisterConfig::<624, 397>::new_custom(params).unwrap();
    /// ```
    pub fn new_custom(
        params: MersenneTwisterParams,
    ) -> Result<Self, MersenneTwisterError> {
        Self::validate(&params)?;
        Ok(MersenneTwisterConfig { params })
    }

    /// Validates the parameters for a `MersenneTwisterConfig`.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to a `MersenneTwisterParams` instance to validate.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    ///
    /// Returns a `MersenneTwisterError::InvalidConfig` if any of the provided parameters are outside of their valid range.
    pub fn validate(
        params: &MersenneTwisterParams,
    ) -> Result<(), MersenneTwisterError> {
        if N < 1 {
            return Err(MersenneTwisterError::InvalidConfig(
                "n must be at least 1".into(),
            ));
        }
        if M < 1 || M >= N {
            return Err(MersenneTwisterError::InvalidConfig(
                "m must be at least 1 and less than n".into(),
            ));
        }
        if params.matrix_a & 0x80000000 != 0x80000000 {
            return Err(MersenneTwisterError::InvalidConfig(
                "matrix_a must have its highest bit set".into(),
            ));
        }
        if params.upper_mask != 0x80000000 {
            return Err(MersenneTwisterError::InvalidConfig(
                "upper_mask must be 0x80000000".into(),
            ));
        }
        if params.lower_mask != 0x7fffffff {
            return Err(MersenneTwisterError::InvalidConfig(
                "lower_mask must be 0x7fffffff".into(),
            ));
        }
        if params.tempering_mask_b != 0x9d2c5680 {
            return Err(MersenneTwisterError::InvalidConfig(
                "tempering_mask_b must be 0x9d2c5680".into(),
            ));
        }
        if params.tempering_mask_c != 0xefc60000 {
            return Err(MersenneTwisterError::InvalidConfig(
                "tempering_mask_c must be 0xefc60000".into(),
            ));
        }
        Ok(())
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
    /// # Returns
    ///
    /// A `Result` containing the new `MersenneTwisterConfig` instance or a `MersenneTwisterError` if validation fails.
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::MersenneTwisterConfig;
    ///
    /// let config = MersenneTwisterConfig::<624, 397>::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, MersenneTwisterError> {
        Self::new_custom(MersenneTwisterParams::default())
    }

    /// Sets the configuration parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Configuration parameters for the Mersenne Twister algorithm.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    ///
    /// Returns a `MersenneTwisterError::InvalidConfig` if the parameters are invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::{MersenneTwisterConfig, MersenneTwisterParams};
    ///
    /// let mut config = MersenneTwisterConfig::<624, 397>::new().unwrap();
    /// let params = MersenneTwisterParams {
    ///     matrix_a: 0x9908b0df,
    ///     upper_mask: 0x80000000,
    ///     lower_mask: 0x7fffffff,
    ///     tempering_mask_b: 0x9d2c5680,
    ///     tempering_mask_c: 0xefc60000,
    /// };
    /// config.set_config(params).unwrap();
    /// ```
    pub fn set_config(
        &mut self,
        params: MersenneTwisterParams,
    ) -> Result<(), MersenneTwisterError> {
        Self::validate(&params)?;
        self.params = params;
        Ok(())
    }

    /// Serialize a `MersenneTwisterConfig` instance to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `filename` - A string slice containing the filename.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    ///
    /// Returns a `MersenneTwisterError::IoError` if an I/O error occurs, or `MersenneTwisterError::SerializationError` if serialization fails.
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::{MersenneTwisterConfig, MersenneTwisterParams};
    ///
    /// let config = MersenneTwisterConfig::<624, 397>::new().unwrap();
    /// config.serialize_to_file("config.json").unwrap();
    /// ```
    pub fn serialize_to_file(
        &self,
        filename: &str,
    ) -> Result<(), MersenneTwisterError> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self).map_err(|e| {
            MersenneTwisterError::SerializationError(e.to_string())
        })
    }

    /// Deserialize a `MersenneTwisterConfig` instance from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `filename` - A string slice containing the filename.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized `MersenneTwisterConfig` instance or an error.
    ///
    /// # Errors
    ///
    /// Returns a `MersenneTwisterError::IoError` if an I/O error occurs, or `MersenneTwisterError::SerializationError` if deserialization fails.
    ///
    /// # Example
    ///
    /// ```
    /// use vrd::mersenne_twister::MersenneTwisterConfig;
    ///
    /// let config = MersenneTwisterConfig::<624, 397>::deserialize_from_file("config.json").unwrap();
    /// ```
    pub fn deserialize_from_file(
        filename: &str,
    ) -> Result<Self, MersenneTwisterError> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| {
            MersenneTwisterError::SerializationError(e.to_string())
        })
    }

    /// Serialize the configuration to YAML.
    ///
    /// # Returns
    ///
    /// A `Result` containing the YAML string or an error.
    ///
    /// # Errors
    ///
    /// Returns a `MersenneTwisterError::SerializationError` if serialization fails.
    #[cfg(feature = "yaml")]
    pub fn serialize_to_yaml(
        &self,
    ) -> Result<String, MersenneTwisterError> {
        serde_yml::to_string(&self).map_err(|e| {
            MersenneTwisterError::SerializationError(e.to_string())
        })
    }

    /// Deserialize the configuration from YAML.
    ///
    /// # Arguments
    ///
    /// * `yaml` - A string slice containing the YAML data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized `MersenneTwisterConfig` instance or an error.
    ///
    /// # Errors
    ///
    /// Returns a `MersenneTwisterError::SerializationError` if deserialization fails.
    #[cfg(feature = "yaml")]
    pub fn deserialize_from_yaml(
        yaml: &str,
    ) -> Result<Self, MersenneTwisterError> {
        serde_yml::from_str(yaml).map_err(|e| {
            MersenneTwisterError::SerializationError(e.to_string())
        })
    }

    /// Serialize the configuration to TOML.
    ///
    /// # Returns
    ///
    /// A `Result` containing the TOML string or an error.
    ///
    /// # Errors
    ///
    /// Returns a `MersenneTwisterError::SerializationError` if serialization fails.
    #[cfg(feature = "toml")]
    pub fn serialize_to_toml(
        &self,
    ) -> Result<String, MersenneTwisterError> {
        toml::to_string(&self).map_err(|e| {
            MersenneTwisterError::SerializationError(e.to_string())
        })
    }

    /// Deserialize the configuration from TOML.
    ///
    /// # Arguments
    ///
    /// * `toml` - A string slice containing the TOML data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized `MersenneTwisterConfig` instance or an error.
    ///
    /// # Errors
    ///
    /// Returns a `MersenneTwisterError::SerializationError` if deserialization fails.
    #[cfg(feature = "toml")]
    pub fn deserialize_from_toml(
        toml: &str,
    ) -> Result<Self, MersenneTwisterError> {
        toml::from_str(toml).map_err(|e| {
            MersenneTwisterError::SerializationError(e.to_string())
        })
    }
}

impl Default for MersenneTwisterConfig<624, 397> {
    fn default() -> Self {
        MersenneTwisterConfig::new().unwrap()
    }
}

impl<const N: usize, const M: usize> fmt::Display
    for MersenneTwisterConfig<N, M>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MersenneTwisterConfig {{ params: MersenneTwisterParams {{ matrix_a: 0x{:08x}, upper_mask: 0x{:08x}, lower_mask: 0x{:08x}, tempering_mask_b: 0x{:08x}, tempering_mask_c: 0x{:08x} }} }}",
            self.params.matrix_a,
            self.params.upper_mask,
            self.params.lower_mask,
            self.params.tempering_mask_b,
            self.params.tempering_mask_c
        )
    }
}

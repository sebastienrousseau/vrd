// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Mersenne Twister (MT19937) configuration types.

use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Errors produced by [`MersenneTwisterConfig`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum MersenneTwisterError {
    /// A configuration parameter was outside its valid range.
    InvalidConfig(&'static str),
}

impl fmt::Display for MersenneTwisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MersenneTwisterError::InvalidConfig(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MersenneTwisterError {}

/// Parameter values for the Mersenne Twister algorithm.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MersenneTwisterParams {
    /// Constant whose highest bit must be set.
    pub matrix_a: u32,
    /// Upper-bit mask.
    pub upper_mask: u32,
    /// Lower-bit mask.
    pub lower_mask: u32,
    /// Tempering mask B.
    pub tempering_mask_b: u32,
    /// Tempering mask C.
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

/// Configuration for an MT19937-style Mersenne Twister.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MersenneTwisterConfig<const N: usize, const M: usize> {
    /// The validated configuration parameters.
    pub params: MersenneTwisterParams,
}

impl<const N: usize, const M: usize> MersenneTwisterConfig<N, M> {
    /// Builds a config with custom parameters.
    pub fn new_custom(
        params: MersenneTwisterParams,
    ) -> Result<Self, MersenneTwisterError> {
        Self::validate(&params)?;
        Ok(MersenneTwisterConfig { params })
    }

    /// Validates `params`.
    pub fn validate(
        params: &MersenneTwisterParams,
    ) -> Result<(), MersenneTwisterError> {
        if N < 1 {
            return Err(MersenneTwisterError::InvalidConfig(
                "N must be at least 1",
            ));
        }
        if M < 1 || M >= N {
            return Err(MersenneTwisterError::InvalidConfig(
                "M must be at least 1 and less than N",
            ));
        }
        if params.matrix_a & 0x80000000 != 0x80000000 {
            return Err(MersenneTwisterError::InvalidConfig(
                "matrix_a must have its highest bit set",
            ));
        }
        if params.upper_mask != 0x80000000 {
            return Err(MersenneTwisterError::InvalidConfig(
                "upper_mask must be 0x80000000",
            ));
        }
        if params.lower_mask != 0x7fffffff {
            return Err(MersenneTwisterError::InvalidConfig(
                "lower_mask must be 0x7fffffff",
            ));
        }
        if params.tempering_mask_b != 0x9d2c5680 {
            return Err(MersenneTwisterError::InvalidConfig(
                "tempering_mask_b must be 0x9d2c5680",
            ));
        }
        if params.tempering_mask_c != 0xefc60000 {
            return Err(MersenneTwisterError::InvalidConfig(
                "tempering_mask_c must be 0xefc60000",
            ));
        }
        Ok(())
    }

    /// Builds a config using the canonical parameters.
    pub fn new() -> Result<Self, MersenneTwisterError> {
        Self::new_custom(MersenneTwisterParams::default())
    }

    /// Replaces the parameters.
    pub fn set_config(
        &mut self,
        params: MersenneTwisterParams,
    ) -> Result<(), MersenneTwisterError> {
        Self::validate(&params)?;
        self.params = params;
        Ok(())
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
            "MersenneTwisterConfig {{ matrix_a: 0x{:08x}, upper_mask: 0x{:08x}, lower_mask: 0x{:08x}, tempering_mask_b: 0x{:08x}, tempering_mask_c: 0x{:08x} }}",
            self.params.matrix_a,
            self.params.upper_mask,
            self.params.lower_mask,
            self.params.tempering_mask_b,
            self.params.tempering_mask_c,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::format;
    #[cfg(all(not(feature = "alloc"), feature = "std"))]
    use std::format;

    #[test]
    fn test_exhaustive_mt_coverage() {
        let mut p = MersenneTwisterParams::default();

        // Error Display
        let err = MersenneTwisterError::InvalidConfig("foo");
        let _ = format!("{}", err);
        #[cfg(feature = "std")]
        {
            use std::error::Error;
            assert!(err.source().is_none());
        }

        // Params Debug
        let _ = format!("{:?}", p);

        // Config Display
        let c = MersenneTwisterConfig::<624, 397>::default();
        let _ = format!("{}", c);

        // Validation branches
        p.matrix_a = 0;
        assert!(
            MersenneTwisterConfig::<624, 397>::validate(&p).is_err()
        );
        p = MersenneTwisterParams::default();
        p.upper_mask = 0;
        assert!(
            MersenneTwisterConfig::<624, 397>::validate(&p).is_err()
        );
        p = MersenneTwisterParams::default();
        p.lower_mask = 0;
        assert!(
            MersenneTwisterConfig::<624, 397>::validate(&p).is_err()
        );
        p = MersenneTwisterParams::default();
        p.tempering_mask_b = 0;
        assert!(
            MersenneTwisterConfig::<624, 397>::validate(&p).is_err()
        );
        p = MersenneTwisterParams::default();
        p.tempering_mask_c = 0;
        assert!(
            MersenneTwisterConfig::<624, 397>::validate(&p).is_err()
        );

        // N, M bounds
        assert!(MersenneTwisterConfig::<0, 0>::validate(&p).is_err());
        assert!(MersenneTwisterConfig::<10, 0>::validate(&p).is_err());
        assert!(MersenneTwisterConfig::<10, 10>::validate(&p).is_err());

        // set_config
        let mut cfg = MersenneTwisterConfig::<624, 397>::default();
        assert!(cfg
            .set_config(MersenneTwisterParams::default())
            .is_ok());
    }
}

// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `FloatExt` impl backed by `libm` for `no_std` builds. Excluded
//! from `cargo tarpaulin` measurement because `cargo test` always
//! runs with `std` and never invokes these bodies. The pure no_std
//! build is validated by the `no_std (thumbv7em-none-eabihf)` CI job.

use crate::FloatExt;

impl FloatExt for f64 {
    #[inline]
    fn ln(self) -> Self {
        libm::log(self)
    }
    #[inline]
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }
    #[inline]
    fn cos(self) -> Self {
        libm::cos(self)
    }
    #[inline]
    fn exp(self) -> Self {
        libm::exp(self)
    }
}

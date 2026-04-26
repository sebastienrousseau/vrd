// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Inspecting and customizing `MersenneTwisterConfig`.
//!
//! Run: `cargo run --example config`

#[path = "support.rs"]
mod support;

use vrd::{MersenneTwisterConfig, MersenneTwisterParams};

fn main() {
    support::header("vrd -- config");

    support::task_with_output("Default config = canonical MT19937", || {
        let cfg = MersenneTwisterConfig::<624, 397>::default();
        vec![
            format!("matrix_a         = 0x{:08x}", cfg.params.matrix_a),
            format!("upper_mask       = 0x{:08x}", cfg.params.upper_mask),
            format!("lower_mask       = 0x{:08x}", cfg.params.lower_mask),
            format!(
                "tempering_mask_b = 0x{:08x}",
                cfg.params.tempering_mask_b
            ),
            format!(
                "tempering_mask_c = 0x{:08x}",
                cfg.params.tempering_mask_c
            ),
        ]
    });

    support::task_with_output("Validation rejects non-canonical params", || {
        let params = MersenneTwisterParams {
            matrix_a: 0x12345678, // highest bit not set -> reject
            ..MersenneTwisterParams::default()
        };
        let result =
            MersenneTwisterConfig::<624, 397>::validate(&params);
        vec![format!("validate(bad matrix_a) = {result:?}")]
    });

    support::task_with_output("Display impl summarises the params", || {
        let cfg = MersenneTwisterConfig::<624, 397>::default();
        vec![format!("{cfg}")]
    });

    support::summary(3);
}

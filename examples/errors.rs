// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types: `VrdError` and `MersenneTwisterError`.
//!
//! Run: `cargo run --example errors`

#[path = "support.rs"]
mod support;

use vrd::{
    MersenneTwisterConfig, MersenneTwisterError, MersenneTwisterParams,
    VrdError,
};

fn maybe_fail(flag: bool) -> Result<u32, VrdError> {
    if flag {
        Err(VrdError::GeneralError("simulated failure"))
    } else {
        Ok(42)
    }
}

fn main() {
    support::header("vrd -- errors");

    support::task_with_output("VrdError carries a &'static str (no_std-friendly)", || {
        let ok = maybe_fail(false);
        let err = maybe_fail(true);
        vec![
            format!("ok  = {ok:?}"),
            format!("err = {err:?}"),
            format!(
                "display = \"{}\"",
                err.unwrap_err()
            ),
        ]
    });

    support::task_with_output(
        "MersenneTwisterError is returned by validate()",
        || {
            // Invalid: M >= N
            let bad =
                MersenneTwisterConfig::<624, 700>::new();
            let msg = match bad {
                Err(MersenneTwisterError::InvalidConfig(m)) => m,
                _ => "BUG: should have failed",
            };
            vec![format!("InvalidConfig: \"{msg}\"")]
        },
    );

    support::task_with_output("Custom params still validate", || {
        let params = MersenneTwisterParams {
            // Toggle one constant to make it invalid.
            tempering_mask_b: 0xDEAD_BEEF,
            ..MersenneTwisterParams::default()
        };
        match MersenneTwisterConfig::<624, 397>::new_custom(params) {
            Err(MersenneTwisterError::InvalidConfig(m)) => {
                vec![format!("Caught: \"{m}\"")]
            }
            _ => vec!["BUG: should have failed".into()],
        }
    });

    support::summary(3);
}

// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! This is the main entry point for the vrd application.
fn main() {
    // Call the `run()` function from the `Random (VRD)` module.
    if let Err(err) = vrd::run() {
        eprintln!("Error running vrd: {}", err);
        std::process::exit(1);
    }
}

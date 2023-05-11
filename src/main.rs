// Copyright © 2023 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This is the main entry point for the vrd application.
fn main() {
    // Call the `run()` function from the `Random (VRD)` module.
    if let Err(err) = vrd::run() {
        eprintln!("Error running vrd: {}", err);
        std::process::exit(1);
    }
}

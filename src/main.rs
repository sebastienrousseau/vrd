// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

use dtt::DateTime;
use rlg::log_level::LogLevel;
use std::process;
use uuid::Uuid;
use vrd::{create_log_entry, log_entry_async};

fn main() {
    // Directly creating a new DateTime instance without matching against a Result
    let date = DateTime::new();

    // Directly generating a new UUID without matching against a Result
    let uuid = Uuid::new_v4().to_string();

    // Extracting ISO 8601 format from DateTime
    let iso = date.iso_8601;

    // Create a single Tokio runtime instance
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Failed to build Tokio runtime: {}", e);
            process::exit(1);
        });

    // Adjusted error handling for vrd::run()
    if let Err(run_error) = vrd::run() {
        let error_message =
            format!("Unexpected error running vrd: {:?}", run_error);
        let log_entry = create_log_entry(
            &uuid,
            &iso,
            LogLevel::ERROR,
            &error_message,
        );

        runtime.block_on(async { log_entry_async(log_entry).await });

        process::exit(1);
    }
}

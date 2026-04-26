// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Run every other vrd example in sequence.
//!
//! Usage: `cargo run --example all`

use std::process::Command;
use std::time::Instant;

const EXAMPLES: &[&str] = &[
    // Core
    "hello",
    "basics",
    "seed",
    "bytes",
    "floats",
    "bools",
    "chars",
    "strings",
    // Bounded sampling
    "bounded",
    "unbiased",
    // Distributions
    "normal",
    "exponential",
    "poisson",
    // Slice operations
    "choose",
    "shuffle",
    "sample",
    "slice",
    "weighted",
    // Backends
    "xoshiro",
    "mersenne",
    "backends",
    // Traits + macros
    "traits",
    "seedable",
    "macros",
    // Applications
    "dice",
    "lottery",
    "passwords",
    "monte",
    // Errors / config / portability
    "errors",
    "config",
    "nostd",
];

fn main() {
    println!("\n  \x1b[1mvrd examples\x1b[0m\n");

    let start = Instant::now();
    let mut passed = 0usize;
    let mut failed = 0usize;

    for name in EXAMPLES {
        print!("  \x1b[90m{name:<14}\x1b[0m");

        let result = Command::new("cargo")
            .args(["run", "--example", name, "--quiet"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();

        match result {
            Ok(status) if status.success() => {
                println!("\x1b[32mdone\x1b[0m");
                passed += 1;
            }
            _ => {
                println!("\x1b[31mfail\x1b[0m");
                failed += 1;
            }
        }
    }

    let elapsed = start.elapsed();
    println!();
    if failed == 0 {
        println!(
            "  \x1b[1;32m{passed} examples passed\x1b[0m \x1b[90m({:.1}s)\x1b[0m\n",
            elapsed.as_secs_f64()
        );
    } else {
        println!(
            "  \x1b[1;31m{failed} failed\x1b[0m, {passed} passed \x1b[90m({:.1}s)\x1b[0m\n",
            elapsed.as_secs_f64()
        );
        std::process::exit(1);
    }
}

// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Statistical validation harness.
//!
//! Pipes the output of every available vrd backend into the
//! external [PractRand](https://pracrand.sourceforge.net) `RNG_test`
//! binary and emits a markdown pass-count summary. Mirror of what
//! sits in `BENCHMARKS.md`.
//!
//! Usage:
//!
//! ```bash
//! # 1. Install PractRand (one-time) and put `RNG_test` in PATH.
//! # 2. Run:
//! cargo run --release --example crush --features crush
//! ```
//!
//! Default test budget is 256 MiB per backend (~30 s on a modern
//! laptop); override with `VRD_CRUSH_BYTES=<MiB>`.
//!
//! Informational only — not a CI gate.

#![cfg(feature = "crush")]

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

use vrd::Random;

fn main() {
    let mib: usize = env::var("VRD_CRUSH_BYTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(256);
    let total_bytes = mib * 1024 * 1024;

    println!("\n  \x1b[1mvrd statistical validation (PractRand)\x1b[0m");
    println!(
        "  Budget per backend: {} MiB. Override with VRD_CRUSH_BYTES.\n",
        mib
    );

    // Probe for the binary up front so we fail loudly rather than
    // silently hanging on the pipe.
    if Command::new("RNG_test")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        eprintln!(
            "\x1b[31merror\x1b[0m: `RNG_test` not found in PATH. \
             Install PractRand (>= 0.94) and re-run.\n\
             See CONTRIBUTING.md for install pointers."
        );
        std::process::exit(2);
    }

    type BackendFactory = Box<dyn FnMut() -> Random>;
    let mut backends: Vec<(&str, BackendFactory)> = Vec::new();
    backends.push((
        "Xoshiro256++",
        Box::new(|| Random::from_u64_seed(1)),
    ));
    backends.push((
        "MersenneTwister",
        Box::new(|| Random::new_mersenne_twister_with_seed(1)),
    ));
    #[cfg(feature = "pcg")]
    {
        backends.push((
            "PCG32",
            Box::new(|| Random::new_pcg32_with_seed(1)),
        ));
        backends.push((
            "PCG64",
            Box::new(|| Random::new_pcg64_with_seed(1)),
        ));
    }
    #[cfg(feature = "crypto")]
    {
        backends.push((
            "ChaCha20",
            Box::new(|| Random::from_secure_seed([1u8; 32])),
        ));
    }

    // Markdown header for the final summary.
    println!(
        "| Backend | Tests run | Anomalies | Failures | Verdict |"
    );
    println!(
        "| :--- | ---: | ---: | ---: | :---: |"
    );

    for (name, make_rng) in backends.iter_mut() {
        let mut rng = make_rng();
        let started = Instant::now();
        let report = run_practrand(&mut rng, total_bytes);
        let elapsed = started.elapsed();
        let (n_tests, anomalies, failures) = parse_summary(&report);
        let verdict = if failures > 0 {
            "❌ FAIL"
        } else if anomalies > 0 {
            "⚠ ANOM"
        } else {
            "✅ PASS"
        };
        println!(
            "| {name} | {n_tests} | {anomalies} | {failures} | {verdict} | \
             \x1b[90m({:.1}s)\x1b[0m",
            elapsed.as_secs_f64()
        );
    }

    println!();
}

#[cfg(feature = "crush")]
fn run_practrand(rng: &mut Random, total_bytes: usize) -> String {
    use rand::rand_core::TryRng;

    let mut child = Command::new("RNG_test")
        .args([
            "stdin64",
            "-tlmin",
            "256KB",
            "-tlmax",
            "1GB",
            "-multithreaded",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("RNG_test spawn");

    let mut stdin = child.stdin.take().expect("child stdin");
    let mut buf = [0u8; 64 * 1024];
    let mut written = 0usize;
    while written < total_bytes {
        let _ = rng.try_fill_bytes(&mut buf);
        if stdin.write_all(&buf).is_err() {
            break;
        }
        written += buf.len();
    }
    drop(stdin);

    let output = child.wait_with_output().expect("child wait");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[cfg(feature = "crush")]
fn parse_summary(report: &str) -> (u32, u32, u32) {
    let mut n_tests = 0u32;
    let mut anomalies = 0u32;
    let mut failures = 0u32;
    for line in report.lines() {
        // PractRand emits lines like:
        //   ...        pass (...)
        //   ...        unusual          ("anomaly", borderline)
        //   ...        suspicious       (anomaly, more severe)
        //   ...        FAIL             (test failed)
        if line.contains("Test Name") || line.contains("Raw:") {
            continue;
        }
        if line.contains("FAIL") {
            failures += 1;
            n_tests += 1;
        } else if line.contains("unusual")
            || line.contains("suspicious")
            || line.contains("VERY SUSPICIOUS")
        {
            anomalies += 1;
            n_tests += 1;
        } else if line.contains("normal") || line.contains("pass") {
            n_tests += 1;
        }
    }
    (n_tests, anomalies, failures)
}

// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! Benchmarks using the `criterion` crate.
//!
//! This file contains benchmarks that use the `criterion` crate for performance testing.
//! It imports the `Random` trait from the `vrd` crate and uses the `criterion` macros and types.
//! The benchmarks showcase the performance of various functions in the `Random` trait.
//!
//! # Example
//!
//! ```
//! extern crate criterion;
//! extern crate vrd;
//!
//! use self::vrd::Random;
//! use criterion::{black_box, criterion_group, criterion_main, Criterion};
//!
//! /// Benchmarks the random number generation functions provided by the `Random` trait.
//! ///
//! /// This function measures the performance of various functions in the `Random` trait,
//! /// such as `bool`, `bytes`, `char`, `choose`, `float`, `int`, `new`, `pseudo`,
//! /// `rand`, and `range`.
//! fn benchmark_random(c: &mut Criterion) {
//!     // Benchmark implementation goes here
//! }
//!
//! criterion_group!(benches, benchmark_random);
//! criterion_main!(benches);
//! ```
#![allow(missing_docs)]
use self::vrd::random::Random;
use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use vrd;

/// Benchmarks the random number generation functions provided by the `Random` trait.
///
/// This function measures the performance of various functions in the `Random` trait,
/// such as `bool`, `bytes`, `char`, `choose`, `float`, `int`, `new`, `pseudo`,
/// `rand`, and `range`.
///
/// # Arguments
///
/// * `c` - A mutable reference to the `Criterion` struct used for benchmarking.
///
fn benchmark_random(c: &mut Criterion) {
    // Benchmark the random bool function
    c.bench_function("Random bool", |b| {
        b.iter(|| Random::bool(&mut Random::new(), black_box(0.5)))
    });

    // Benchmark the random bytes function
    c.bench_function("Random bytes", |b| {
        b.iter(|| Random::bytes(&mut Random::new(), black_box(1000)))
    });

    // Benchmark the random char function
    c.bench_function("Random char", |b| {
        b.iter(|| Random::char(&mut Random::new()))
    });

    // Benchmark the random choose function
    c.bench_function("Random choose", |b| {
        b.iter(|| {
            let mut rng = Random::new();
            let values = vec![1, 2, 3, 4, 5];
            Random::choose(&mut rng, &values);
        })
    });

    // Benchmark the random float function
    c.bench_function("Random float", |b| {
        b.iter(|| Random::float(&mut Random::new()))
    });

    // Benchmark the random int function
    c.bench_function("Random int", |b| {
        b.iter(|| {
            Random::int(
                &mut Random::new(),
                black_box(0),
                black_box(100),
            )
        })
    });

    // Benchmark the random new function
    c.bench_function("Random new", |b| b.iter(|| Random::new));

    // Benchmark the random pseudo function
    c.bench_function("Random pseudo", |b| {
        let mut rng = Random::new();
        b.iter(|| rng.pseudo())
    });

    // Benchmark the random function
    c.bench_function("Random random", |b| {
        let mut rng = Random::new();
        b.iter(|| rng.rand())
    });

    // Benchmark the random range function
    c.bench_function("Random range", |b| {
        let mut rng = Random::new();
        b.iter(|| rng.range(black_box(0), black_box(100)))
    });
}

// Groups the benchmarks and runs them using the `criterion_group` macro.
criterion_group!(benches, benchmark_random);
criterion_main!(benches);

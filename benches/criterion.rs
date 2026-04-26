// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(missing_docs)]
//! Comparative benchmarks for the `vrd` random number generators.
//!
//! Compares:
//! - `vrd::Random` on Xoshiro256++ (the default)
//! - `vrd::Random` on Mersenne Twister
//! - `fastrand` (top-level helpers)
//! - `rand` (`rand::rng()` thread-local)
//!
//! Run with `cargo bench`.

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use rand::rand_core::Rng;
use vrd::xoshiro::Xoshiro256PlusPlus;
use vrd::Random;

/// `u32` generation throughput across the four backends.
fn bench_rng_u32(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32");

    group.bench_function("vrd::Random / Xoshiro256++ (default)", |b| {
        let mut rng = Random::new();
        b.iter(|| black_box(rng.rand()));
    });

    group.bench_function("vrd::Random / MersenneTwister", |b| {
        let mut rng = Random::new_mersenne_twister();
        b.iter(|| black_box(rng.rand()));
    });

    group.bench_function("fastrand", |b| {
        b.iter(|| black_box(fastrand::u32(..)));
    });

    group.bench_function("rand::rng()", |b| {
        b.iter(|| black_box(rand::rng().next_u32()));
    });

    group.finish();
}

/// `u64` generation — exercises Xoshiro's native 64-bit path versus the
/// MT path that concatenates two `u32`s.
fn bench_rng_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64");

    group.bench_function("vrd::Random / Xoshiro256++ (default)", |b| {
        let mut rng = Random::new();
        b.iter(|| black_box(rng.u64()));
    });

    group.bench_function("vrd::Random / MersenneTwister", |b| {
        let mut rng = Random::new_mersenne_twister();
        b.iter(|| black_box(rng.u64()));
    });

    group.bench_function("fastrand", |b| {
        b.iter(|| black_box(fastrand::u64(..)));
    });

    group.bench_function("rand::rng()", |b| {
        b.iter(|| black_box(rand::rng().next_u64()));
    });

    group.finish();
}

/// 1 KiB byte-fill — exercises the bulk byte generation path.
fn bench_fill_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill_1024_bytes");

    group.bench_function("vrd::Random / Xoshiro256++ (default)", |b| {
        let mut rng = Random::new();
        let mut buf = [0u8; 1024];
        b.iter(|| {
            use rand::rand_core::TryRng;
            let _ = rng.try_fill_bytes(black_box(&mut buf));
        });
    });

    group.finish();
}

/// Distribution sampling cost for the statistical helpers.
fn bench_distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("distributions");
    let mut rng = Random::new();

    group.bench_function("normal(0, 1)", |b| {
        b.iter(|| black_box(rng.normal(0.0, 1.0)));
    });

    group.bench_function("exponential(1)", |b| {
        b.iter(|| black_box(rng.exponential(1.0)));
    });

    group.bench_function("poisson(3)", |b| {
        b.iter(|| black_box(rng.poisson(3.0)));
    });

    group.finish();
}

/// Pins the `Random` enum-dispatch overhead against raw
/// `Xoshiro256PlusPlus`. Both columns should land within criterion noise
/// — if a refactor regresses this group, the wrapper has stopped being
/// inlined.
fn bench_wrapper_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("wrapper_overhead");

    group.bench_function("u32 raw Xoshiro256PlusPlus", |b| {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(1);
        b.iter(|| black_box(rng.next_u32()));
    });

    group.bench_function("u32 vrd::Random (Xoshiro)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.rand()));
    });

    group.bench_function("u64 raw Xoshiro256PlusPlus", |b| {
        let mut rng = Xoshiro256PlusPlus::from_u64_seed(1);
        b.iter(|| black_box(rng.next_u64()));
    });

    group.bench_function("u64 vrd::Random (Xoshiro)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.u64()));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_rng_u32,
    bench_rng_u64,
    bench_fill_bytes,
    bench_distributions,
    bench_wrapper_overhead,
);
criterion_main!(benches);

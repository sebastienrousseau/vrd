// Copyright © 2023-2026 vrd. All rights reserved.
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

/// Bounded integer sampling — the foundation of `int`/`uint`/
/// `random_range`/`range`/`choose`/`shuffle`/`sample`. All routes
/// land here, so a regression on `bounded` shows up everywhere.
fn bench_bounded_sampling(c: &mut Criterion) {
    let mut group = c.benchmark_group("bounded_sampling");

    group.bench_function("bounded(7) — Lemire rejection ~hot", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.bounded(7)));
    });

    group.bench_function("bounded(1024) — power of two", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.bounded(1024)));
    });

    group.bench_function("int(-1000, 1000)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.int(-1000, 1000)));
    });

    group.bench_function("uint(0, 1000)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.uint(0, 1000)));
    });

    group.bench_function("random_range(0, 1000)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.random_range(0, 1000)));
    });

    group.finish();
}

/// Slice operations on the fixed allocation-free path: `choose`,
/// `shuffle`, `sample`, `sample_with_replacement`, `rand_slice`.
fn bench_slice_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("slice_ops");

    let pool: Vec<u32> = (0..256).collect();

    group.bench_function("choose (256 elements)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.choose(&pool)));
    });

    group.bench_function("shuffle (32 elements)", |b| {
        let mut rng = Random::from_u64_seed(1);
        let mut deck: Vec<u32> = (0..32).collect();
        b.iter(|| {
            rng.shuffle(black_box(&mut deck));
        });
    });

    group.bench_function("sample 8 from 256 (no replacement)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.sample(&pool, 8)));
    });

    group.bench_function("sample_with_replacement 8 from 256", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.sample_with_replacement(&pool, 8)));
    });

    group.bench_function("rand_slice (window of 16 from 256)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.rand_slice(&pool, 16)));
    });

    group.finish();
}

/// Float, bool, and char output. Confirms the bit-precise scaling
/// (24 mantissa bits on `f32`, 53 on `f64`) doesn't add measurable
/// overhead vs. the underlying `u32`/`u64` draw.
fn bench_scalar_misc(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar_misc");

    group.bench_function("float() — f32 in [0, 1)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.float()));
    });

    group.bench_function("double() — f64 in [0, 1)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.double()));
    });

    group.bench_function("bool(0.5)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.bool(0.5)));
    });

    group.bench_function("char()", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.char()));
    });

    group.bench_function("string(16)", |b| {
        let mut rng = Random::from_u64_seed(1);
        b.iter(|| black_box(rng.string(16)));
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
    bench_bounded_sampling,
    bench_slice_ops,
    bench_scalar_misc,
);
criterion_main!(benches);

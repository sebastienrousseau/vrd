<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Migrating from `fastrand` 2.x to `vrd`

`fastrand` is the speed-first option: a single-`u64`-state
LCG (Wyrand) with a thread-local globally-shared default.
Migrating to vrd costs you the sub-nanosecond inline path but
buys you backend choice, output stability across patches,
SIMD bulk-byte support, quasi-random sequences, distributions,
and UUIDs/tokens in a single crate.

## Quick translation

| `fastrand` 2.x | `vrd` equivalent | Notes |
| :-- | :-- | :-- |
| `fastrand::u32(..)` | `Random::new().rand()` (or reuse a `Random`) | vrd doesn't ship a thread-local default |
| `fastrand::u64(..)` | `rng.u64()` | |
| `fastrand::f32()` | `rng.float()` | both `[0, 1)` |
| `fastrand::f64()` | `rng.double()` | |
| `fastrand::bool()` | `rng.bool(0.5)` | vrd takes the probability |
| `fastrand::usize(0..n)` | `rng.uint(0, (n - 1) as u32) as usize` | |
| `fastrand::i32(lo..=hi)` | `rng.int(lo, hi)` | both unbiased |
| `fastrand::char(..)` | `rng.char()` | |
| `fastrand::shuffle(&mut v)` | `rng.shuffle(&mut v)` | `alloc` |
| `fastrand::choice(slice)` | `rng.choose(slice)` | `Option<&T>` |
| `fastrand::seed(s)` | `let mut rng = Random::from_u64_seed(s);` | |
| `fastrand::Rng::with_seed(s)` | `Random::from_u64_seed(s)` | |

## What you give up

- **Sub-nanosecond `u32` / `u64`**. fastrand's inline LCG
  call is 0.6 ns; vrd's facade dispatch is 3.1 ns (Xoshiro)
  or 2.7 ns (PCG32). For workloads dominated by single-`u32`
  draws, fastrand is genuinely faster.
- **Thread-local global default**. `fastrand::u32(..)` reaches
  for a thread-local RNG with no `let mut rng = …` ceremony.
  vrd requires you to hold the `Random` (matches the
  `rand` discipline; eases reasoning about determinism).

## What you gain

- **Output stability across patches**. fastrand changes its
  algorithm freely; vrd commits to bit-exact output for a
  given seed across patch releases.
- **Multiple backends**. PCG32 / PCG64 / MT19937 / ChaCha20
  via feature flags, all behind the same `Random` facade.
- **Crypto-quality option**. fastrand explicitly is not a
  CSPRNG; vrd's `crypto` feature gives you ChaCha20 in the
  same crate.
- **SIMD bulk-byte path**. `--features simd` is 2–3× faster
  than scalar for large `fill_bytes` calls.
- **Quasi-random**. Halton / Sobol / Van der Corput under
  `--features quasirandom` for Monte Carlo workloads.
- **Distributions**. Normal, Exponential, Uniform, Poisson
  + pluggable `Distribution<T>` trait. fastrand has none.
- **UUIDs and tokens**. `uuid_v4`, `hex_token`, `base64_token`
  inline - no separate `uuid`/`hex`/`base64` deps.
- **`no_std` with explicit `alloc` gating**. fastrand requires
  `std` for its default thread-local; vrd's pure no_std path
  runs on Cortex-M and WebAssembly.

## When to stay on fastrand

- Your hot path is a single `u32` per loop iteration and you've
  measured the trait-dispatch overhead as material to your
  application.
- Binary size dominates everything else (fastrand is ~3 KB
  compiled; vrd's library + default backend is ~30 KB).
- You're using only `u32` / `u64` / `bool` / `shuffle` and
  nothing vrd uniquely ships justifies the migration.

## When to migrate

- You want output-stability commitments for golden-file tests.
- You're already pulling in `uuid` or `hex` or `base64`
  alongside `fastrand` - vrd consolidates the three.
- You need crypto-grade output somewhere in the codebase.
- You're doing bulk byte fills and the SIMD win is meaningful.
- You need quasi-random for Monte Carlo / financial / graphics.
- You need distributions beyond `f64` uniform.

## Example: a typical fastrand block

```rust
// fastrand:
fastrand::seed(42);
let n = fastrand::u32(..);
let r = fastrand::i32(1..=100);
let f = fastrand::f64();

// vrd:
let mut rng = vrd::Random::from_u64_seed(42);
let n = rng.rand();
let r = rng.int(1, 100);
let f = rng.double();
```

The line count is similar; vrd's version is explicit about
where the RNG lives (which makes thread-safety reasoning
straightforward - see [`POLICIES.md`](POLICIES.md)).

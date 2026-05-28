<!-- SPDX-FileCopyrightText: 2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# vrd v0.0.11 — Release Notes

A double-barrel release: the v0.0.10 performance-audit follow-ups
(#88, #89) **plus** the entire v0.1.0 differentiator backlog
(#84, #90, #91, #92, #93, #94, #95). After this lands, vrd is no
longer "yet another non-crypto PRNG with a nice API" — it's a
**tri-backend (Xoshiro / MT / PCG) + crypto-backend (ChaCha20) +
quasi-random (Halton / Sobol / Van der Corput)** RNG with a SIMD
bulk-byte path and a pluggable `Distribution` trait.

## Headline numbers

- **`Random::normal()` is 4.3× faster** — 14.66 ns → 3.71 ns per
  sample via a 256-strip Ziggurat sampler with build-time table
  generation (#89). Apple Silicon M-series.
- **`Random::try_fill_bytes` is 2.2× faster at 1 KiB and 3.0×
  faster at 16 KiB** under the new `simd` feature (#88) — NEON on
  AArch64, AVX2 on x86_64; ~16.5 GB/s and ~22.6 GB/s respectively.
- **PCG32 = 2.72 ns/u32** through the `Random` facade, faster
  than Xoshiro256++ (3.09 ns) and the `rand::rng()` baseline
  (4.05 ns).
- **100.00 % line coverage** (773 / 773 lines on every measured
  file). Codecov gate passes on both `codecov/patch` and
  `codecov/project`.
- **9 issues closed** in this release; 0 left open in the
  v0.1.0 milestone.
- **122 tests + 122 doc-tests pass** under `cargo test
  --all-features`. CI matrix: default / pcg / crypto /
  quasirandom / simd / all-features, on x86_64 and AArch64,
  plus `wasm32-unknown-unknown` and `thumbv7em-none-eabihf`.

## What ships

### Performance (carried over from v0.0.10's perf audit)

- **#88 — `simd` feature**: SIMD-batched `fill_bytes` holding K
  independent Xoshiro256++ states in vector registers (K = 2 on
  AArch64 NEON, K = 4 on x86_64 AVX2 — built from two
  interleaved 2-lane groups for ILP). Lanes are derived from a
  SplitMix64 whitening of the scalar state (collision probability
  ≤ K²/2²⁵⁶). The contract is documented: **same seed produces a
  different byte stream** under `simd` vs. scalar — fundamental
  to parallelising. Buffers under 64 B fall back to scalar.
- **#89 — Ziggurat `normal()`**: 256-strip Marsaglia & Tsang
  (2000) sampler with `build.rs`-generated `K`/`W`/`F` tables.
  Fast path is one `u32` draw, one table lookup, one `f64`
  multiply. Overhang (~1%) adds one `exp` and one `f64` draw.
  Tail (~0.03%) uses exponential rejection. A golden vector
  pins 16 bit-exact samples and a moments test verifies
  skewness < 0.05 / kurtosis < 0.1 over 200 000 samples.

### v0.1.0 differentiators

- **#94 — `Random::fill_array<const N>()`**: const-generic,
  allocation-free bulk byte gen returning `[u8; N]` on the
  stack. Works in pure `no_std`.
- **#92 — `Random::split()`**: parallel-safe stream derivation
  via Xoshiro256++ `jump()`. Returns `Some(Random)` on the
  Xoshiro backend (2¹²⁸-step separation guarantee), `None` on
  Mersenne Twister / PCG / ChaCha20 (no analogous
  fixed-distance jump).
- **#95 — PCG backends** under a new `pcg` feature:
  `Pcg32` (PCG-XSH-RR-64/32, 16 B state, 32-bit output) and
  `Pcg64` (PCG-XSL-RR-128/64, 32 B state, native 64-bit
  output). Hand-rolled per O'Neill 2014; no new external
  dependencies. Constructors:
  `Random::new_pcg32` / `new_pcg32_with_seed` /
  `new_pcg64` / `new_pcg64_with_seed`.
- **#90 — ChaCha20 CSPRNG backend** under a new `crypto`
  feature: `Random::new_secure()` (OS-entropy seeded) and
  `Random::from_secure_seed([u8; 32])` (deterministic), backed
  by `rand_chacha::ChaCha20Rng`. **Bit-for-bit equivalent** to
  `rand_chacha::ChaCha20Rng::from_seed()` — drop-in for callers
  already on `rand_chacha`. Replaces the prior "Not a CSPRNG"
  callout in the README with a four-row "Choosing a backend"
  table.
- **#91 — Quasi-random sequences** under a new `quasirandom`
  feature: `HaltonSequence` (up to 32 dimensions via the first
  32 primes), `SobolSequence` (up to 6 dimensions via
  Bratley-Fox 1988 direction numbers), `VanDerCorputSequence`
  (any prime base). Variance scales `O((log n)^d / n)` for
  Monte Carlo integration, ray-tracing, finance. No competitor
  in vrd's weight class ships these.
- **#84 — `Distribution<T>` trait**: pluggable distribution
  sampling. Built-in `Normal`, `Exponential`, `Uniform`,
  `Poisson` impls forward to the optimised methods on `Random`;
  users add their own via `impl Distribution<MyType> for
  MyDist`. Closes the "Custom distribution support" item.
- **#93 — PractRand validation harness** behind a `crush`
  feature: `examples/crush.rs` pipes every backend through an
  external PractRand `RNG_test` binary and emits a markdown
  pass-count summary. New `BENCHMARKS.md` tracks per-release
  results. Informational only — CI does **not** gate on it.

### Coverage

100 % line coverage across every measured file:

| File | Lines |
|---|---:|
| `src/chacha.rs` | 22 / 22 |
| `src/distribution.rs` | 11 / 11 |
| `src/lib.rs` | 4 / 4 |
| `src/main.rs` | 25 / 25 |
| `src/mersenne_twister.rs` | 42 / 42 |
| `src/pcg.rs` | 88 / 88 |
| `src/quasirandom.rs` | 91 / 91 |
| `src/random.rs` | 375 / 375 |
| `src/xoshiro.rs` | 87 / 87 |
| `src/ziggurat.rs` | 28 / 28 |
| **Total** | **773 / 773** |

Two files are excluded from tarpaulin via `.tarpaulin.toml`:

- `src/xoshiro_simd.rs` — architecture-conditional (NEON on
  AArch64, AVX2 on x86_64). A single-platform tarpaulin run
  cannot observe both halves; validated by the `simd` CI matrix
  job that runs on both `ubuntu-latest` and `macos-latest`.
- `src/float_libm.rs` — the no_std `FloatExt` impl. `cargo
  test` always runs with `std`, so these libm-backed bodies
  never execute under tarpaulin. Validated by the
  `no_std (thumbv7em-none-eabihf)` CI job.

## Breaking changes

- `Random` and `RngBackend` no longer derive `Eq`, `Hash`, `Ord`,
  or `PartialOrd`. `PartialEq` is kept for snapshot / determinism
  comparisons. The drop is needed because `ChaCha20Rng` (the new
  `crypto` backend's underlying type) doesn't implement them,
  and "ordering RNG states" was never a meaningful operation.
- `#![forbid(unsafe_code)]` becomes `#![deny(unsafe_code)]` at
  the crate root so the optional `xoshiro_simd` module can lift
  the deny locally for architecture intrinsics. **All other
  modules remain `unsafe`-free.**

## Documentation

- [`README.md`](README.md) — install, quick start, choosing a
  backend, feature flags, examples, FAQ.
- [`CHANGELOG.md`](CHANGELOG.md) — full per-version diff.
- [`BENCHMARKS.md`](BENCHMARKS.md) — performance evidence with
  per-bench commands to reproduce.
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — pre-submit checklist,
  PractRand install pointers for the optional statistical
  validation harness.
- Issue tracker:
  https://github.com/sebastienrousseau/vrd/issues —
  closed-this-release: #84, #88, #89, #90, #91, #92, #93, #94,
  #95.

## Verification

This is a library-only release on crates.io; no signed binaries
ship yet. Future v0.1.x releases will introduce a `pkg/`
directory with cosign keyless signing on per-platform artefacts
(homebrew / docker / nix to start). See the v0.1.0 roadmap in
[`PLAN.md`](PLAN.md) (coming in the next release).

## What's next

The repo-template alignment with `noyalib` (Makefile, `doc/`,
`fuzz/`, `pkg/` subset, governance files) is the immediate
follow-up — tracked under `feat/repo-template`. After that, the
v0.0.12 milestone will collect any post-launch bug reports
plus the SIMD AVX2 path's 4-lane micro-optimisation that the
v0.0.11 numbers leave on the table.

---

vrd ᛫ Sebastien Rousseau ᛫ <https://vrdlib.com>

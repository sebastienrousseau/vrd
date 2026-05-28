<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# vrd architecture

A reference for what's in the box, why, and where each design
decision is paid for. Read top-to-bottom if you're contributing;
skim the section that matches your question otherwise.

## Layers

```
┌───────────────────────────────────────────────────────────┐
│  Random - facade                                          │
│   • dispatches over RngBackend via match                  │
│   • implements rand 0.10 TryRng + SeedableRng             │
│   • exposes 50+ ergonomic methods (rand, u64, int, ...)   │
├───────────────────────────────────────────────────────────┤
│  RngBackend - enum                                        │
│   ├─ Xoshiro256PlusPlus  (32 B inline, default)           │
│   ├─ MersenneTwister     (Box, alloc-gated)               │
│   ├─ Pcg32 / Pcg64       (inline, `pcg` feature)          │
│   └─ ChaCha20            (Box, `crypto` feature)          │
├───────────────────────────────────────────────────────────┤
│  Per-backend modules - pure-function generators           │
│   • xoshiro::Xoshiro256PlusPlus    (src/xoshiro.rs)       │
│   • mersenne_twister::MersenneTwister                     │
│   • pcg::{Pcg32, Pcg64}                                   │
│   • chacha::ChaChaRng              (wraps rand_chacha)    │
├───────────────────────────────────────────────────────────┤
│  Specialised samplers                                     │
│   • ziggurat::sample_normal()  (build.rs tables)          │
│   • xoshiro_simd::fill_bytes()  (`simd` feature; NEON/AVX2)│
│   • quasirandom::{HaltonSequence, SobolSequence, ...}     │
├───────────────────────────────────────────────────────────┤
│  Distribution<T> trait - pluggable sampling               │
│   • built-in: Normal / Uniform / Exponential / Poisson    │
│   • user-defined via `impl Distribution<T> for MyDist`    │
└───────────────────────────────────────────────────────────┘
```

## Backend selection

`Random` is an `enum`-dispatched tagged union. Every method on
the facade pattern-matches the active backend and forwards to
its inherent method. The match arms are `#[inline]` and the
inliner elides them entirely in release builds - verified in
`cargo bench`: the wrapped Xoshiro path is bit-identical in
codegen to the raw `Xoshiro256PlusPlus::next_u32()` call.

```
                       ╭──────── Random::rand() ────────╮
                       │                                │
       Xoshiro256++  ──┤  match &mut self.backend {     │
                       │    Xoshiro256PlusPlus(x) =>    │
                       │      x.next_u32(),             │
                       │    MersenneTwister(m) =>       │
                       │      m.rand(),                 │
                       │    Pcg32(p) =>                 │
                       │      p.next_u32(),             │
                       │    …                            │
                       │  }                              │
                       ╰────────────────────────────────╯
```

Adding a backend = one variant + one match arm in 5 dispatch
sites + a per-backend `pub mod` and feature gate. The
"variant_size_differences" lint is allowed crate-wide for this
enum because heap-boxing the large variants (MT, ChaCha20)
keeps the inline `Xoshiro256PlusPlus` case cheap.

## Ziggurat normal()

Source: `src/ziggurat.rs` + `build.rs` table generation.

The 256-strip Ziggurat sampler (Marsaglia & Tsang, 2000) is the
default `Random::normal()`. Three lookup tables are computed at
build time from the Marsaglia recurrence:

- `ZIG_NORM_K[256]` - `u32` thresholds for the fast-accept path.
- `ZIG_NORM_W[256]` - `f64` per-bin x-scale.
- `ZIG_NORM_F[256]` - `f64` per-bin heights `exp(-x²/2)`.

Tables are generated deterministically by `build.rs` so a typo
in any of the 768 magic constants is impossible - there are no
hand-written magic constants. A golden vector test in
`src/ziggurat.rs::tests::golden_normal_vector_stable` pins 16
bit-exact samples to detect drift if the recurrence ever changes.

Fast path (~99% of calls):

```
let raw = rng.next_u32() as i32;
let i = (raw as u32 & 0xff) as usize;        // bin
if raw.unsigned_abs() < ZIG_NORM_K[i] {
    return f64::from(raw) * ZIG_NORM_W[i];   // accept
}
// else: overhang (~1%) or tail (~0.03%)
```

The result: 3.7 ns/sample on Apple Silicon vs. 14.66 ns for the
Marsaglia polar method this replaced (≈4× faster).

## SIMD fill_bytes (`simd` feature)

Source: `src/xoshiro_simd.rs`.

Holds K independent Xoshiro256++ states in SIMD registers:

- AArch64 NEON: K = 2 (`uint64x2_t`), 16 bytes per inner step.
- x86_64 AVX2: K = 4 (`__m256i`), 32 bytes per inner step.

Two interleaved 2-lane `Lanes` groups give 4-way effective
ILP, plenty for the M-series' 4-wide NEON pipeline. Lanes are
derived per call from a SplitMix64 whitening of the scalar
state (collision probability ≤ K²/2²⁵⁶); the scalar state is
then advanced by adopting lane 0's final state.

**Reproducibility contract**: same seed produces a **different**
byte stream under `simd` vs. scalar. This is fundamental to
parallelising - there's no correctness-preserving way to
interleave K independent generators into a single-threaded
sequence. Documented in module rustdoc and the README's
"Squeezing more performance" section.

Below the `SIMD_THRESHOLD` (64 bytes) the dispatch falls back
to scalar so the per-call setup cost doesn't dominate.

Excluded from tarpaulin via `.tarpaulin.toml` because the
architecture-conditional code (NEON vs AVX2) can't be observed
by a single-platform coverage run. The dedicated `simd` CI
matrix job covers both halves on `ubuntu-latest` and
`macos-latest`.

## PCG backends (`pcg` feature)

Source: `src/pcg.rs`.

Two variants from O'Neill (2014):

- **PCG-XSH-RR-64/32** (`Pcg32`): 16-byte state (one `u64` LCG
  state + 64-bit stream-selecting increment). 32-bit output via
  XOR-shift-high + random-rotate. The smallest-state member of
  the PCG family.
- **PCG-XSL-RR-128/64** (`Pcg64`): 32-byte state (one `u128` +
  128-bit increment). Native 64-bit output via XOR-shift-low +
  random-rotate. Same state size as Xoshiro256++.

Both are statistically excellent (pass TestU01 BigCrush) but
not CSPRNGs. Hand-rolled; no new external dependencies.

## ChaCha20 CSPRNG backend (`crypto` feature)

Source: `src/chacha.rs`. Thin wrapper around
`rand_chacha::ChaCha20Rng` - the rand-ecosystem reference
implementation, audited via its upstream maintainers. vrd
doesn't roll its own crypto; we vendor the proof.

Two constructors:

- `Random::new_secure()` - OS-entropy seeded; requires `std`.
- `Random::from_secure_seed([u8; 32])` - deterministic.

**Bit-for-bit equivalent** to `rand_chacha::ChaCha20Rng::from_seed`
output (pinned by a test). Callers already on `rand_chacha`
can drop into vrd's facade without behavioural surprises.

The `crypto` feature requires `alloc` because the CSPRNG
variant is boxed in `RngBackend` to keep the enum's discriminant
small.

## Quasi-random sequences (`quasirandom` feature)

Source: `src/quasirandom.rs`. Three constructions:

- **`VanDerCorputSequence`** - 1-D, any prime base.
- **`HaltonSequence`** - multi-dim Van der Corput across the
  first 32 primes (`HALTON_MAX_DIM = 32`).
- **`SobolSequence`** - multi-dim with Bratley-Fox (1988)
  starter direction numbers (`SOBOL_MAX_DIM = 6`). Extending
  beyond 6 dims needs the Joe-Kuo D6 table - deferred to
  future v0.1.x.

These are **not** PRNGs and live alongside (not inside)
`Random`. Variance for Monte Carlo integration scales
`O((log n)^d / n)` rather than `O(1/√n)` for a uniform PRNG.

## no_std story

```
    no_std (Cortex-M, RISC-V, WASM):
    ─────────────────────────────────
                         Random
                            │
                            ├─ Xoshiro256PlusPlus  ✓ inline
                            ├─ MersenneTwister     ✗ needs alloc
                            ├─ Pcg32 / Pcg64       ✓ (pcg feature)
                            └─ ChaCha20            ✗ needs alloc

    Distributions (normal, exponential, uniform, poisson)
      ├─ std build:  uses f64::ln / sqrt / cos / exp
      └─ no_std:     uses libm (src/float_libm.rs)
```

The `FloatExt` trait abstracts the two paths. Two impls coexist
in the source - one gated on `feature = "std"`, the other on
`not(feature = "std")` - so the active build only compiles one.

Validated in CI on `thumbv7em-none-eabihf` (Cortex-M4F/M7F) and
`wasm32-unknown-unknown` under both `--no-default-features` and
`--no-default-features --features alloc`.

## Coverage discipline

`cargo tarpaulin --all-features` runs in CI and is gated by
Codecov's `codecov/patch` + `codecov/project` thresholds. The
target floor for new code is 100 %; current score is 100 %
(773 / 773 lines on every measured file). Two files are
excluded via `.tarpaulin.toml`:

- `src/xoshiro_simd.rs` - architecture-conditional; validated by
  the `simd` CI matrix job.
- `src/float_libm.rs` - `no_std` libm impl; `cargo test` always
  runs with `std`, so these bodies never execute under
  tarpaulin. Validated by the `no_std` CI job.

See `doc/TESTING.md` for the full strategy.

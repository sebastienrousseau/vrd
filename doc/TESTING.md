<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Testing strategy

vrd's test surface has four layers. Each catches a different
class of regression; they're complementary, not redundant.

## 1. Unit tests (inside each `src/*.rs` file)

Cover correctness of each backend's inherent methods, the
`Random` facade dispatch, and the specialised samplers
(Ziggurat, SIMD, quasi-random). Total: **122 unit tests**.

```
cargo test --all-features
```

Notable suites:

- `src/ziggurat.rs::tests` - golden vector (16 bit-exact normal
  samples), moments check (mean / stddev / skewness / kurtosis
  over 200 000 samples), zero-uniform guard exercise.
- `src/xoshiro_simd.rs::tests` - χ² uniformity on 64 KiB of
  output, short / unaligned length handling, scalar-vs-SIMD
  divergence pinned by the contract.
- `src/quasirandom.rs::tests` - first-points match the
  classical Van der Corput sequence, Sobol exhaustion at
  `i = 2³²` triggers the defensive branch, π-convergence via
  Monte Carlo with N = 4096.
- `src/pcg.rs::tests` - deterministic per seed, byte-fill on
  unaligned lengths, all `TryRng` + `SeedableRng` impls.

## 2. Integration tests (`tests/test_*.rs`)

Live in `tests/`; each file is its own crate, exercises only
the public API. Total: **34 integration tests** across:

- `tests/test_lib.rs` - surface checks.
- `tests/test_random.rs` - every `Random` constructor + every
  backend's seed path, split path, fill_array path.
- `tests/test_chacha.rs` - ChaCha20 backend integration
  (`crypto` feature only).
- `tests/test_mersenne_twister.rs` - MT19937 specific cases.
- `tests/test_macros.rs` - convenience macros.
- `tests/test_cli.rs` - `assert_cmd` exec of the `vrd` binary.

## 3. Property tests (`tests/test_proptest.rs`)

19 invariants × 256 cases each via `proptest`. Catches edge
cases that hand-written tests miss:

- bounded-below-range, int/uint inclusive bounds.
- `random_range` is half-open.
- float / double / uniform stay in their advertised intervals.
- bool edges; char ASCII.
- choose-returns-member; shuffle preserves multiset; sample
  yields distinct items.
- determinism: same seed → same first-N outputs.
- iterator-length and UUID-format / token-shape invariants.

## 4. Doc tests

Every public item carries a worked `# Examples` block. Total:
**~122 doc tests**, run by `cargo test --doc --all-features`.
Doc tests catch the most common regression - an example that
no longer compiles because a signature changed - without
needing a separate test file.

## Coverage

`cargo tarpaulin --all-features` runs in CI; gated by Codecov
(`codecov/patch` + `codecov/project`). Current coverage: **100 %
line coverage** (773 / 773 lines) on every measured file.

```
make coverage          # writes Lcov + HTML report
make coverage-gap      # flags any file below the floor
```

`.tarpaulin.toml` excludes two files that can't be measured by
a single-platform run:

- `src/xoshiro_simd.rs` - architecture-conditional (NEON on
  AArch64, AVX2 on x86_64). Validated by the `simd` CI matrix
  job that runs `cargo test --features simd` on both
  `ubuntu-latest` and `macos-latest`.
- `src/float_libm.rs` - `no_std` `FloatExt` impl. `cargo test`
  always runs with `std`, so these bodies never execute under
  tarpaulin. Validated by the
  `no_std (thumbv7em-none-eabihf)` CI job.

## Statistical validation (PractRand)

Optional informational evidence: `examples/crush.rs` pipes each
backend through PractRand's `RNG_test` and emits a markdown
pass-count summary.

```
make crush             # needs RNG_test in PATH; see scripts/crush.sh
```

CI does **not** gate on this - it's slow (~30 s per backend at
256 MiB default) and needs an external binary. Results land in
[`doc/BENCHMARKS.md`](BENCHMARKS.md) per release.

## Fuzzing (cargo-fuzz / libFuzzer)

`fuzz/` carries six libFuzzer targets:

- `fuzz_xoshiro_seed` / `fuzz_mt_seed` / `fuzz_pcg_seed` -
  drive each backend's seed path with arbitrary 32 byte inputs,
  then take 64 draws; panics are bugs.
- `fuzz_fill_array` - fuzzes length × seed.
- `fuzz_uniform_range` - `Random::uniform(low, high)` with
  arbitrary `f64` pairs; asserts output is finite and in range.
- `fuzz_quasirandom` - `Halton`/`Sobol` with arbitrary skip
  counts; asserts every point lands in `[0, 1)^D`.

Local-only by default - `make fuzz-quick` runs each for 60 s;
`make fuzz-deep` runs each for 24 h. Not wired into CI.

## CI matrix

Every PR runs (in `.github/workflows/ci.yml`):

| Job | What it covers |
|---|---|
| Rust CI / Check & Test | `cargo check + clippy + test --all-features` |
| Rust CI / Coverage | tarpaulin → Lcov → Codecov upload |
| Rust CI / Audit | `cargo audit` against RUSTSEC DB |
| Bench (compile) | `cargo bench --no-run` (catches API drift) |
| Test (nightly) | `cargo test --all-features` on nightly |
| Optional features | `cargo test` for pcg, crypto, quasirandom |
| SIMD (ubuntu-latest) | `cargo test --features simd` (AVX2) |
| SIMD (macos-latest) | `cargo test --features simd` (NEON) |
| WebAssembly | `cargo check --target wasm32-unknown-unknown` |
| no_std (thumbv7em-none-eabihf) | `cargo check --no-default-features` |
| CodeQL | static analysis |
| security / Rust Security Audit | `cargo deny check` |
| security / Dependency Review | new-deps scan on PR |

All gates must be green before merge.

## What we don't test (yet)

- Miri - no_std + std interplay would surface unsoundness in
  the SIMD `unsafe` blocks. Tracked for v0.0.12.
- TestU01 BigCrush - multi-hour run; deferred to release-only
  with PractRand SmallCrush as the per-PR cheap proxy.
- Cross-target benchmark regression detection (CodSpeed) -
  requires CodSpeed account; under evaluation.

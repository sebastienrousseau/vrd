# Changelog

All notable changes to `vrd` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] — v0.0.11 — Performance follow-ups + v0.1.0 differentiators

This release pulls in both the perf items from the v0.0.10 audit
(#88, #89) and the v0.1.0 differentiator backlog (#90–#95, #84 close-out).
After this lands, vrd is no longer "yet another non-crypto PRNG with a
nice API" — it's a tri-backend (Xoshiro / MT / PCG) + crypto-backend
(ChaCha20) + quasi-random (Halton / Sobol / VdC) RNG with a SIMD
bulk-byte path and a pluggable `Distribution` trait.

### Added

- **`Random::split()`** — parallel-safe stream derivation via
  Xoshiro256++ `jump()`. Returns `Some(Random)` on the Xoshiro backend
  (2¹²⁸-step separation guarantee), `None` on MT / PCG / ChaCha20
  (no analogous fixed-distance jump). New `examples/split.rs`. (#92)
- **`Random::fill_array<const N>()`** — stack-allocated bulk byte
  generation; allocation-free, works in pure `no_std`. (#94)
- **`pcg` feature** — adds `Pcg32` (16-byte state) and `Pcg64`
  (32-byte state) as third / fourth `RngBackend` variants. New
  constructors: `Random::new_pcg32`, `new_pcg32_with_seed`,
  `new_pcg64`, `new_pcg64_with_seed`. Hand-rolled PCG-XSH-RR-64/32
  and PCG-XSL-RR-128/64 per O'Neill 2014; no new external deps. PCG32
  measures faster than Xoshiro through the facade
  (2.7 ns vs. 3.2 ns/`u32` on Apple Silicon). New `examples/pcg.rs`.
  (#95)
- **`crypto` feature** — adds `Random::new_secure()` and
  `Random::from_secure_seed([u8; 32])`, backed by ChaCha20 via the
  audited `rand_chacha 0.10` reference implementation. Bit-for-bit
  equivalent to `rand_chacha::ChaCha20Rng::from_seed`. Moves vrd
  out of the documented "non-crypto only" pack into the small set of
  crates that cover both ends with one API. Tested via
  `tests/test_chacha.rs`; new `examples/secure.rs`. README "Choosing a
  backend" table replaces the prior "Not a CSPRNG" callout. (#90)
- **`quasirandom` feature** — `HaltonSequence` (up to 32 dims via the
  first 32 primes), `SobolSequence` (up to 6 dims via Bratley-Fox
  direction numbers), `VanDerCorputSequence` (any prime base).
  Variance scales `O((log n)^d / n)` for Monte Carlo integration,
  ray-tracing, finance. New module `src/quasirandom.rs`;
  `examples/halton.rs` and `examples/sobol.rs` show side-by-side π
  convergence vs. the PRNG path. (#91)
- **`Distribution<T>` trait** — pluggable distribution sampling.
  Built-in `Normal`, `Exponential`, `Uniform`, `Poisson` impls forward
  to the optimised methods on `Random`; users add their own via
  `impl Distribution<MyType> for MyDist`. Closes the "Custom
  distribution support" item from #84.
- **`crush` feature** — `examples/crush.rs` pipes vrd output through
  an external PractRand `RNG_test` binary and emits a pass-count
  summary. New `BENCHMARKS.md` tracks the reference results per
  release. Informational only — CI does NOT gate on it. (#93)
- **`simd` feature** — optional SIMD-batched `fill_bytes` for bulk byte
  generation. Holds K independent Xoshiro256++ states in vector
  registers (K = 2 on AArch64 NEON, K = 4 on x86_64 AVX2 — built up
  from two interleaved 2-lane groups for ILP). Off by default.
  - **Reproducibility contract**: same seed produces a **different
    byte stream** under `simd` vs. scalar. This is fundamental to
    parallelising — there's no correctness-preserving way to interleave
    K independent generators into the same sequence a single-threaded
    one would produce. Lanes are derived from a SplitMix64 whitening
    of the scalar state (collision probability ≤ K²/2²⁵⁶); after the
    SIMD loop, the scalar state is advanced by adopting lane 0's
    final state. Buffers under 64 B fall back to the scalar path
    where setup cost would dominate.
  - Statistical quality is unchanged — each lane is a full Xoshiro256++.

### Performance

- **`Random::normal()` is ~4× faster** — replaced the Marsaglia polar
  method with a **256-strip Ziggurat sampler** (Marsaglia & Tsang, 2000)
  whose `K`/`W`/`F` tables are pre-computed at build time by `build.rs`.
  The fast path costs one `u32` draw, one table lookup, and one `f64`
  multiply; the overhang branch (~1% of calls) adds one `exp` and one
  `f64` draw; the tail branch (~0.03% of calls) falls back to
  exponential rejection. On Apple Silicon M-series:
  `distributions/normal(0, 1)` 14.66 ns → 3.39 ns. A golden vector
  test pins down 16 bit-exact samples to catch table drift; a
  moments check verifies skewness < 0.05 and excess kurtosis < 0.1
  over 200 000 samples. (#89)
- **`fill_bytes` SIMD path** under the new `simd` feature reaches
  **2.22× on 1 KiB** and **3.02× on 16 KiB** vs. the scalar baseline on
  Apple Silicon M-series:

  | Buffer | Scalar | SIMD (NEON) | Speedup | GB/s |
  | :--- | ---: | ---: | ---: | ---: |
  | 1 KiB | 137.9 ns | 62.1 ns | 2.22× | 16.5 |
  | 16 KiB | 2188 ns | 725 ns | 3.02× | 22.6 |

  AVX2 path is implemented but un-benchmarked here; targets ~25–40
  GB/s on a modern x86_64 part. (#88)

### Changed

- **`#![forbid(unsafe_code)]` → `#![deny(unsafe_code)]`** at the crate
  root, so the `xoshiro_simd` module can lift the deny locally for
  architecture intrinsics. All other modules remain `unsafe`-free.
- **`Xoshiro256PlusPlus::fill_bytes`** documents the SIMD vs. scalar
  reproducibility contract in its rustdoc.
- **`Random` / `RngBackend`** no longer derive `Eq` / `Hash` / `Ord` /
  `PartialOrd`. The ChaCha20 backend's underlying type doesn't
  implement them, and "ordering RNG states" was never a meaningful
  operation. `PartialEq` is kept for snapshot / determinism
  comparisons.
- **`serde` feature** now forwards through to `rand_chacha?/serde` so
  the ChaCha20 backend round-trips cleanly when both `crypto` and
  `serde` are enabled.

## [0.0.10] — 2026-04-29 — Modernization release

This release rewrites `vrd` around **Xoshiro256++** as the default
backend, adds **`no_std` + WebAssembly support**, ships a competitive
convenience surface (UUID v4, hex/base64 tokens, iterator adapters,
uniform distribution), and drops a large amount of dependency surface
that wasn't part of the core RNG mission. Existing 0.0.x callers will
need to make a few small changes — see *Migration* below.

### Added

#### Backends and target support
- **Xoshiro256++ generator** (`vrd::Xoshiro256PlusPlus`) — small-state (32 B), period 2^256 - 1, with SplitMix64 seed whitening so even pathological seeds (`[0u8; 32]`, `[1u8; 32]`) yield well-distributed output.
- **`no_std` support** — `default-features = false` produces a pure-core build with no allocator. Validated on `thumbv7em-none-eabihf` (Cortex-M4F/M7F) in CI.
- **WebAssembly support** — `wasm32-unknown-unknown` gated in CI under both `--no-default-features` and `--features alloc`.
- **`alloc` feature** — re-introduces `Random::bytes`, `Random::string`, `Random::sample`, and the boxed Mersenne Twister backend.
- **`rand 0.10` traits** — `TryRng`, the blanket-implemented `Rng`, and `SeedableRng` are implemented on `Random`, `Xoshiro256PlusPlus`, and `MersenneTwister`.
- **Native u64 path** — `Random::u64` calls Xoshiro's native 64-bit output directly; under MT it concatenates two 32-bit outputs as before.
- **`MersenneTwister::from_seed([u8; 32])`** — accepts the same seed shape as Xoshiro, XOR-folding the bytes so callers don't silently lose entropy.
- **`Random::from_u64_seed`** — convenience constructor for deterministic, allocation-free seeding.

#### Convenience APIs
- **Iterator adapters on `Random`** — `iter_u32()`, `iter_u64()`, and `iter_bytes()` return `impl Iterator<Item = T>` (the byte iterator is the new public `ByteIter` struct). Composes with `take` / `filter` / `collect` for ergonomic streaming.
- **UUID v4 generation** — `Random::uuid_v4_bytes()` returns `[u8; 16]` allocation-free (RFC 4122 §4.4 version + §4.1.1 variant bits set); `Random::uuid_v4()` returns the canonical hyphenated 36-char `String` (`alloc`).
- **Random tokens** — `Random::hex_token(byte_len)` (lowercase hex, length `byte_len * 2`) and `Random::base64_token(byte_len)` (URL-safe base64 per RFC 4648 §5, no padding). Both `alloc`-only.
- **Uniform float distribution** — `Random::uniform(low, high)` for `f64` in `[low, high)`.
- **Unbiased bounded sampling** — `Random::int`, `Random::uint`, `Random::random_range`, and the new public `Random::bounded` use Lemire's nearly-divisionless method, eliminating the modulo bias present in earlier versions.
- **Bit-precise floats** — `Random::float` carries 24 mantissa bits (the f32 maximum); `Random::double` / `Random::f64` carry 53 (the f64 maximum). Both are guaranteed `[0.0, 1.0)`.

#### Tooling, tests, and examples
- **Comparative benchmarks** — `cargo bench` runs `vrd::Random` (Xoshiro), `vrd::Random` (MT), `fastrand`, and `rand::rng()` head-to-head on `u32`, `u64`, byte fills, bounded sampling, and distribution sampling.
- **Property-based tests** — `tests/test_proptest.rs` adds 19 invariants × 256 cases each: bounded-below-range, int/uint inclusive bounds, random_range half-open, float/double/uniform in `[0, 1)` (or specified bounds), bool edges, char ASCII, choose-returns-member, shuffle-preserves-multiset, sample-distinct, determinism, iterator length, UUID format, hex/base64 token shape.
- **CLI integration tests** — `tests/test_cli.rs` uses `assert_cmd` to exec the `vrd` binary and exercise the `fn main()` argv → `dispatch` → `process::exit` glue.
- **35 single-word examples** under `examples/`, with an `examples/all.rs` orchestrator that runs every demo (`cargo run --example all`).
- **Embedded CI job** — every PR is checked against `thumbv7em-none-eabihf` with both `--no-default-features` and `--no-default-features --features alloc`.
- **`bench-build` CI guard** — runs `cargo bench --no-run` on every PR to catch bench-suite breakage from API refactors.
- **`CHANGELOG.md`** — this file.

### Changed

- **Default backend is Xoshiro256++.** `Random::new()` returns a Xoshiro-backed instance. To opt back into MT19937, use `Random::new_mersenne_twister()` (requires `alloc + std`) or `Random::new_mersenne_twister_with_seed(u32)` (`alloc` only).
- **MSRV bumped to 1.70** — earlier versions advertised `1.56` but didn't actually compile that low.
- **Release profile** — `opt-level = 3` (was `"s"`); RNG throughput trumps binary size for this crate's positioning.
- **`MersenneTwisterError` simplified** — now a single `InvalidConfig(&'static str)` variant. The `IoError` and `SerializationError` variants, plus the `serialize_to_file` / `deserialize_from_file` / yaml / toml helpers, were removed.
- **`VrdError::GeneralError`** now carries `&'static str` instead of `String` for `no_std` compatibility.
- **Crate description, keywords, categories** updated to reflect the dual-backend / `no_std` positioning.
- **Crate identity** — full name `Versatile Random Distributions (VRD)` introduced once at the top of `README.md`, `lib.rs` rustdoc, `TEMPLATE.md`, and the GitHub repo description; every other reference uses lowercase `vrd`.
- **GitHub repo description and topics** — updated for 2026 RNG-niche SEO: 20 topics covering `rust`, `rng`, `prng`, `random-number-generator`, `xoshiro`, `xoshiro256`, `mersenne-twister`, `mt19937`, `no-std`, `no-alloc`, `embedded`, `cortex-m`, `rand`, `rand-core`, `distributions`, `algorithms`, `lightweight`, `fast`, `entropy`.
- **Logo CDN migration** — image references moved from `kura.pro/vrd/images/` to `cloudcdn.pro/vrd/v1/`.
- **`docs.rs` metadata** — `all-features = true` so the rendered docs surface the alloc/serde-gated APIs.
- **`serde` minimum** bumped `1.0.209 → 1.0.228` to match the resolved lockfile.
- **`criterion`** bumped `0.5.1 → 0.8.2` (dev-dep); migrated `criterion::black_box` → `std::hint::black_box` to clear deprecation warnings.
- **`actions/checkout`** bumped `v4 → v6` in `ci.yml`.
- **TEMPLATE.md** rewritten around the v0.0.10 dual-backend narrative; previously framed vrd as MT-only.
- **CONTRIBUTING.md** replaced generic boilerplate with an actionable pre-submit checklist mirroring the CI gates and a documented coverage policy.

### Removed

- **`Random::fill<T>`** — generic `T: Default + RemAssign<u32> + BitOrAssign<u32>` API was opaque and produced output that was effectively `T::default() | rand`. Use `Random::try_fill_bytes(&mut [u8])` for bulk randomness.
- **`Random::new_xoshiro`** — superseded by `Random::new`, which now *is* Xoshiro.
- **`create_log_entry` and the `logging` feature** — vrd is an RNG library, not a log formatter.
- **`tokio`, `dtt`, `rlg`, `uuid`, `serde_json`, `serde_yml`, `toml`, `commons`, `bitflags`** dependencies — none of them were on the core RNG path.
- **`run()` entry-point** — leftover from the binary scaffolding.
- **Per-format ser/de helpers on `MersenneTwisterConfig`** — use `serde_json` / `serde_yaml_ng` / `toml` directly via the `serde` feature if you need them.

### Fixed

- **RUSTSEC-2026-0097 (`rand` unsound with custom logger)** — addressed by upgrading to `rand 0.10.1`.
- **RUSTSEC bytes integer overflow in `BytesMut::reserve` (2026-02-03)** — transitive dep, fixed by `bytes 1.11.1`.
- **RUSTSEC-2024-0421 (`tokio`)** — bumped through `1.52`; tokio is now removed entirely.
- **RUSTSEC-2024-0436 (`paste` unmaintained)** — eliminated by dropping `dtt` / `rlg`.
- **RUSTSEC-2025-0067 / 0068 (`libyml` / `serde_yml`)** — eliminated by dropping the `yaml` feature and its deps.
- **`Xoshiro256PlusPlus` low-entropy seed handling** — raw seed bytes are now whitened through SplitMix64 before priming the four-word state. Previously, `[1u8; 32]` produced four identical state words and a degenerate sequence.
- **`Random::seed(u32)` on Xoshiro** — re-seed now produces a well-distributed state via SplitMix64; previously only the low 4 bytes of a 32-byte buffer were populated.
- **MSRV claim** — `src/lib.rs` rustdoc preamble matches `Cargo.toml` (`1.70.0`); the prior `1.56.0` claim was a false promise.
- **Cold-rejection split on `Random::bounded`** — Lemire rejection helper marked `#[cold] #[inline(never)]`; common path stays tight in i-cache.
- **Coverage gap on the MT-side `Random` API** — added `test_full_api_on_mersenne_backend` walking every public method on the MT backend, plus targeted tests for `Display` on MT, `set_mti` / `twist` no-op contract on Xoshiro, and `MersenneTwisterConfig::set_config` round-trip.
- **Broken in-page anchor** — `Back to Top` now points at `#versatile-random-distributions-vrd` to match the new H1.

### Performance

- **`normal()`** switched from Box-Muller to Marsaglia polar method. Empirically a wash on Apple Silicon (libm `cos` is near-free); meaningful win on platforms with slower trig. The 3.5× target via Ziggurat is tracked in [#89](https://github.com/sebastienrousseau/vrd/issues/89).

### Documentation

- **100% rustdoc + 100 doctests** across every public item: 110 documented items, with `# Examples` blocks; 100 doctests passing (was 53). `# Errors` and `# Panics` sections added on every fallible/panicking method.
- **README rewrite** around the dual-backend / `no_std` / WebAssembly story. Replaced "When to reach for vrd" with a 12-row competitive comparison table (`vrd` vs `rand` 0.10 vs `fastrand` 2.x vs `oorandom` 11.x). Added "What you don't have to depend on" callout naming the four companion crates (`uuid`, `hex` / `data-encoding`, `base64`, `rand_distr`) that vrd makes optional.
- **FAQ rewritten with code** — 13 questions covering everyday API surface, `rand` migration, non-security tokens, thread safety, save/restore via serde, MT-vs-Xoshiro tradeoffs, output stability commitment (bit-stable across patch releases), state size, and benchmark numbers.
- **README PGO + `target-cpu=native` documentation** — opt-in performance knobs documented for downstream consumers.
- **Coverage policy in `CONTRIBUTING.md`** — canonical `cargo llvm-cov` command, the documented ≥98% per-file floor, and the explanation of remaining `#[should_panic]` accounting artifacts.
- **Strategic roadmap captured as GitHub issues** under two milestones:
  - [v0.0.11 — Performance follow-ups](https://github.com/sebastienrousseau/vrd/milestone/1): SIMD `fill_bytes` ([#88](https://github.com/sebastienrousseau/vrd/issues/88)), Ziggurat normal ([#89](https://github.com/sebastienrousseau/vrd/issues/89)).
  - [v0.1.0 — Strategic differentiators](https://github.com/sebastienrousseau/vrd/milestone/2): ChaCha20 CSPRNG ([#90](https://github.com/sebastienrousseau/vrd/issues/90)), quasi-random sequences ([#91](https://github.com/sebastienrousseau/vrd/issues/91)), `Random::split()` ([#92](https://github.com/sebastienrousseau/vrd/issues/92)), PractRand validation ([#93](https://github.com/sebastienrousseau/vrd/issues/93)), `fill_array<const N>` ([#94](https://github.com/sebastienrousseau/vrd/issues/94)), PCG backend ([#95](https://github.com/sebastienrousseau/vrd/issues/95)).

### Migration

| Before | After |
| :--- | :--- |
| `use rand::{RngCore, SeedableRng};` | `use rand::rand_core::{Rng, SeedableRng};` (or `TryRng` for fallible variants) |
| `Random::new()` (MT-backed) | `Random::new_mersenne_twister()` to keep MT; otherwise `Random::new()` is now Xoshiro |
| `Random::new_xoshiro()` | `Random::new()` |
| `rng.fill(&mut buf)` | `use rand::rand_core::TryRng; rng.try_fill_bytes(&mut buf)` |
| `MersenneTwisterError::IoError(_)` / `SerializationError(_)` | Use `serde` directly; the helpers have moved out of vrd |
| `VrdError::GeneralError("…".to_string())` | `VrdError::GeneralError("…")` (`&'static str`) |

### Security audit

`cargo audit` is now clean. The previously deferred `paste` / `libyml` / `serde_yml` advisories are gone — those crates are no longer in the dependency tree.

[0.0.10]: https://github.com/sebastienrousseau/vrd/releases/tag/v0.0.10

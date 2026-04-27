# Changelog

All notable changes to `vrd` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Post-modernization polish layered on top of the `0.0.10` release. No
breaking changes; everything below is additive or documentation.

### Added

- **Iterator adapters on `Random`** — `iter_u32()`, `iter_u64()`, and `iter_bytes()` return `impl Iterator<Item = T>` (the byte iterator is the new public `ByteIter` struct). Composes with `take` / `filter` / `collect` for ergonomic streaming.
- **UUID v4 generation** — `Random::uuid_v4_bytes()` returns a `[u8; 16]` allocation-free (RFC 4122 §4.4 version + §4.1.1 variant bits set); `Random::uuid_v4()` returns the canonical hyphenated 36-char `String` (`alloc`).
- **Random tokens** — `Random::hex_token(byte_len)` (lowercase hex, length `byte_len * 2`) and `Random::base64_token(byte_len)` (URL-safe base64 per RFC 4648 §5, no padding). Both `alloc`-only.
- **Uniform float distribution** — `Random::uniform(low, high)` for `f64` in `[low, high)`.
- **Property-based tests** — `tests/test_proptest.rs` adds 19 invariants × 256 cases each: bounded-below-range, int/uint inclusive bounds, random_range half-open, float/double/uniform in `[0, 1)` (or specified bounds), bool edges, char ASCII, choose-returns-member, shuffle-preserves-multiset, sample-distinct, determinism, iterator length, UUID format, hex/base64 token shape.
- **WebAssembly CI gate** — new `wasm` job runs `cargo check --target wasm32-unknown-unknown` for `--no-default-features` and `--features alloc`. CI now spans macOS / Linux / Windows / Cortex-M / WebAssembly.
- **Three new examples** — `examples/iterators.rs`, `examples/uuid.rs`, `examples/tokens.rs`. All wired into `examples/all.rs` and `Cargo.toml`'s `[[example]]` list.

### Changed

- **Crate identity** — full name `Versatile Random Distributions (VRD)` introduced once at the top of `README.md`, `lib.rs` rustdoc, `TEMPLATE.md`, and the GitHub repo description; every other reference uses lowercase `vrd`. Replaces the redundant paired form `Random (VRD)` (54 occurrences across 38 files). Pattern matches the `PCG (Permuted Congruential Generator)` / `MT19937 (Mersenne Twister)` convention.
- **GitHub repo description and topics** — updated for 2026 RNG-niche SEO: 20 topics covering `rust`, `rng`, `prng`, `random-number-generator`, `xoshiro`, `xoshiro256`, `mersenne-twister`, `mt19937`, `no-std`, `no-alloc`, `embedded`, `cortex-m`, `rand`, `rand-core`, `distributions`, `algorithms`, `lightweight`, `fast`, `entropy`.
- **MSRV claim** — `src/lib.rs` rustdoc preamble now matches `Cargo.toml` (`1.70.0`); the prior `1.56.0` claim was a false promise.
- **TEMPLATE.md** — rewritten around the v0.0.10 dual-backend narrative; previously framed vrd as MT-only.
- **CONTRIBUTING.md** — replaced generic boilerplate with an actionable pre-submit checklist mirroring the CI gates (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `cargo build --no-default-features`); documents signed-commit and `Assisted-by:` trailer conventions.
- **Logo CDN migration** — image references moved from `kura.pro/vrd/images/` to `cloudcdn.pro/vrd/v1/`.
- **`serde` minimum** bumped `1.0.209 → 1.0.228` to match the resolved lockfile (closes Dependabot PR #81).

### Fixed

- **Coverage gap on the MT-side `Random` API** — added `test_full_api_on_mersenne_backend` walking every public method on the MT backend, plus targeted tests for `Display` on MT, `set_mti` / `twist` no-op contract on Xoshiro, and `MersenneTwisterConfig::set_config` round-trip.
- **README/code drift on `Random::pseudo`** — the migration list incorrectly claimed `pseudo` had been removed; the method is still present and supported. Migration entry removed.
- **Broken in-page anchor** — `Back to Top` now points at `#versatile-random-distributions-vrd` to match the new H1.
- **Bench-build CI guard** — new `bench-build` job runs `cargo bench --no-run` on every PR to catch bench-suite breakage from API refactors.
- **Cold-rejection split on `Random::bounded`** — Lemire rejection helper marked `#[cold] #[inline(never)]`; common path stays tight in i-cache.

### Performance

- **`normal()`** switched from Box-Muller to Marsaglia polar method. Empirically a wash on Apple Silicon (libm `cos` is near-free); meaningful win on platforms with slower trig. The 3.5× target via Ziggurat is tracked in [#89](https://github.com/sebastienrousseau/vrd/issues/89).

### Documentation

- **100% rustdoc + 100% doctest examples** across every public item: 110 documented items, 97 with `# Examples` blocks, 99 doctests passing (was 53). `# Errors` and `# Panics` sections added on every fallible/panicking method.
- README **TL;DR section removed** (information was already in the tagline, Highlights, and FAQ).
- README **PGO + `target-cpu=native` documentation** — opt-in performance knobs documented for downstream consumers.
- **Strategic roadmap captured as GitHub issues** under two milestones:
  - [v0.0.11 — Performance follow-ups](https://github.com/sebastienrousseau/vrd/milestone/1): SIMD `fill_bytes` (#88), Ziggurat normal (#89).
  - [v0.1.0 — Strategic differentiators](https://github.com/sebastienrousseau/vrd/milestone/2): ChaCha20 CSPRNG (#90), quasi-random sequences (#91), `Random::split()` (#92), PractRand validation (#93), `fill_array<const N>` (#94), PCG backend (#95).

## [0.0.10] — Modernization release

This release rewrites `vrd` around **Xoshiro256++** as the default backend,
adds **`no_std` support**, and drops a large amount of dependency surface
that wasn't part of the core RNG mission. Existing 0.0.x callers will
need to make a few small changes — see *Migration* below.

### Added

- **Xoshiro256++ generator** (`vrd::Xoshiro256PlusPlus`) — small-state (32 B), period 2^256 - 1, with SplitMix64 seed whitening so even pathological seeds (`[0u8; 32]`, `[1u8; 32]`) yield well-distributed output.
- **`no_std` support** — `default-features = false` produces a pure-core build with no allocator. Validated on `thumbv7em-none-eabihf` (Cortex-M4F/M7F) in CI.
- **`alloc` feature** — re-introduces `Random::bytes`, `Random::string`, `Random::sample`, and the boxed Mersenne Twister backend.
- **Unbiased bounded sampling** — `Random::int`, `Random::uint`, `Random::random_range`, and the new public `Random::bounded` use Lemire's nearly-divisionless method, eliminating the modulo bias present in earlier versions.
- **Bit-precise floats** — `Random::float` carries 24 mantissa bits (the f32 maximum); `Random::double` / `Random::f64` carry 53 (the f64 maximum). Both are guaranteed `[0.0, 1.0)`.
- **Native u64 path** — `Random::u64` calls Xoshiro's native 64-bit output directly; under MT it concatenates two 32-bit outputs as before.
- **`rand 0.10` traits** — `TryRng`, the blanket-implemented `Rng`, and `SeedableRng` are implemented on `Random`, `Xoshiro256PlusPlus`, and `MersenneTwister`.
- **`MersenneTwister::from_seed([u8; 32])`** — accepts the same seed shape as Xoshiro, XOR-folding the bytes so callers don't silently lose entropy.
- **`Random::from_u64_seed`** — convenience constructor for deterministic, allocation-free seeding.
- **Comparative benchmarks** — `cargo bench` runs `vrd::Random` (Xoshiro), `vrd::Random` (MT), `fastrand`, and `rand::rng()` head-to-head on `u32`, `u64`, byte fills, and distribution sampling.
- **Embedded CI job** — every PR is checked against `thumbv7em-none-eabihf` with both `--no-default-features` and `--no-default-features --features alloc`.
- **`CHANGELOG.md`** — this file.

### Changed

- **Default backend is Xoshiro256++.** `Random::new()` returns a Xoshiro-backed instance. To opt back into MT19937, use `Random::new_mersenne_twister()` (requires `alloc` + `std`) or `Random::new_mersenne_twister_with_seed(u32)` (`alloc` only).
- **MSRV bumped to 1.70** — earlier versions advertised `1.56` but didn't actually compile that low.
- **Release profile** — `opt-level = 3` (was `"s"`); RNG throughput trumps binary size for this crate's positioning.
- **`MersenneTwisterError` simplified** — now a single `InvalidConfig(&'static str)` variant. The `IoError` and `SerializationError` variants, plus the `serialize_to_file` / `deserialize_from_file` / yaml / toml helpers, were removed.
- **`VrdError::GeneralError`** now carries `&'static str` instead of `String` for `no_std` compatibility.
- **Crate description, keywords, categories** updated to reflect the dual-backend / no_std positioning.
- **`docs.rs` metadata** — `all-features = true` so the rendered docs surface the alloc/serde-gated APIs.

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

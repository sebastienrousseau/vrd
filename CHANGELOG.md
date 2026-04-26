# Changelog

All notable changes to `vrd` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

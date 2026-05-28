<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Engineering policies

The non-functional promises vrd commits to and the rules the
contributor base follows. Bind tightly to whatever release
you're using; loosen at major versions.

## MSRV (Minimum Supported Rust Version)

- **Declared**: 1.70.
- **Verified**: `make msrv` runs `cargo check` on the declared
  toolchain across `--no-default-features`, `--features alloc`,
  and `--all-features`.
- **Floor change rule**: any MSRV bump is a minor-version bump
  on its own; never sneaks in with a patch release. Documented
  in `CHANGELOG.md` and `RELEASE-NOTES-vX.Y.Z.md`.

## SemVer

- **Patch releases (`0.0.x → 0.0.x+1`)** — bit-stable output
  for every public method, given the same seed. Bug fixes,
  internal refactors, doc-only changes, dev-dep bumps.
- **Minor releases (`0.x → 0.x+1` while pre-1.0)** — may change
  algorithm (Ziggurat replacing polar in v0.0.11 was a minor
  bump; we treated it as one because we were already pinning
  v0.0.11 for the broader differentiator set). MSRV bumps,
  feature additions, behavioural changes are clearly flagged.
- **Major releases (`x → x+1` post-1.0)** — breaking. Removals,
  trait surface changes, default-feature shifts.

## Output stability

For a given seed and method:

- **Patch releases** — bit-exact output guarantee.
- **Minor releases** — output may change if an algorithm is
  swapped. Affected methods are listed in the changelog's
  "Migration" subsection by name.
- **Major releases** — no output-stability guarantee across
  major boundaries.

The `rand` crate explicitly does not provide either guarantee.
If your test suite depends on stable seed → output mapping,
that's a meaningful difference. The output-stability commitment
covers every backend: Xoshiro256++, MT19937, PCG, and ChaCha20.

The **`simd` feature is the exception**: its output is
deterministically different from the scalar path for the same
seed (lane derivation is fundamental to parallelising). Same
seed gives the same SIMD output across patches; **same seed
gives a different output if `simd` is toggled on/off**. Code
that depends on bit-stability must stay on the scalar default.

## Concurrency

- `Random` (and `Xoshiro256PlusPlus`, and `MersenneTwister`,
  and `Pcg32`, and `Pcg64`, and `ChaChaRng`) hold mutable
  internal state.
- They are **not** `Send + Sync`-safe under concurrent
  mutation; the standard pattern is one RNG per thread, seeded
  distinctly.
- For parallel deterministic streams on the default Xoshiro
  backend, use `Random::split()` to derive a child stream
  2¹²⁸ calls ahead.
- For non-Xoshiro backends, derive distinct seeds per worker
  via your own scheme (typically `from_u64_seed(worker_id)`).

## Panic discipline

- `Random::new()` may panic if the OS entropy source is
  unavailable (delegates to `rand::random()`). Use
  `Random::from_seed(...)` for no-panic alternatives.
- All other constructors are infallible.
- Bounded sampling panics on `low >= high` (programmer error;
  matches `rand`'s behaviour).
- Distributions panic on non-finite parameters
  (`normal(NaN, _)`, `exponential(0.0)`); we don't silently
  emit garbage. The panic message names the bad parameter.
- The fuzz harness asserts the public API never panics on
  arbitrary input; any panic surfaced by fuzz is a bug.

## Unsafe

- Crate root: `#![deny(unsafe_code)]` (was `forbid` until
  v0.0.11; relaxed to `deny` so `xoshiro_simd` can lift it
  locally for architecture intrinsics).
- Only one module uses `unsafe`: `src/xoshiro_simd.rs` (NEON
  intrinsics on AArch64, AVX2 on x86_64). Each `unsafe` block
  carries an inline `SAFETY:` comment.
- No `unsafe` in any of: `random.rs`, `xoshiro.rs`,
  `mersenne_twister.rs`, `pcg.rs`, `chacha.rs`,
  `quasirandom.rs`, `ziggurat.rs`, `distribution.rs`,
  `float_libm.rs`, `macros.rs`, `lib.rs`.

## Security

vrd is **not** a CSPRNG by default. Enable the `crypto` feature
and use `Random::new_secure()` / `from_secure_seed()` for
crypto-quality output. See [`../SECURITY.md`](../SECURITY.md)
for the disclosure address and the full threat model.

## Supply chain

- `cargo deny check` runs on every PR (`.github/workflows/ci.yml`
  Audit job + `make deny`).
- `cargo audit` runs on every PR against the RUSTSEC advisory
  database.
- `cargo about generate` produces `NOTICE` listing every
  third-party crate vrd redistributes with its license text
  (`make notice`).
- Dependabot auto-PRs minor/patch dev-dep bumps; we land them
  promptly.

## Documentation

- Every public item carries a `# Examples` block.
- Module-level rustdoc explains *why* the module exists, not
  just *what* it does.
- Intra-doc links use full crate paths
  (`[`crate::pcg::Pcg32`]`) — the linter on `cargo doc
  --no-deps --all-features` is gated in CI.
- Doc tests are part of the normal test count (~122 currently)
  and must all pass under `--all-features`.
- Long-form architectural docs live in `doc/`:
  `ARCHITECTURE.md`, `TESTING.md`, `COMPARISON.md`,
  `BENCHMARKS.md`, `USER-GUIDE.md`, `MIGRATION-FROM-*.md`.

## Performance

- Every public method's hot path is included in `cargo bench`.
- Release profile: `opt-level = 3`, `lto = true`,
  `codegen-units = 1`, `panic = "abort"`. The Cargo.toml's
  `[profile.release]` is the canonical reference.
- `target-cpu=native` is **not** baked into the release
  profile (would break `cargo install` for binaries); enable
  it in the consuming crate.
- Performance regressions in benchmarks should land with a
  reasoned justification in the PR description.

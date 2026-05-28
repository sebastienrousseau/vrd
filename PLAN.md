<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# vrd roadmap

Where we are, what's next. Treat dates as targets, not promises.

## Phase 1 - Core RNG ✅ (shipped in v0.0.10, 2026-04-29)

- [x] Xoshiro256++ as default backend
- [x] MT19937 as legacy backend
- [x] `no_std` core; `alloc`-gated extras
- [x] Cortex-M + WebAssembly CI gating
- [x] Unbiased bounded sampling (Lemire)
- [x] Bit-precise floats
- [x] `uniform`, `normal` (polar), `exponential`, `poisson`
- [x] UUID v4, hex / base64 tokens
- [x] `rand 0.10` trait surface (`TryRng`, `Rng`, `SeedableRng`)

## Phase 2 - Performance follow-ups + v0.1.0 differentiators ✅ (shipped in v0.0.11, 2026-05-28)

- [x] Ziggurat `normal()` with build-time tables (#89)
- [x] SIMD `fill_bytes` (NEON / AVX2) behind `simd` feature (#88)
- [x] ChaCha20 CSPRNG backend behind `crypto` feature (#90)
- [x] PCG32 / PCG64 backends behind `pcg` feature (#95)
- [x] Halton / Sobol / Van der Corput behind `quasirandom`
      feature (#91)
- [x] `Random::split()` for parallel-safe stream derivation (#92)
- [x] `Random::fill_array<const N>` for stack-allocated bulk
      bytes (#94)
- [x] PractRand validation harness behind `crush` feature (#93)
- [x] `Distribution<T>` trait for pluggable distributions (#84)
- [x] 100 % line coverage gate (Codecov)

## Phase 3 - Repo template alignment + 100% doc coverage ✅ (shipped in v0.0.12, 2026-05-28)

Mirrored noyalib's repo conventions inside vrd's single-crate layout.

- [x] README front matter + Contents nav
- [x] Makefile + `scripts/`
- [x] `doc/` directory: ARCHITECTURE, TESTING, COMPARISON,
      POLICIES, USER-GUIDE, MIGRATION-FROM-RAND,
      MIGRATION-FROM-FASTRAND, BENCHMARKS
- [x] Governance: SECURITY, GETTING_STARTED, GLOSSARY,
      PLAN (this file), `rust-toolchain.toml`, REUSE.toml,
      `about.toml` + `about.hbs`
- [x] `fuzz/` with 6 libFuzzer targets
- [x] `pkg/` subset: homebrew + docker + nix
- [x] `examples/distribution.rs`
- [x] CI: docs.yml (Pages build+deploy), scorecard.yml,
      security.yml
- [x] 100% public doc coverage via `#![deny(missing_docs)]`;
      100% private doc coverage via clippy lint
- [x] GitHub Pages migrated from `gh-pages` branch to Actions
      workflow; custom domain `doc.vrdlib.com` preserved
- [x] Repo description + 20 topics refreshed

## Phase 4 - v0.0.12 (target: 2026-Q3)

Post-launch bug roll-up + the SIMD micro-optimisation that
v0.0.11 left on the table.

- [ ] AVX2 4-lane micro-optimisation (per benchmark, ≤ 1.5×
      headroom remaining)
- [ ] Miri runs in CI matrix (nightly, focused on `unsafe`
      blocks in `xoshiro_simd`)
- [ ] Any bug reports from v0.0.11 in-the-wild adoption
- [ ] PractRand SmallCrush baseline numbers in `doc/BENCHMARKS.md`

## Phase 5 - v0.0.13: expanded distribution catalogue (target: 2026-Q4)

Close the gap with `rand_distr`. Each distribution lands as a
new `impl Distribution<f64> for FooDist` in `src/distribution.rs`.

- [ ] Binomial
- [ ] Gamma (Marsaglia-Tsang squeeze method)
- [ ] Beta (derived from Gamma)
- [ ] Log-normal
- [ ] Cauchy
- [ ] Triangular
- [ ] Discrete distributions: `Bernoulli`, `Geometric`

## Phase 6 - v0.1.0: binary distribution (target: 2027-Q1)

Activate the `pkg/` channels with a `release-binaries.yml`
workflow.

- [ ] Build `vrd` CLI binary for 14 platforms (Linux ARM/x86,
      macOS ARM/x86, Windows MSVC ARM/x86, FreeBSD x86)
- [ ] cosign keyless signing + SLSA L3 provenance
- [ ] Auto-publish to homebrew tap, docker registry, nix flake
- [ ] Expand `pkg/` to include deb, rpm, arch, scoop if user
      demand surfaces (current `pkg/` ships only the subset:
      homebrew + docker + nix)

## Phase 7 - v0.2.0: Sobol direction-number expansion (no target)

Currently `SobolSequence` supports 6 dimensions (Bratley-Fox
1988 starter set). Extending to higher dimensions requires
shipping the Joe-Kuo D6 direction-number tables (~5 MB at
21 201 dimensions; a smaller cut at, say, 256 dimensions is
~30 KB and probably covers most use).

- [ ] Decide on dimension cap (256 vs 1024 vs full 21 201)
- [ ] Ship direction-number table as compile-time-included
      asset
- [ ] Update README and `doc/COMPARISON.md`

## Phase 8 - v1.0.0: stability commitment (no target)

The 1.0 release locks the public API and extends the
output-stability commitment from patch-releases-only to
minor-releases-too.

- [ ] Audit every public symbol; remove deprecated /
      experimental
- [ ] Confirm 100 % coverage holds
- [ ] PractRand BigCrush results checked into `doc/BENCHMARKS.md`
      (one-off run; not per-release)
- [ ] OSS-Fuzz integration if community wants continuous
      fuzzing

## What's NOT planned

- **AES-CTR or other CSPRNG algorithms beyond ChaCha20**.
  Keep the `crypto` surface small. Add later via separate
  sub-features only if demand surfaces.
- **Argon2 / scrypt / PBKDF2**. KDFs, not RNGs. Out of scope.
- **A `vrd` CSPRNG audit**. vrd vendors `rand_chacha`'s
  audited ChaCha20; we don't claim our own crypto audit.
- **Workspace conversion**. vrd stays single-crate; noyalib's
  workspace pattern fits noyalib's CLI / LSP / MCP / WASM
  satellite-crate story but not vrd's library-first model.

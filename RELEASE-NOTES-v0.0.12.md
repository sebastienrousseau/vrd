<!-- SPDX-FileCopyrightText: 2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# vrd v0.0.12 - Release Notes

Repo template alignment with the noyalib reference layout, plus
100% rust doc coverage across every public and private item.
No public API changes; all source-level diffs are doc comments
and operational scaffolding.

## Headline numbers

- **100% public + private doc coverage**. `#![deny(missing_docs)]`
  at the lib root catches public-item gaps; the lib-scoped
  `#![warn(clippy::missing_docs_in_private_items)]` lint catches
  private-item gaps. The docs.yml CI gate runs with
  `RUSTDOCFLAGS=-D warnings`, upgrading both to hard failures.
- **GitHub Pages migrated to Actions**. The legacy `gh-pages`
  branch source is retired; `.github/workflows/docs.yml` now
  builds + deploys via `actions/deploy-pages@v4`. Custom domain
  `doc.vrdlib.com` preserved.
- **Six new long-form references** in `doc/`: ARCHITECTURE,
  TESTING, COMPARISON (feature matrix vs `rand`, `fastrand`,
  `oorandom`, `nanorand`, `rand_xoshiro`), POLICIES, USER-GUIDE,
  MIGRATION-FROM-RAND, MIGRATION-FROM-FASTRAND.
- **Six libFuzzer targets** under `fuzz/` via cargo-fuzz
  (local-only; `make fuzz-quick` runs each for 60 s).
- **Three packaging scaffolds** under `pkg/`: homebrew, docker
  (distroless + alpine), nix flake. Activates in v0.1.0 when
  `release-binaries.yml` lands.
- **33 / 33 CI checks green** on PR #111; coverage stays at
  100% (773 / 773 lines).

## What ships

### Long-form documentation (`doc/`)

- `ARCHITECTURE.md` - layered diagram, backend dispatch, Ziggurat
  tables, SIMD lane derivation, no_std story.
- `TESTING.md` - four-layer test strategy (unit / integration /
  proptest / doc), coverage discipline, PractRand harness, fuzz
  targets, CI matrix.
- `COMPARISON.md` - feature matrix vs `rand`, `fastrand`,
  `oorandom`, `nanorand`, `rand_xoshiro`, plus speed table on
  Apple Silicon.
- `POLICIES.md` - MSRV (1.70), SemVer rules, output-stability
  commitment (patch = bit-exact; minor may swap algorithm with
  changelog flag), concurrency, panic discipline, unsafe
  discipline, supply chain.
- `USER-GUIDE.md` - walkthrough of constructors, numbers, floats,
  distributions, bytes, slice ops, tokens, parallel streams,
  iterators, save/restore, quasi-random.
- `MIGRATION-FROM-RAND.md` - name-for-name mapping for rand 0.10
  idioms; distribution + token + dep-reduction notes.
- `MIGRATION-FROM-FASTRAND.md` - what you give up (sub-ns
  inline), what you gain (output stability, multi-backend, SIMD,
  crypto, quasi-random, distributions).
- `BENCHMARKS.md` moved from the root into `doc/`; root file
  becomes a 3-line stub.

### Governance and supply chain

- `SECURITY.md` - disclosure address, RNG threat model, supported-
  versions table, CSPRNG guidance, 14-day coordinated disclosure
  timeline.
- `GETTING_STARTED.md` - three-step onboarding.
- `GLOSSARY.md` - RNG terminology (backend, bounded sampling,
  CSPRNG, equidistribution, jump, lane, LCG, MSRV, no_std,
  output stability, PCG, period, PRNG, quasi-random, seed,
  Ziggurat).
- `PLAN.md` - multi-phase roadmap through v1.0.
- `rust-toolchain.toml` - stable channel pin + rustfmt + clippy.
- `REUSE.toml` - REUSE 3.0 annotations for files that can't
  carry inline SPDX headers.
- `about.toml` + `about.hbs` - cargo-about pipeline producing
  `NOTICE`. `make notice` generates it from the current
  dependency tree.
- `codecov.yml` - mirrors `.tarpaulin.toml` excludes;
  `project.target: auto` with 1% threshold; `patch.target:
  100%`.

### Tooling and CI

- `Makefile` rewritten as a 30-target POSIX file (tab-indented,
  `.PHONY`-complete, one or two lines per target). Default
  `all = check + clippy + test`. New `examples` target loops
  every example with `[ok]/[fail]`.
- `scripts/coverage-gap-report.sh`, `scripts/msrv.sh`,
  `scripts/pgo.sh`, `scripts/crush.sh` (PractRand wrapper).
- `.github/workflows/docs.yml` rewritten as a two-job
  build-and-deploy pipeline.
- `.github/workflows/scorecard.yml` - weekly OpenSSF Security
  Scorecard.
- `.github/workflows/security.yml` - Trivy filesystem scan
  (CRITICAL + HIGH only; non-gating). Action pinned to a SHA
  for supply-chain hygiene.
- GitHub repo description and 20 topics refreshed to call out
  the v0.0.11 backends (csprng, pcg, chacha20, simd,
  quasi-random, halton, sobol, ziggurat, webassembly).

### Fuzzing (`fuzz/`)

Six cargo-fuzz targets, local-only:

- `fuzz_xoshiro_seed` / `fuzz_mt_seed` / `fuzz_pcg_seed` - drive
  each backend's seed path on arbitrary 32-byte inputs, then
  64 draws.
- `fuzz_fill_array` - fuzzes length × seed on
  `Random::bytes(len)` + `fill_array::<64>()`.
- `fuzz_uniform_range` - `Random::uniform(low, high)` on
  arbitrary finite f64 pairs; asserts output is finite and
  in `[low, high)`.
- `fuzz_quasirandom` - Halton / Sobol / VdC on arbitrary dim +
  skip counts; asserts every point lands in `[0, 1)^D`.

`make fuzz-quick` runs each for 60 s; `make fuzz-deep` for 24 h.

### Packaging (`pkg/`)

Subset of noyalib's 11-channel matrix (the rest defer to v0.1.0
binary distribution):

- `pkg/homebrew/vrd.rb` - formula template with `__VERSION__` +
  `__SHA256__` placeholders.
- `pkg/docker/Dockerfile` - distroless multi-stage build (~10 MB).
- `pkg/docker/Dockerfile.alpine` - musl-static alternative (~3 MB).
- `pkg/nix/flake.nix` + `pkg/nix/package.nix` - flake exposing the
  `vrd` CLI binary plus a dev shell.
- `pkg/README.md`, `pkg/PUBLISH.md`, `pkg/VERIFY.md` - channel
  matrix, runbook, verification cookbook.

### Coverage and validation

- `cargo doc --no-deps --all-features` runs in CI on every PR
  with `-D warnings`; catches broken intra-doc links and the
  missing-docs lints.
- `cargo clippy --all-features --no-deps -- -W
  clippy::missing_docs_in_private_items` → 0 warnings.
- Line coverage stays at 100% (773 / 773 lines on every
  measured file).
- `examples/distribution.rs` closes the trait-coverage gap
  from issue #84 (built-in + user-defined Bernoulli).

## Breaking changes

None. v0.0.12 is a drop-in upgrade from v0.0.11.

## Style

- Em dashes (`-`) throughout the codebase and documentation
  replaced with hyphens for a more direct prose style.

## Documentation

- [`README.md`](README.md) - install, quick start, choosing a
  backend, feature flags, examples, FAQ.
- [`CHANGELOG.md`](CHANGELOG.md) - full per-version diff.
- [`doc/BENCHMARKS.md`](doc/BENCHMARKS.md) - performance evidence
  with per-bench commands to reproduce.
- [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) - design rationale
  for every backend.
- [`PLAN.md`](PLAN.md) - multi-phase roadmap (Phase 3 complete
  with this release; Phase 4 = v0.0.13).
- Issue tracker: https://github.com/sebastienrousseau/vrd/issues
- API docs (rendered from this release):
  https://doc.vrdlib.com/vrd/.

## Verification

This is a library-only release; no signed binaries ship yet.
crates.io: https://crates.io/crates/vrd/0.0.12.

## What's next

The v0.0.13 milestone collects post-launch bug reports plus the
SIMD AVX2 4-lane micro-optimisation that v0.0.11's headline
numbers left on the table. v0.0.14 starts expanding the
distribution catalogue (Binomial, Gamma, Beta, Log-normal,
Cauchy). See `PLAN.md` for the full sequencing.

---

vrd · Sebastien Rousseau · <https://vrdlib.com>

<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Glossary

Domain terminology used across vrd's docs, source comments,
and changelog. Listed alphabetically.

### Backend

A concrete random-number generator algorithm wrapped by the
`Random` facade. vrd ships four: Xoshiro256++ (default),
MT19937, PCG32 / PCG64, ChaCha20. Each is selected by a
`RngBackend` enum variant.

### Bounded sampling

Drawing an integer constrained to a half-open or inclusive
range. vrd's `int(lo, hi)`, `uint(lo, hi)`, `random_range(lo,
hi)`, and `bounded(range)` all use Lemire's nearly-divisionless
method to avoid modulo bias.

### CSPRNG (Cryptographically Secure PRNG)

An RNG whose output is computationally indistinguishable from
true random bytes to a polynomial-time observer who doesn't
see the seed. vrd's ChaCha20 backend is a CSPRNG; the others
(Xoshiro, MT, PCG) are not. Enable the `crypto` feature and
construct via `Random::new_secure()` for crypto-grade output.

### Determinism

Same seed → same sequence forever. All vrd backends are
deterministic. The output-stability commitment in
[`doc/POLICIES.md`](doc/POLICIES.md) pins this across patch
releases.

### Discrepancy

A measure of how unevenly a sequence of points covers `[0,1)^D`.
Low-discrepancy sequences (Halton, Sobol, Van der Corput) cover
the unit cube more evenly than a uniform PRNG, giving Monte
Carlo integration variance that scales `O((log n)^d / n)`
rather than `O(1/√n)`. Available under `--features quasirandom`.

### Entropy

Random bits sourced from a physical or operating-system pool
(e.g., `/dev/urandom`, `getrandom(2)`). vrd's `Random::new()`
seeds from the OS entropy source on `std` targets; pure no_std
targets must seed manually.

### Equidistribution

A statistical property: a sequence is k-equidistributed if
every k-tuple of consecutive outputs is uniformly distributed.
MT19937 is 623-equidistributed; Xoshiro256++ is 4-equidistributed.
Higher numbers indicate stronger uniformity guarantees.

### Fast path

The common branch of a sampler that handles the typical case
in the fewest instructions. In Ziggurat's `normal()`, the fast
path is one `u32` draw + one table lookup + one `f64` multiply
(~99 % of calls). In `xoshiro_simd::fill_bytes`, the fast path
is the inner SIMD loop.

### `fill_bytes`

The bulk byte-generation API: takes a `&mut [u8]` and writes
random bytes into it. vrd's default scalar path is one
Xoshiro `next_u64` per 8 bytes; with `--features simd`, the
SIMD path uses K independent lanes per call.

### Jump

A constant-time advance of an RNG's state by 2^n steps without
generating each intermediate value. Xoshiro256++ provides
`jump()` (2¹²⁸ steps) and `long_jump()` (2¹⁹² steps). vrd uses
`jump()` internally for `Random::split()`.

### Lane

In a SIMD generator, one of K parallel RNG states held in a
vector register. vrd's `xoshiro_simd` holds K=2 lanes on
AArch64 NEON and K=4 lanes on x86_64 AVX2.

### LCG (Linear Congruential Generator)

A simple RNG family of the form `state = state * a + c`. PCG
is a permutation-augmented LCG. `fastrand`'s Wyrand is also
in this family. Generally fast but with mediocre statistical
quality unless permuted (PCG) or combined with other steps.

### MSRV (Minimum Supported Rust Version)

The oldest Rust toolchain vrd commits to compile under.
Currently 1.70. Verified by `make msrv`. See
[`doc/POLICIES.md`](doc/POLICIES.md).

### MT19937 (Mersenne Twister)

A widely-used non-crypto RNG with period 2¹⁹⁹³⁷ - 1 and
623-equidistribution. Standard in NumPy, older `rand`, SciPy,
MATLAB. vrd ships MT19937 specifically for reproducibility
against existing MT-generated test vectors. Reach for
`Random::new_mersenne_twister()`.

### `no_std`

A Rust crate that compiles without the standard library. vrd
supports pure no_std (`default-features = false`) for embedded
targets like Cortex-M. The `alloc` feature unlocks `Vec` /
`String` / `Box`-backed APIs while staying no_std.

### Output stability

The commitment that a given seed produces the same output
forever (within bounds defined in
[`doc/POLICIES.md`](doc/POLICIES.md)). vrd commits to bit-exact
output across patch releases; `rand` explicitly does not.

### PCG (Permuted Congruential Generator)

O'Neill's RNG family that takes an LCG state and applies a
permutation (XOR-shift + random-rotate) to produce the output.
Statistically excellent (passes TestU01 BigCrush), small state
(16 B for PCG32, 32 B for PCG64). Available under
`--features pcg`.

### Period

The number of distinct outputs an RNG produces before its
state cycles. Xoshiro256++: 2²⁵⁶ - 1. MT19937: 2¹⁹⁹³⁷ - 1.
PCG32: 2⁶⁴ (per stream). All comfortably beyond any single
application's lifetime usage.

### PRNG (Pseudo-Random Number Generator)

A deterministic algorithm that produces a sequence indistinguishable
from random for most non-adversarial purposes. All vrd
backends are PRNGs. The ChaCha20 backend is also a CSPRNG.

### Quasi-random

A sequence of points designed to cover the unit cube
`[0, 1)^D` evenly, *not* to look random. Used for Monte Carlo
integration where the smooth coverage gives faster convergence
than uniform random points. vrd ships Halton, Sobol, and Van
der Corput under `--features quasirandom`.

### Seed

The initial state of an RNG. Same seed → same sequence (for
deterministic RNGs). vrd accepts seeds as `[u8; 32]`
(`from_seed`), `u64` (`from_u64_seed`), `u32` (the legacy
`seed()` method on `Random`), or `u128` (PCG64 / ChaCha20 via
their typed constructors).

### Ziggurat

A rejection-sampling algorithm for the normal distribution.
Uses precomputed lookup tables (`K`, `W`, `F`) of strip
heights and widths to avoid trigonometric functions in the
hot path. vrd's tables are generated at build time by
`build.rs`. See [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md).

### `splittable`

Property of an RNG where you can derive an independent child
stream from a parent without depleting the parent's randomness.
`Random::split()` uses Xoshiro's `jump()` to produce a child
2¹²⁸ calls ahead — both halves remain valid forever.

### Tarpaulin

Coverage tool (`cargo tarpaulin`). vrd uses it via
`make coverage` and the Codecov CI gate. Two files are
excluded via `.tarpaulin.toml`:
`src/xoshiro_simd.rs` (architecture-conditional) and
`src/float_libm.rs` (`no_std`-only).

### TestU01

A statistical test battery for RNGs from L'Ecuyer & Simard
(2007). Includes SmallCrush, Crush, and BigCrush. vrd's
backends pass the published TestU01 results for their
respective algorithms; we don't run TestU01 in CI (BigCrush
takes hours) — see PractRand below for the per-PR proxy.

### PractRand

A modern statistical test battery for non-crypto PRNGs
(http://pracrand.sourceforge.net/). vrd's
`examples/crush.rs` pipes each backend through PractRand's
`RNG_test` and reports pass / anomaly / failure counts. Run
via `make crush` (needs `RNG_test` in PATH).

<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Feature comparison vs other Rust RNG crates

vrd vs the most commonly reached-for non-crypto and crypto RNG
crates in the 2026 Rust ecosystem. Rows that vrd uniquely
ships are highlighted with **bold**.

|  | `vrd` | `rand` 0.10 | `fastrand` 2.x | `oorandom` 11.x | `nanorand` 0.7 | `rand_xoshiro` 0.7 |
| :-- | :-: | :-: | :-: | :-: | :-: | :-: |
| **Single-crate tri-backend** (Xoshiro / MT / PCG) | **✓** | - | - | - | - | - |
| ChaCha20 CSPRNG path | ✓ (`crypto` feature) | ✓ (`rand_chacha`) | - | - | - | - |
| MT19937 backend | ✓ (built-in) | external (`rand_mt`) | - | - | - | - |
| PCG32 / PCG64 | ✓ (`pcg` feature) | external (`pcg_rand`) | - | ✓ (default) | - | - |
| Xoshiro256++ | ✓ (default) | external (`rand_xoshiro`) | - | - | - | ✓ (only) |
| **SIMD bulk-byte path** | **✓** (`simd`) | - | - | - | - | - |
| **Quasi-random (Halton, Sobol, VdC)** | **✓** (`quasirandom`) | - | - | - | - | - |
| **`Distribution<T>` trait + built-ins** | **✓** | ✓ (via `rand_distr`) | - | - | - | - |
| Ziggurat normal | ✓ (built-in) | ✓ (`rand_distr`) | - | - | - | - |
| Built-in `uuid_v4` / `uuid_v4_bytes` | **✓** | needs `uuid` | - | - | - | - |
| Built-in `hex_token` / `base64_token` | **✓** | needs `hex` + `base64` | - | - | - | - |
| Pure `no_std` core | ✓ | partial | ✓ | ✓ | ✓ | ✓ |
| `alloc`-gated extras | ✓ | ✓ | - | - | ✓ | - |
| Cortex-M + WASM CI gated | ✓ | - | - | - | - | - |
| Unbiased bounded sampling (Lemire) | ✓ | ✓ | ✓ | - | ✓ | - |
| Bit-precise floats (24 / 53 bit) | ✓ | ✓ | partial | ✓ | ✓ | ✓ |
| Output stability across patches | ✓ | ✗ (explicit) | ✗ | partial | ✗ | ✗ |
| `rand 0.10` traits | ✓ | (native) | - | - | partial | ✓ |
| Iterator adapters (`iter_u32`, `iter_bytes`, ...) | **✓** | partial | - | - | - | - |
| `split()` for parallel streams | ✓ (Xoshiro) | partial (`ReseedingRng`) | - | - | - | partial |
| `fill_array<const N>` | **✓** | - | - | - | - | - |
| Bit-exact `rand_chacha::ChaCha20Rng` output | ✓ (`crypto`) | ✓ (native) | - | - | - | - |
| External fuzzing harness | ✓ (cargo-fuzz, 6 targets) | partial | - | - | - | - |
| PractRand validation harness | ✓ (`crush` feature) | - | - | - | - | - |
| Built-in benchmarks | ✓ (criterion vs all of these) | partial | - | - | - | - |
| MSRV declared | 1.70 | varies | 1.61 | 1.36 | 1.51 | 1.70 |
| Audited supply chain | ✓ (`cargo deny`, `cargo audit`) | partial | partial | - | - | - |
| Coverage gate | **100 %** | partial | - | - | - | - |
| GitHub Security Scorecard | ✓ | ✓ | - | - | - | - |

## Speed (Apple Silicon, through facade where applicable)

| Operation | vrd (Xoshiro) | vrd (PCG32) | rand::rng() | fastrand |
| :-- | --: | --: | --: | --: |
| `u32` | 3.09 ns | **2.72 ns** | 4.05 ns | 0.60 ns (inline) |
| `u64` | 3.10 ns | 3.72 ns | 6.18 ns | 0.60 ns (inline) |
| `fill_1024_bytes` (scalar) | 138 ns | n/a | n/a | n/a |
| `fill_1024_bytes` (SIMD) | **62 ns** | n/a | n/a | n/a |
| `normal(0, 1)` (Ziggurat) | **3.7 ns** | n/a | n/a (rand_distr) | n/a |
| `Sobol(2)::next_point` | **1.07 ns** | n/a | n/a | n/a |

(fastrand's sub-nanosecond numbers come from being a thin `u64`
LCG without trait dispatch; vrd's numbers go through the
`Random` facade. Both are fair characterisations of how the
crate is normally consumed.)

## When each crate is the right choice

- **vrd** - one library that does tri-backend non-crypto +
  crypto-quality + quasi-random + SIMD bulk-bytes + UUIDs +
  tokens, with output-stability commitments. Reach for it when
  you want fewer transitive deps and a single audit boundary.
- **`rand` + `rand_distr`** - the full statistical-distribution
  catalogue (20+ distributions), every backend the ecosystem
  has produced, native `rand_chacha`. The bigger surface area
  is the price.
- **`fastrand`** - pure speed, zero abstractions. If you're
  filling huge buffers and don't care about anything else,
  fastrand wins.
- **`oorandom`** - tiny PCG-only crate, zero deps. Perfect for
  embedded scenarios where binary size dominates.
- **`nanorand`** - small surface, no `unsafe`, no_std-first.
- **`rand_xoshiro`** - just the Xoshiro family on top of
  `rand_core`. Smaller than `vrd` if you only need one
  algorithm and don't want the facade dispatch.

## What vrd doesn't ship today

- The 20+ distributions in `rand_distr` (vrd ships 4 + a
  pluggable trait). Tracked.
- Multi-target binary releases (`cargo binstall`-able CLI).
  Tracked under `pkg/`.
- `Send + Sync` blanket impl for `Random` - RNGs hold mutable
  state; the standard pattern is one RNG per thread.

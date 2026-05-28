<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/vrd/v1/logos/vrd.svg" alt="vrd logo" width="128" />
</p>

<h1 align="center">vrd</h1>

<p align="center">
  A lightweight, <code>no_std</code>-friendly random number
  generator for Rust — Xoshiro256++ by default, with optional
  Mersenne&nbsp;Twister, PCG, ChaCha20&nbsp;CSPRNG, SIMD
  bulk-byte, and quasi-random backends.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/vrd/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/vrd/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/vrd"><img src="https://img.shields.io/crates/v/vrd.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/vrd"><img src="https://img.shields.io/badge/docs.rs-vrd-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://lib.rs/crates/vrd"><img src="https://img.shields.io/badge/lib.rs-vrd-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
  <a href="https://api.securityscorecards.dev/projects/github.com/sebastienrousseau/vrd"><img src="https://api.securityscorecards.dev/projects/github.com/sebastienrousseau/vrd/badge" alt="OpenSSF Scorecard" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — Cargo, MSRV, targets
- [Quick Start](#quick-start) — entropy-seeded, deterministic, `no_std`
- [Choosing a backend](#choosing-a-backend) — Xoshiro / MT / PCG / ChaCha20

**Library reference**

- [Cargo features](#cargo-features) — feature-flag matrix
- [Library Usage](#library-usage) — common idioms
- [Statistical validation](#statistical-validation) — PractRand harness
- [Quasi-random sequences](#quasi-random-sequences) — Halton, Sobol, VdC
- [Examples](#examples) — runnable example index
- [How vrd compares](#how-vrd-compares) — feature matrix vs `rand`, `fastrand`, `oorandom`

**Operational**

- [When not to use vrd](#when-not-to-use-vrd) — limitations
- [Migrating from `rand`](#migrating-from-rand) — name-for-name mapping
- [Migrating from earlier `0.0.x`](#migrating-from-earlier-00x) — breaking changes
- [Development](#development) — make targets, CI, performance knobs
- [FAQ](#faq) — common questions
- [Documentation](#documentation) — extended reading
- [License](#license)

---

## Install

```bash
cargo add vrd
```

Or in `Cargo.toml`:

```toml
[dependencies]
vrd = "0.0.11"
```

Requires [Rust](https://rustup.rs/) 1.70.0 or later. Builds for macOS, Linux, Windows, `no_std` embedded targets (Cortex-M, `thumbv7em-none-eabihf`), and `wasm32-unknown-unknown` — all validated in CI.

---

## Quick Start

```rust
use vrd::Random;

fn main() {
    let mut rng = Random::new();              // entropy-seeded Xoshiro256++

    println!("u32:        {}", rng.rand());
    println!("u64:        {}", rng.u64());
    println!("[1, 100]:   {}", rng.int(1, 100));
    println!("[0.0, 1.0): {}", rng.float());
}
```

### Deterministic sequences

```rust
use vrd::Random;

let mut rng = Random::from_u64_seed(0xCAFE_BABE);
let a = rng.rand();
let b = rng.rand();
// Re-seed with the same value to reproduce.
```

### `no_std` embedded usage

```rust
use vrd::Random;

// Allocation-free; works on any target — including Cortex-M.
let mut rng = Random::from_seed([0x42u8; 32]);
let n = rng.rand();
```

### Mersenne Twister (legacy reproducibility)

```rust
use vrd::Random;

let mut mt = Random::new_mersenne_twister();   // alloc + std
let v = mt.rand();
```

### Crypto-quality randomness

```rust
# #[cfg(all(feature = "crypto", feature = "std"))]
# {
use vrd::Random;

let mut rng = Random::new_secure();            // ChaCha20, OS-seeded
let token = rng.base64_token(32);
# let _ = token;
# }
```

---

## Choosing a backend

Default `Random` is non-cryptographic Xoshiro256++. For credentials, session IDs, or anything an attacker would benefit from predicting, enable the `crypto` feature and construct via [`Random::new_secure`] (entropy-seeded) or [`Random::from_secure_seed`] (deterministic).

| Backend | Constructor | State | Speed (u32) | Crypto-quality? |
| :--- | :--- | ---: | ---: | :---: |
| **Xoshiro256++** | `Random::new()` | 32 B | 3.1 ns | no |
| **Mersenne Twister** | `Random::new_mersenne_twister()` | 2 488 B | 3.1 ns | no |
| **PCG32 / PCG64** | `Random::new_pcg32()` / `new_pcg64()` | 16 / 32 B | **2.7 ns** / 4.3 ns | no |
| **ChaCha20** | `Random::new_secure()` | ~256 B | ~9 ns | **yes** |

Times measured through the `Random` facade on Apple Silicon. PCG32 is the fastest backend; ChaCha20 is the only crypto-quality one.

[`Random::new_secure`]: https://docs.rs/vrd/latest/vrd/struct.Random.html#method.new_secure
[`Random::from_secure_seed`]: https://docs.rs/vrd/latest/vrd/struct.Random.html#method.from_secure_seed

---

## Cargo features

| Flag | Default? | What it does |
| :--- | :--- | :--- |
| `std` | yes | Entropy seeding via `rand::rng()`; `std::error::Error` impls. |
| `alloc` | via `std` | `Random::bytes`, `Random::string`, `Random::sample`, `Random::uuid_v4`, `Random::hex_token`, `Random::base64_token`, the heap-stored Mersenne Twister backend. |
| `serde` | no | `Serialize` / `Deserialize` derives for the public types. |
| `simd` | no | SIMD-batched `fill_bytes` on AArch64 NEON / x86_64 AVX2 (~2–3× on bulk byte fills). |
| `pcg` | no | PCG32 (16 B state) and PCG64 (32 B state) as additional backends. |
| `crypto` | no | ChaCha20 CSPRNG backend: `Random::new_secure()`, `Random::from_secure_seed`. |
| `quasirandom` | no | Halton / Sobol / Van der Corput low-discrepancy sequences for Monte Carlo. |
| `crush` | no | Optional PractRand validation example. Informational only. |

Disable defaults to ship into `no_std`:

```toml
vrd = { version = "0.0.11", default-features = false }            # core only
vrd = { version = "0.0.11", default-features = false, features = ["alloc"] }  # core + alloc
```

---

## Library Usage

Every public method is documented at [docs.rs/vrd](https://docs.rs/vrd) with a worked example. The handful you'll reach for first:

```rust
use vrd::Random;

# #[cfg(feature = "std")]
# {
let mut rng = Random::new();                  // entropy-seeded Xoshiro256++

let n: u32 = rng.rand();                      // any u32
let n: u64 = rng.u64();                       // any u64
let n      = rng.int(1, 100);                 // i32 in [1, 100], uniform
let n      = rng.uint(1, 100);                // u32 in [1, 100], uniform
let f      = rng.double();                    // f64 in [0.0, 1.0)
let b      = rng.bool(0.5);                   // 50/50 coin
let pick   = rng.choose(&[10, 20, 30]);       // Option<&T>

#[cfg(feature = "alloc")]
let buf    = rng.bytes(32);                   // Vec<u8>, 32 random bytes
# }
```

**Non-security tokens** (correlation IDs, log markers, debug fixtures):

```rust
# #[cfg(all(feature = "alloc", feature = "std"))]
# {
use vrd::Random;
let mut rng = Random::new();

let trace_id = rng.uuid_v4_bytes();           // [u8; 16], no_std
let trace_id = rng.uuid_v4();                 // RFC 4122 hyphenated, alloc
let log_id   = rng.hex_token(16);             // 32 lowercase hex chars
let csrf_id  = rng.base64_token(15);          // 20 URL-safe base64 chars (no padding)
# }
```

For **security-sensitive** tokens (API keys, session IDs, password-reset links, CSRF tokens), enable the `crypto` feature and use `Random::new_secure()`.

**Save and restore state** (with the `serde` feature):

```toml
vrd = { version = "0.0.11", features = ["serde"] }
```

```rust,ignore
use vrd::Random;

let mut rng = Random::from_u64_seed(42);
let snap = serde_json::to_string(&rng).unwrap();

let mut restored: Random = serde_json::from_str(&snap).unwrap();
assert_eq!(rng.rand(), restored.rand());      // identical state, identical output
```

**User-defined distributions** (with the built-in `Distribution<T>` trait):

```rust
use vrd::{Distribution, Random};

struct Bernoulli { p: f64 }

impl Distribution<bool> for Bernoulli {
    fn sample(&self, rng: &mut Random) -> bool {
        rng.double() < self.p
    }
}

let mut rng = Random::from_u64_seed(1);
let coin = Bernoulli { p: 0.5 }.sample(&mut rng);
# let _ = coin;
```

---

## Statistical validation

Beyond Xoshiro256++ and MT19937's published academic pedigree, vrd ships a reproducible PractRand harness. See [BENCHMARKS.md](BENCHMARKS.md) for the latest pass-count table per backend; reproduce locally with `cargo run --release --example crush --features crush`. The example is informational — CI does **not** gate on it.

---

## Quasi-random sequences

Low-discrepancy sequences for Monte Carlo integration, ray-tracing, and high-dimensional optimisation. Variance scales `O((log n)^d / n)` rather than `O(1/√n)` for a uniform PRNG. Behind the `quasirandom` feature:

```toml
vrd = { version = "0.0.11", features = ["quasirandom"] }
```

```rust
use vrd::quasirandom::{HaltonSequence, SobolSequence, VanDerCorputSequence};

// 1-D Van der Corput in base 2: 0.5, 0.25, 0.75, 0.125, ...
let mut vdc = VanDerCorputSequence::new(2);
let _ = vdc.next_point();

// 2-D Halton across primes (2, 3); up to 32 dimensions shipped.
let mut h = HaltonSequence::new(2);
let _ = h.next_point::<2>();

// 6-D Sobol with Bratley-Fox direction numbers; up to 6 dimensions
// shipped (extending past that needs the Joe-Kuo D6 table).
let mut s = SobolSequence::new(6);
let _ = s.next_point::<6>();
```

Three constructions cover the standard ground; see [`examples/halton.rs`](examples/halton.rs) and [`examples/sobol.rs`](examples/sobol.rs) for Monte Carlo π convergence demos.

---

## Examples

44 runnable examples under [`examples/`](examples/). Browse the orchestrator with `cargo run --example all`. Highlights:

| Topic | Example | Run |
|---|---|---|
| Hello / basics | `hello`, `basics`, `seed` | `cargo run --example basics` |
| Bytes / floats / bools | `bytes`, `floats`, `bools`, `chars`, `strings` | `cargo run --example bytes` |
| Bounded sampling | `bounded`, `unbiased` | `cargo run --example bounded` |
| Distributions | `normal`, `exponential`, `poisson`, `distribution` | `cargo run --example normal` |
| Slice ops | `choose`, `shuffle`, `sample`, `slice`, `weighted` | `cargo run --example shuffle` |
| Backends | `xoshiro`, `mersenne`, `pcg`, `secure`, `backends` | `cargo run --example backends --features pcg,crypto` |
| Quasi-random | `halton`, `sobol` | `cargo run --example sobol --features quasirandom` |
| Parallel | `split` | `cargo run --example split` |
| Apps | `dice`, `lottery`, `passwords`, `monte` | `cargo run --example monte` |
| Helpers | `iterators`, `uuid`, `tokens` | `cargo run --example uuid` |
| Validation | `crush` | `cargo run --release --example crush --features crush` |

---

## How vrd compares

|  | `vrd` | `rand` 0.10 | `fastrand` 2.x | `oorandom` 11.x |
| :-- | :-: | :-: | :-: | :-: |
| Default backend | Xoshiro256++ | ChaCha12 / SmallRng | Wyrand | PCG family |
| MT19937 backend | ✓ (built-in) | external (`rand_mt`) | — | — |
| PCG32 / PCG64 | ✓ (`pcg` feature) | external (`pcg_rand`) | — | ✓ (default) |
| ChaCha20 CSPRNG | ✓ (`crypto` feature) | ✓ (`rand_chacha`) | — | — |
| Quasi-random (Halton / Sobol / VdC) | ✓ (`quasirandom` feature) | — | — | — |
| Pluggable `Distribution<T>` trait | ✓ | ✓ (via `rand_distr`) | — | — |
| SIMD-batched `fill_bytes` | ✓ (`simd` feature) | — | — | — |
| Pure `no_std` core | ✓ | partial | ✓ | ✓ |
| Cortex-M + WASM CI gated | ✓ | — | — | — |
| Unbiased bounded sampling (Lemire) | ✓ | ✓ | ✓ | — |
| Bit-precise floats (24-bit `f32` / 53-bit `f64`) | ✓ | ✓ | partial | ✓ |
| Built-in `uuid_v4` / `uuid_v4_bytes` | ✓ | needs `uuid` | — | — |
| Built-in `hex_token` / `base64_token` | ✓ | needs `hex` + `base64` | — | — |
| Output stability commitment | ✓ (patch) | explicitly **none** | — | — |
| `rand` 0.10 traits (`TryRng`, `SeedableRng`) | ✓ | (native) | — | — |

**Reach for `vrd`** when you want one small crate that covers fast non-cryptographic RNG, MT19937 for legacy reproducibility, PCG for scientific computing, ChaCha20 for crypto-quality tokens, and quasi-random sequences for Monte Carlo — across `std`, `no_std + alloc`, embedded (Cortex-M), and WebAssembly — without stitching together five other crates.

**Reach for `rand` + `rand_distr`** when you need the full statistical-distribution catalogue or are already deep in the rand-ecosystem trait stack.

### What you don't have to depend on

Pulling `vrd` in instead of `rand` + companion crates typically lets you drop these from your dependency tree:

- `uuid` — covered by `Random::uuid_v4` / `uuid_v4_bytes`
- `hex` or `data-encoding` — covered by `Random::hex_token`
- `base64` — covered by `Random::base64_token`
- `rand_distr` — if `uniform` / `normal` / `exponential` / `poisson` cover your needs
- `rand_chacha` — covered by the `crypto` feature
- `pcg_rand` — covered by the `pcg` feature

Fewer transitive crates, less compiled code, fewer audit boundaries to track.

---

## When not to use vrd

- **You need the rand-distr catalogue.** vrd ships four built-in distributions (`uniform`, `normal`, `exponential`, `poisson`) plus a pluggable `Distribution<T>` trait. If you need the full 20+ rand_distr set (binomial, gamma, log-normal, Cauchy, etc.) reach for `rand` + `rand_distr` until vrd ports the missing few.
- **You need fork-safe parallel deterministic streams beyond Xoshiro.** `Random::split()` works on the default backend; the MT / PCG / ChaCha20 backends return `None`. For massive Rayon-style fan-out across non-Xoshiro backends, derive distinct seeds per worker.
- **You're already deep in the rand-ecosystem trait stack.** vrd implements rand 0.10's `TryRng` / `Rng` / `SeedableRng`, so it interoperates — but if your codebase passes `rand::rngs::ThreadRng` everywhere, the migration cost may not be worth it.
- **You need cryptographic randomness without enabling a feature flag.** vrd's default backend is non-cryptographic. Enable the `crypto` feature and use `Random::new_secure()` for CSPRNG output.

---

## Migrating from `rand`

`vrd` implements the rand 0.10 traits, so most idioms translate directly:

| `rand 0.10` | `vrd` equivalent |
| :-- | :-- |
| `let mut rng = rand::rng();` | `let mut rng = Random::new();` |
| `rng.random::<u32>()` | `rng.rand()` |
| `rng.random_range(0..n)` | `rng.uint(0, n - 1)` |
| `rng.fill_bytes(&mut buf)` | `rng.try_fill_bytes(&mut buf).unwrap()` |
| `slice.choose(&mut rng)` | `rng.choose(slice)` |
| `slice.shuffle(&mut rng)` (`alloc`) | `rng.shuffle(slice)` (`alloc`) |
| `rand::rngs::StdRng::seed_from_u64(s)` | `Random::from_u64_seed(s)` |
| `rand_chacha::ChaCha20Rng::from_seed(s)` | `Random::from_secure_seed(s)` (`crypto`) |
| `rand_chacha::ChaCha20Rng::from_os_rng()` | `Random::new_secure()` (`crypto`) |

Or pass a `Random` directly to any crate that takes a `rand_core::TryRng`, `Rng`, or `SeedableRng` — vrd implements all three.

---

## Migrating from earlier `0.0.x`

The 0.0.10 release modernised the architecture. Breaking changes carried into 0.0.11:

- `Random` defaults to **Xoshiro256++**, not Mersenne Twister. Use `Random::new_mersenne_twister()` if you need MT.
- The generic `fill()` method is gone — use `Random::try_fill_bytes(&mut [u8])` from the `rand_core::TryRng` trait, or build types from `rand()` / `u64()`.
- `int`, `uint`, `random_range` are **unbiased** (Lemire's nearly-divisionless method). Outputs differ from pre-0.0.10 versions for the same seed.
- `MersenneTwisterError` lost its `IoError` and `SerializationError` variants — direct `serde_json` / `serde_yml` / `toml` helpers were removed. Use `serde` directly with the `serde` feature for that.
- `VrdError::GeneralError` carries `&'static str` instead of `String` — `no_std`-friendly.
- The `logging` feature and `create_log_entry` helper are gone — vrd is no longer a log-formatting library.

0.0.11-specific:

- `Random` and `RngBackend` no longer derive `Eq` / `Hash` / `Ord` / `PartialOrd`. `PartialEq` is kept for snapshot / determinism comparisons. The drop is needed because `ChaCha20Rng` (the new `crypto` backend's underlying type) doesn't implement them.
- `#![forbid(unsafe_code)]` becomes `#![deny(unsafe_code)]` at the crate root so the optional `xoshiro_simd` module can lift the deny locally for architecture intrinsics. All other modules remain `unsafe`-free.

See [CHANGELOG.md](CHANGELOG.md) and [RELEASE-NOTES-v0.0.11.md](RELEASE-NOTES-v0.0.11.md) for the full diff.

---

## Development

```bash
make                                                                 # check + clippy + test (default)
make test                                                            # cargo test --all-features
make bench                                                           # cargo bench
make doc                                                             # cargo doc --no-deps --all-features
make examples                                                        # run every example with [ok]/[fail]
make crush                                                           # PractRand harness (needs RNG_test in PATH)
make fuzz-quick                                                      # 60 s per fuzz target (cargo-fuzz)
cargo check --target thumbv7em-none-eabihf --no-default-features     # Cortex-M smoke check
cargo check --target wasm32-unknown-unknown --no-default-features    # WebAssembly smoke check
```

### Squeezing more performance

The default release profile (`opt-level = 3`, `lto = true`, `codegen-units = 1`) gets vrd to ~1.1 ns per `u32` on Apple Silicon. Three knobs are available to downstream consumers who want every cycle:

**Native CPU targeting** — enables AArch64 NEON or x86 AVX/AVX-512 codegen for whichever host you're running on:

```toml
# .cargo/config.toml in your binary crate
[build]
rustflags = ["-C", "target-cpu=native"]
```

`target-cpu=native` is **not** baked into vrd's release profile because it would break `cargo install` for users on machines that download crates as binaries. Set it in the consuming crate.

**`simd` feature for bulk byte generation** — opts into SIMD-batched `fill_bytes` that holds K independent Xoshiro256++ states in vector registers (K = 2 on AArch64 NEON, K = 4 on x86_64 AVX2). ~2.2× faster on 1 KiB and ~3× on 16 KiB:

```toml
vrd = { version = "0.0.11", features = ["simd"] }
```

The SIMD path produces a **different byte stream** than the scalar path for the same seed — see [`xoshiro_simd`](https://docs.rs/vrd/latest/vrd/xoshiro_simd/) for the contract. Reproducibility-sensitive code must stay on the scalar default.

**Profile-Guided Optimization (PGO)** — typically yields 5–15% on hot loops:

```bash
# 1. Instrumented build that emits .profraw counters
RUSTFLAGS="-Cprofile-generate=/tmp/pgo" cargo build --release
# 2. Run a representative workload to populate the profile
./target/release/your-app
# 3. Merge into a single .profdata
$(rustc --print sysroot)/lib/rustlib/*/bin/llvm-profdata merge -o /tmp/pgo/merged.profdata /tmp/pgo
# 4. Rebuild with the profile applied
RUSTFLAGS="-Cprofile-use=/tmp/pgo/merged.profdata" cargo build --release
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, signed commits, and PR guidelines.

---

## FAQ

### Do I need one RNG per thread?

Yes. `Random` (and `Xoshiro256PlusPlus`, and `MersenneTwister`) hold mutable state and are not designed for concurrent access. The standard pattern is one RNG per thread, seeded distinctly:

```rust
use vrd::Random;

# let thread_id: u64 = 0;
let mut rng = Random::from_u64_seed(thread_id);   // distinct per thread
let _ = rng.rand();
```

For parallel deterministic streams that don't drift, use `Random::split()` on the default Xoshiro backend — returns `Some(Random)` with a stream 2¹²⁸ calls ahead.

### Does vrd work without `std`?

Yes. With `default-features = false`, vrd compiles for pure `no_std` targets — Cortex-M is gated in CI on every PR. The `alloc` feature unlocks `Vec` / `String` / `Box`-backed APIs (`bytes`, `string`, `sample`, `shuffle`, `uuid_v4`, `hex_token`, `base64_token`, the Mersenne Twister backend). Without `alloc`, `Random::from_seed([u8; 32])` and `Random::from_u64_seed(u64)` give you a fully-functional Xoshiro256++ on bare metal.

### Does vrd work in WebAssembly?

Yes. `wasm32-unknown-unknown` is gated in CI under both `--no-default-features` and `--features alloc`. Default WebAssembly has no entropy source, so seed manually with `Random::from_seed([u8; 32])` or `Random::from_u64_seed(u64)` rather than `Random::new()`. If you want OS-level entropy in the browser, enable `getrandom`'s `js` feature in your binary crate — that's downstream's choice, not vrd's.

### Why ship Mersenne Twister at all if Xoshiro is the default?

Reproducibility against existing MT-generated test vectors. Numerical-simulation pipelines, scientific software, and tooling that emits "random-looking" reference data often pin MT19937 because that's what NumPy / older `rand` / SciPy / MATLAB historically used. Reach for `Random::new_mersenne_twister()` (or `new_mersenne_twister_with_seed(u32)` for `alloc`-only) only when you need bit-for-bit MT19937 output.

### Can I get the same sequence on two machines?

Yes — use `Random::from_seed([u8; 32])` or `Random::from_u64_seed(u64)`. Both are deterministic and allocation-free. The output is byte-identical across architectures (x86, ARM, RISC-V, WebAssembly) — only floating-point operations *downstream* of the RNG (your code's arithmetic) may differ across targets.

### Is the output stable across vrd versions?

For a given seed and method, vrd commits to bit-stable output across **patch** releases. Algorithm changes (e.g., the v0.0.11 Ziggurat `normal()`) bump at least the minor version and are flagged in the `CHANGELOG`'s `Migration` section, naming the affected methods. Once vrd reaches 1.0, this stability commitment will extend to minor releases as well. The `rand` crate explicitly does not guarantee either. If you have golden-file tests, fuzzing corpora, or reproducible-research workflows depending on a stable RNG sequence, that's a meaningful difference.

### How big is the RNG state?

- `Xoshiro256PlusPlus`: **32 bytes** (four `u64` words). Stored inline.
- `Pcg32`: **16 bytes** (state + increment).
- `Pcg64`: **32 bytes** (state + increment).
- `MersenneTwister`: **~2.5 KB** (624 × `u32` + index). Heap-stored when wrapped in `Random` to keep the enum discriminant small.
- `ChaCha20Rng` (via `crypto` feature): **~256 bytes**. Heap-stored in `Random`.
- `Random`: a tagged enum sized for the largest inline variant. The wrapper-vs-direct dispatch overhead is **zero** — the inliner elides the match completely (verified in `cargo bench`).

### How fast is it?

`cargo bench` runs head-to-head against `fastrand` 2.x and `rand::rng()` on `u32`, `u64`, byte fills, bounded sampling, and distribution sampling. On Apple Silicon, Xoshiro vrd produces a `u32` in ~3.1 ns through the `Random` facade (~1.1 ns inline); PCG32 is ~2.7 ns; the Ziggurat `normal()` is ~3.7 ns. Run them locally — absolute numbers are workload- and platform-dependent.

---

## Documentation

- [`README.md`](README.md) — this file.
- [`CHANGELOG.md`](CHANGELOG.md) — per-version diff.
- [`BENCHMARKS.md`](BENCHMARKS.md) — full benchmark tables.
- [`RELEASE-NOTES-v0.0.11.md`](RELEASE-NOTES-v0.0.11.md) — current release notes.
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — pre-submit checklist and PractRand install pointers.
- [`docs.rs/vrd`](https://docs.rs/vrd) — full API documentation, every public method worked.

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#vrd">Back to Top</a></p>

<p align="center">
  <img src="https://cloudcdn.pro/vrd/v1/logos/vrd.svg" alt="vrd logo" width="128" />
</p>

<h1 align="center">Versatile Random Distributions (VRD)</h1>

<p align="center">
  <strong>A lightweight, <code>no_std</code>-friendly random number generator backed by Xoshiro256++, with optional Mersenne Twister support.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/vrd/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/vrd/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/vrd"><img src="https://img.shields.io/crates/v/vrd.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/vrd"><img src="https://img.shields.io/badge/docs.rs-vrd-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/vrd"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/vrd?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://lib.rs/crates/vrd"><img src="https://img.shields.io/badge/lib.rs-v0.0.10-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
</p>

---

## Install

```bash
cargo add vrd
```

Or in `Cargo.toml`:

```toml
[dependencies]
vrd = "0.0.10"
```

Requires [Rust](https://rustup.rs/) 1.70.0 or later. Builds for macOS, Linux, Windows, `no_std` embedded targets (Cortex-M, `thumbv7em-none-eabihf`), and `wasm32-unknown-unknown` — all validated in CI.

---

## Highlights

- **Xoshiro256++ default** — 32-byte state, 2^256 - 1 period, high statistical quality, faster than MT19937 in practice.
- **Mersenne Twister opt-in** — keep MT19937 for legacy reproducibility. `Random::new_mersenne_twister()` (entropy-seeded, requires `alloc + std`) or `Random::new_mersenne_twister_with_seed(u32)` (deterministic, `alloc` only).
- **`no_std` ready** — pure-core build with no allocator: `Random::from_seed([u8; 32])` gives you a working RNG on any embedded target.
- **Unbiased bounded sampling** — `int`, `uint`, `random_range`, `bounded` use Lemire's nearly-divisionless method, not modulo.
- **Bit-precise floats** — `float()` carries 24 mantissa bits (the f32 maximum); `double()` / `f64()` carry 53 (the f64 maximum). Always `[0.0, 1.0)`.
- **Distributions** — `uniform(low, high)`, `normal`, `exponential`, `poisson` (`std`-free, via `libm`).
- **Convenience helpers** — `iter_u32` / `iter_u64` / `iter_bytes` iterator adapters, `uuid_v4_bytes` (no_std) and `uuid_v4` (alloc), `hex_token`, `base64_token`. fastrand and oorandom don't ship these; they spare callers reaching for a second crate.
- **`rand 0.10` traits** — implements `TryRng` (and the auto-derived `Rng`) plus `SeedableRng`, so vrd plugs into the wider `rand` ecosystem.

## Feature flags

| Flag | Default? | What it does |
| :--- | :--- | :--- |
| `std` | yes | Entropy seeding via `rand::rng()`; `std::error::Error` impls. |
| `alloc` | via `std` | `Random::bytes`, `Random::string`, `Random::sample`, `Random::uuid_v4`, `Random::hex_token`, `Random::base64_token`, the heap-stored Mersenne Twister backend. |
| `serde` | no | `Serialize` / `Deserialize` derives for the public types. |

Disable defaults to ship into `no_std`:

```toml
vrd = { version = "0.0.10", default-features = false }            # core only
vrd = { version = "0.0.10", default-features = false, features = ["alloc"] }  # core + alloc
```

---

## Quickstart

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

---

## Migrating from earlier 0.0.x

The 0.0.10 release modernizes the architecture. Breaking changes:

- `Random` now defaults to **Xoshiro256++**, not Mersenne Twister. Use `Random::new_mersenne_twister()` if you need MT.
- The generic `fill()` method is gone — use `Random::try_fill_bytes(&mut [u8])` from the `rand_core::TryRng` trait, or build types from `rand()` / `u64()`.
- `int`, `uint`, `random_range` are now **unbiased** — outputs are uniformly distributed even when the requested range doesn't divide `2^32` cleanly. Outputs differ from prior versions for the same seed.
- `MersenneTwisterError` lost its `IoError` and `SerializationError` variants — direct `serde_json` / `serde_yml` / `toml` helpers were removed. Use `serde` directly with the `serde` feature for that.
- `VrdError::GeneralError` now carries `&'static str` instead of `String` — `no_std`-friendly.
- The `logging` feature and `create_log_entry` helper are gone — vrd is no longer a log-formatting library.

See [CHANGELOG.md](CHANGELOG.md) for the full diff.

---

## Development

```bash
cargo build                                                          # default features (std)
cargo build --no-default-features                                    # pure no_std
cargo build --no-default-features --features alloc                   # no_std + alloc
cargo test --all-features                                            # all tests
cargo clippy --all-targets --all-features -- -D warnings             # lint clean
cargo bench                                                          # comparative criterion benches
cargo check --target thumbv7em-none-eabihf --no-default-features     # Cortex-M smoke check
cargo check --target wasm32-unknown-unknown --no-default-features    # WebAssembly smoke check
cargo run --example all                                              # run every demo in examples/
```

### Squeezing more performance

The default release profile (`opt-level = 3`, `lto = true`, `codegen-units = 1`) gets vrd to ~1.1 ns per `u32` on Apple Silicon. Two extra knobs are available to downstream consumers who want every cycle:

**Native CPU targeting** — enables AArch64 NEON or x86 AVX/AVX-512 codegen for whichever host you're running on:

```toml
# .cargo/config.toml in your binary crate
[build]
rustflags = ["-C", "target-cpu=native"]
```

`target-cpu=native` is **not** baked into vrd's release profile because it would break `cargo install` for users on machines that download crates as binaries. Set it in the consuming crate.

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

## How `vrd` compares

|  | `vrd` | `rand` 0.10 | `fastrand` 2.x | `oorandom` 11.x |
| :-- | :-: | :-: | :-: | :-: |
| Default backend | Xoshiro256++ | ChaCha12 / SmallRng | Wyrand | PCG family |
| MT19937 backend | ✓ (built-in) | external (`rand_mt`) | — | — |
| Pure `no_std` core | ✓ | partial | ✓ | ✓ |
| Cortex-M + WASM CI gated | ✓ | — | — | — |
| Unbiased bounded sampling (Lemire) | ✓ | ✓ | ✓ | — |
| Bit-precise floats (24-bit `f32` / 53-bit `f64`) | ✓ | ✓ | partial | ✓ |
| Built-in `uuid_v4` / `uuid_v4_bytes` | ✓ | needs `uuid` | — | — |
| Built-in `hex_token` / `base64_token` | ✓ | needs `hex` + `base64` | — | — |
| Output stability commitment (patch versions) | ✓ | explicitly **none** | — | — |
| `rand` 0.10 traits (`TryRng`, `SeedableRng`) | ✓ | (native) | — | — |
| CSPRNG path | planned ([#90](https://github.com/sebastienrousseau/vrd/issues/90)) | ✓ (`OsRng`, `ChaCha20Rng`) | — | — |
| Distribution catalogue | 4 (built-in) | 20+ via `rand_distr` | — | — |

**Reach for `vrd`** when you want a single small crate that gives you fast non-cryptographic randomness, MT19937 for legacy reproducibility, UUIDs, and URL-safe tokens — across `std`, `no_std + alloc`, embedded (Cortex-M), and WebAssembly — without building a CSPRNG into your binary.

**Reach for `rand` + `rand_distr`** when you need cryptographically secure randomness today, or the full statistical-distribution catalogue.

### What you don't have to depend on

Pulling `vrd` in instead of `rand` + companion crates typically lets you drop these from your dependency tree:

- `uuid` — covered by `Random::uuid_v4` / `uuid_v4_bytes`
- `hex` or `data-encoding` — covered by `Random::hex_token`
- `base64` — covered by `Random::base64_token`
- `rand_distr` — if `uniform` / `normal` / `exponential` / `poisson` cover your needs

Fewer transitive crates, less compiled code, fewer audit boundaries to track.

---

## FAQ

### Which methods will I use most often?

```rust
use vrd::Random;

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
```

Every public method is documented at [docs.rs/vrd](https://docs.rs/vrd) with a worked example.

### How do I migrate from `rand`?

`vrd` implements the `rand 0.10` traits, so most idioms translate directly:

| `rand 0.10` | `vrd` equivalent |
| :-- | :-- |
| `let mut rng = rand::rng();` | `let mut rng = Random::new();` |
| `rng.random::<u32>()` | `rng.rand()` |
| `rng.random_range(0..n)` | `rng.uint(0, n - 1)` |
| `rng.fill_bytes(&mut buf)` | `rng.try_fill_bytes(&mut buf).unwrap()` |
| `slice.choose(&mut rng)` | `rng.choose(slice)` |
| `slice.shuffle(&mut rng)` (`alloc`) | `rng.shuffle(slice)` (`alloc`) |
| `rand::rngs::StdRng::seed_from_u64(s)` | `Random::from_u64_seed(s)` |

Or pass a `Random` directly to any crate that takes a `rand_core::TryRng`, `Rng`, or `SeedableRng` — vrd implements all three.

### How do I generate non-security tokens (correlation IDs, log markers, debug fixtures)?

```rust
use vrd::Random;
let mut rng = Random::new();

let trace_id = rng.uuid_v4_bytes();           // [u8; 16], no_std
# #[cfg(feature = "alloc")]
# {
let trace_id = rng.uuid_v4();                 // RFC 4122 hyphenated, alloc
let log_id   = rng.hex_token(16);             // 32 lowercase hex chars
let csrf_id  = rng.base64_token(15);          // 20 URL-safe base64 chars (no padding)
# }
```

For **security-sensitive** tokens (API keys, session IDs, password-reset links, CSRF tokens), `vrd` is **not** the right tool. Use `rand::rngs::OsRng` or the `getrandom` crate — both produce CSPRNG-grade output backed by the OS entropy source.

### Do I need one RNG per thread?

Yes. `Random` (and `Xoshiro256PlusPlus`, and `MersenneTwister`) hold mutable state and are not designed for concurrent access. The standard pattern is one RNG per thread, seeded distinctly:

```rust
use vrd::Random;

# let thread_id: u64 = 0;
let mut rng = Random::from_u64_seed(thread_id);   // distinct per thread
let _ = rng.rand();
```

For parallel deterministic streams that don't drift, a forking `Random::split()` API is tracked in [#92](https://github.com/sebastienrousseau/vrd/issues/92).

### Can I save and restore RNG state?

Yes — enable the `serde` feature.

```toml
vrd = { version = "0.0.10", features = ["serde"] }
```

```rust,ignore
use vrd::Random;

let mut rng = Random::from_u64_seed(42);
let snap = serde_json::to_string(&rng).unwrap();

let mut restored: Random = serde_json::from_str(&snap).unwrap();
assert_eq!(rng.rand(), restored.rand());      // identical state, identical output
```

`Random`, `Xoshiro256PlusPlus`, `MersenneTwisterParams`, and `MersenneTwisterConfig` all derive `Serialize` / `Deserialize` under the `serde` feature.

### Is `vrd` cryptographically secure?
No. `Random` is a non-cryptographic PRNG built on Xoshiro256++. For credentials, secrets, session IDs, or anything that an attacker would benefit from predicting, use a CSPRNG such as `rand::rngs::OsRng` or the `getrandom` crate. A built-in ChaCha20-based CSPRNG backend is tracked in [#90](https://github.com/sebastienrousseau/vrd/issues/90).

### Does `vrd` work without `std`?
Yes. With `default-features = false`, `vrd` compiles for pure `no_std` targets — Cortex-M is gated in CI on every PR. The `alloc` feature unlocks `Vec`/`String`/`Box`-backed APIs (`bytes`, `string`, `sample`, `shuffle`, `uuid_v4`, `hex_token`, `base64_token`, the Mersenne Twister backend). Without `alloc`, `Random::from_seed([u8; 32])` and `Random::from_u64_seed(u64)` give you a fully-functional Xoshiro256++ on bare metal.

### Does `vrd` work in WebAssembly?
Yes. `wasm32-unknown-unknown` is gated in CI under both `--no-default-features` and `--features alloc`. Default WebAssembly has no entropy source, so seed manually with `Random::from_seed([u8; 32])` or `Random::from_u64_seed(u64)` rather than `Random::new()`. If you want OS-level entropy in the browser, enable `getrandom`'s `js` feature in your binary crate — that's downstream's choice, not vrd's.

### Why ship Mersenne Twister at all if Xoshiro is the default?
Reproducibility against existing MT-generated test vectors. Numerical-simulation pipelines, scientific software, and tooling that emits "random-looking" reference data often pin MT19937 because that's what NumPy / older `rand` / SciPy / MATLAB historically used. Reach for `Random::new_mersenne_twister()` (or `new_mersenne_twister_with_seed(u32)` for `alloc`-only) only when you need bit-for-bit MT19937 output.

### Can I get the same sequence on two machines?
Yes — use `Random::from_seed([u8; 32])` or `Random::from_u64_seed(u64)`. Both are deterministic and allocation-free. The output is byte-identical across architectures (x86, ARM, RISC-V, WebAssembly) — only floating-point operations *downstream* of the RNG (your code's arithmetic) may differ across targets.

### Is the output stable across vrd versions?
For a given seed and method, vrd commits to bit-stable output across **patch** releases. Algorithm changes (e.g., a faster `normal()` sampling method) bump at least the minor version and are flagged in the `CHANGELOG`'s `Migration` section, naming the affected methods. Once vrd reaches 1.0, this stability commitment will extend to minor releases as well. The `rand` crate explicitly does not guarantee either. If you have golden-file tests, fuzzing corpora, or reproducible-research workflows depending on a stable RNG sequence, that's a meaningful difference.

### How big is the RNG state?
- `Xoshiro256PlusPlus`: **32 bytes** (four `u64` words). Stored inline.
- `MersenneTwister`: **~2.5 KB** (624 × `u32` + index). Heap-stored when wrapped in `Random` to keep the enum discriminant small.
- `Random`: a tagged enum holding either `Xoshiro256PlusPlus` inline or `Box<MersenneTwister>`; sized for the Xoshiro variant. The wrapper-vs-direct dispatch overhead is **zero** — the inliner elides the match completely (verified in `cargo bench`).

### How fast is it?
`cargo bench` runs head-to-head against `fastrand` 2.x and `rand::rng()` on `u32`, `u64`, byte fills, bounded sampling, and distribution sampling. On Apple Silicon, Xoshiro vrd produces a `u32` in ~1.1 ns; the wrapper adds zero overhead vs the raw `Xoshiro256PlusPlus`. Run them locally — absolute numbers are workload- and platform-dependent.

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#versatile-random-distributions-vrd">Back to Top</a></p>

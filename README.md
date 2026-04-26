<p align="center">
  <img src="https://cloudcdn.pro/vrd/v1/logos/vrd.svg" alt="Random (VRD) logo" width="128" />
</p>

<h1 align="center">Random (VRD)</h1>

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

Requires [Rust](https://rustup.rs/) 1.70.0 or later. Builds for macOS, Linux, Windows, and `no_std` embedded targets (Cortex-M validated in CI).

---

## Highlights

- **Xoshiro256++ default** — 32-byte state, 2^256 - 1 period, high statistical quality, faster than MT19937 in practice.
- **Mersenne Twister opt-in** — keep MT19937 for legacy reproducibility via `Random::new_mersenne_twister()` (requires the `alloc` feature).
- **`no_std` ready** — pure-core build with no allocator: `Random::from_seed([u8; 32])` gives you a working RNG on any embedded target.
- **Unbiased bounded sampling** — `int`, `uint`, `random_range`, `bounded` use Lemire's nearly-divisionless method, not modulo.
- **Bit-precise floats** — `float()` carries 24 mantissa bits (the f32 maximum); `double()` / `f64()` carry 53 (the f64 maximum). Always `[0.0, 1.0)`.
- **Distributions** — `normal`, `exponential`, `poisson` (`std`-free, via `libm`).
- **`rand 0.10` traits** — implements `TryRng` (and the auto-derived `Rng`) plus `SeedableRng`, so vrd plugs into the wider `rand` ecosystem.

## Feature flags

| Flag | Default? | What it does |
| :--- | :--- | :--- |
| `std` | yes | Entropy seeding via `rand::rng()`; `std::error::Error` impls. |
| `alloc` | via `std` | `Random::bytes`, `Random::string`, `Random::sample`, the heap-stored Mersenne Twister backend. |
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
- The `pseudo()` method is gone (XOR-folding RNG outputs is statistically a no-op; the operation was misleading).
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
cargo check --target thumbv7em-none-eabihf --no-default-features     # embedded smoke check
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, signed commits, and PR guidelines.

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#random-vrd">Back to Top</a></p>

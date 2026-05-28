<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Getting started with vrd

Three steps from a fresh project to a working RNG.

## 1. Add the dependency

```bash
cargo add vrd
```

Or in `Cargo.toml`:

```toml
[dependencies]
vrd = "0.0.12"
```

Requires Rust 1.70.0 or later.

## 2. Generate a number

```rust
use vrd::Random;

fn main() {
    let mut rng = Random::new();
    println!("a random u32: {}", rng.rand());
}
```

`Random::new()` produces an entropy-seeded Xoshiro256++
generator. Requires `std`.

For deterministic, allocation-free sequences (works on `no_std`
embedded targets):

```rust
use vrd::Random;

let mut rng = vrd::Random::from_u64_seed(42);
let _ = rng.rand();
```

## 3. Pick the right backend

vrd ships four backends. Default is fine for most uses.

| Use case | Backend | Construct via |
| :-- | :-- | :-- |
| Default (fast, non-crypto) | Xoshiro256++ | `Random::new()` |
| Legacy reproducibility | MT19937 | `Random::new_mersenne_twister()` (`alloc + std`) |
| Smallest state, fastest | PCG32 | `Random::new_pcg32()` (`pcg` feature, `std`) |
| Security tokens, API keys | ChaCha20 CSPRNG | `Random::new_secure()` (`crypto + std`) |

Full backend table and selection guidance in
[`README.md`](README.md#choosing-a-backend).

## What next?

- [`doc/USER-GUIDE.md`](doc/USER-GUIDE.md) - the full walkthrough.
- [`doc/MIGRATION-FROM-RAND.md`](doc/MIGRATION-FROM-RAND.md) - if
  you're coming from `rand`.
- [`doc/MIGRATION-FROM-FASTRAND.md`](doc/MIGRATION-FROM-FASTRAND.md)
  - if you're coming from `fastrand`.
- [`docs.rs/vrd`](https://docs.rs/vrd) - every public method
  has a worked example.
- [`examples/`](examples/) - 44 runnable examples; `cargo run
  --example all` runs every one with `[ok]`/`[fail]` status.

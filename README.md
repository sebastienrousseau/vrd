<p align="center">
  <img src="https://kura.pro/vrd/images/logos/vrd.svg" alt="Random (VRD) logo" width="128" />
</p>

<h1 align="center">Random (VRD)</h1>

<p align="center">
  <strong>A Rust library for generating random and pseudo-random numbers based on the Mersenne Twister algorithm.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/vrd/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/vrd/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/vrd"><img src="https://img.shields.io/crates/v/vrd.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/vrd"><img src="https://img.shields.io/badge/docs.rs-vrd-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/vrd"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/vrd?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://lib.rs/crates/vrd"><img src="https://img.shields.io/badge/lib.rs-v0.0.9-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
</p>

---

## Install

```bash
cargo add vrd
```

Or add to `Cargo.toml`:

```toml
[dependencies]
vrd = "0.0.9"
```

You need [Rust](https://rustup.rs/) 1.56.0 or later. Works on macOS, Linux, and Windows.

---

## Overview

VRD generates random and pseudo-random numbers using the Mersenne Twister (MT19937) algorithm.

- **MT19937 implementation** for high-quality pseudo-random output
- **Seedable generator** for deterministic sequences
- **Range generation** within specified bounds
- **Cross-platform** — consistent output everywhere

---

## Features

| | |
| :--- | :--- |
| **Mersenne Twister** | MT19937 pseudo-random number generator |
| **Seedable** | Deterministic output with configurable seeds |
| **Distributions** | Uniform, normal, and other distributions |
| **Range generation** | Generate numbers within specified ranges |
| **Cross-platform** | Consistent output across macOS, Linux, and Windows |

---

## Usage

```rust
use vrd::Random;

fn main() {
    let mut rng = Random::new();
    println!("Random u32: {}", rng.rand());
    println!("Random in range [1, 100]: {}", rng.int(1, 100));
    println!("Random float: {}", rng.float());
}
```

---

## Development

```bash
cargo build        # Build the project
cargo test         # Run all tests
cargo clippy       # Lint with Clippy
cargo fmt          # Format with rustfmt
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, signed commits, and PR guidelines.

---

**THE ARCHITECT** \u1d2b [Sebastien Rousseau](https://sebastienrousseau.com)
**THE ENGINE** \u1d5e [EUXIS](https://euxis.co) \u1d2b Enterprise Unified Execution Intelligence System

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#random-vrd">Back to Top</a></p>
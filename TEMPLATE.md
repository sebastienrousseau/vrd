<!-- markdownlint-disable MD033 MD041 -->

<img src="https://cloudcdn.pro/vrd/v1/logos/vrd.svg"
alt="vrd logo" width="66" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# Versatile Random Distributions (VRD)

A lightweight, `no_std`-friendly random number generator backed by Xoshiro256++,
with optional Mersenne Twister support.

[![Made With Love][made-with-rust]][11] [![Crates.io][crates-badge]][06] [![lib.rs][libs-badge]][08] [![Docs.rs][docs-badge]][07] [![Codecov][codecov-badge]][12] [![Build Status][build-badge]][09] [![GitHub][github-badge]][13]

![divider][divider]

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

• [Website][00] • [Documentation][07] • [Report Bug][03]
• [Request Feature][03] • [Contributing Guidelines][04]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview

`vrd` generates high-quality random numbers in Rust. The default backend is
**Xoshiro256++** — 32-byte state, period 2^256 - 1, with SplitMix64 seed
whitening. **Mersenne Twister (MT19937)** is opt-in for legacy
reproducibility.

Bounded integer sampling is unbiased (Lemire's nearly-divisionless method);
floats carry full 24-bit (`f32`) and 53-bit (`f64`) mantissa precision; the
crate compiles for `no_std` targets such as Cortex-M
(`thumbv7em-none-eabihf`) and WebAssembly (`wasm32-unknown-unknown`) with
no allocator — both validated in CI.

[00]: https://vrdlib.com "vrd"
[03]: https://github.com/sebastienrousseau/vrd/issues "Issues"
[04]: https://github.com/sebastienrousseau/vrd/blob/main/CONTRIBUTING.md "Contributing Instructions"
[06]: https://crates.io/crates/vrd "Crates.io"
[07]: https://docs.rs/vrd "Docs.rs"
[08]: https://lib.rs/crates/vrd "Lib.rs"
[09]: https://github.com/sebastienrousseau/vrd/actions?query=branch%3Amain
[11]: https://www.rust-lang.org "The Rust Programming Language"
[12]: https://codecov.io/gh/sebastienrousseau/vrd "Codecov"
[13]: https://github.com/sebastienrousseau/vrd/ "GitHub"

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/vrd/ci.yml?branch=main&style=for-the-badge&logo=github "Build Status"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/vrd?style=for-the-badge&token=oEisyTucB5 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/vrd.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/vrd.svg?style=for-the-badge 'Docs.rs badge'
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/vrd-8da0cb?style=for-the-badge&labelColor=555555&logo=github "GitHub"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.10-orange.svg?style=for-the-badge 'Lib.rs badge'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'

## Changelog 📚

<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/vrd/images/logos/vrd.svg"
alt="Random (VRD) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# Random (VRD)

A Rust library for generating high-quality random numbers based on the Mersenne
Twister algorithm.

*Part of the [Mini Functions][00] family of libraries.*

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Random (VRD) Banner][banner]

[![Made With Rust][made-with-rust-badge]][14] [![Crates.io][crates-badge]][08] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![License][license-badge]][02] [![Codecov][codecov-badge]][15]

• [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

![divider][divider]

## Overview 📖

`Random (VRD)` is a Rust library for generating high-quality random numbers
based on the Mersenne Twister algorithm.

The `Random` struct in this library provides a robust interface for generating
a variety of random numbers using the Mersenne Twister algorithm. Additionally,
the `MersenneTwisterConfig` struct allows for advanced configuration of the
algorithm.

The Random (VRD) is used to generate random numbers using the Mersenne Twister
algorithm. It generates pseudorandom integers uniformly distributed in
0..(2^32 - 1) starting from any odd seed in 0..(2^32 - 1).

The index is incremented after each random number is generated. When the index
reaches 624, the array is reinitialized and the index is reset to 0.

## Features ✨

- Create new random number generator and seed it with a value
- Designed for speed and efficiency
- Generate random 32-bit unsigned integers within a given range
- Provide random numbers of different types, including booleans, bytes, chars,
  doubles, floats, integers, and unsigned integers
- Mutate the state of the random number generator
- Produce pseudo-random number sequences that are different from each other
- Regulate the randomness of the generated numbers, including the seed value
  and the number of bits used
- Select a random element from a slice of values
- Generate random numbers from various probability distributions, including
  uniform, normal, exponential, and Poisson

## Getting Started 🚀

It takes just a few minutes to get up and running with `Random (VRD)`.

### Installation

To install `Random (VRD)`, you need to have the Rust toolchain installed on
your machine. You can install the Rust toolchain by following the instructions
on the [Rust website][14].

Once you have the Rust toolchain installed, you can install `Random (VRD)`
using the following command:

```shell
cargo install vrd
```

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
vrd = "0.0.6"
serde = { version = "1.0.160", features = ["derive"] }
```

### Usage

```rust
use vrd::random::Random;

let mut rng = Random::new();
let rand_int = rng.int(1, 10);
println!("Random integer between 1 and 10: {}", rand_int);

let rand_float = rng.float();
println!("Random float: {}", rand_float);

let rand_bytes = rng.bytes(10);
println!("Random bytes: {:?}", rand_bytes);
```

### Examples

To get started with `Random (VRD)`, you can use the examples provided in the
`examples` directory of the project.

To run the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example vrd
```

## Macros 🦀

The `Random (VRD)` library provides a set of macros that simplify the usage of
the library. These macros offer a convenient way to generate random numbers of
different types and distributions.

Here are some of the available macros:

- `rand_bool!(rng, probability)`: Generate a random boolean with the provided
  probability.
- `rand_bytes!(rng, length)`: Generate a vector of random bytes with the
  specified length.
- `rand_char!(rng)`: Generate a random character within the range 'a'..='z'.
- `rand_choose!(rng, values)`: Generate a random element from a slice of values.
- `rand_float!(rng)`: Generate a random float.
- `rand_int!(rng, min, max)`: Generate a random integer within the given range.
- `rand_uint!(rng, min, max)`: Generate a random 32-bit unsigned integer within
  the given range.
- `rand_double!(rng)`: Generate a random double.
- `rand_string!(rng, length)`: Generate a random string of the specified length.
- `rand_alphanumeric!(rng)`: Generate a random alphanumeric character.
- `rand_shuffle!(rng, slice)`: Shuffle a mutable slice randomly.
- `rand_weighted_choice!(rng, choices, weights)`: Select a random element from
  a slice based on the provided weights.
- `rand_normal!(rng, mu, sigma)`: Generate a normally distributed random number
  with the given mean and standard deviation.
- `rand_exponential!(rng, rate)`: Generate a random number from the exponential
  distribution with the given rate parameter.
- `rand_poisson!(rng, mean)`: Generate a random number from a Poisson
  distribution with the specified mean parameter.

For more details on how to use these macros, please refer to the
[documentation](https://docs.rs/vrd).

### Platform support

`Random (VRD)` is supported and tested on the following platforms:

#### Tier 1 platforms 🏆

| Operating System | Target | Description |
| --- | --- | --- |
| Linux   | aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture |
| Linux   | i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) |
| Linux   | x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) |
| macOS   | x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) |
| Windows | i686-pc-windows-gnu | 32-bit Windows (7 or later) |
| Windows | i686-pc-windows-msvc | 32-bit Windows (7 or later) |
| Windows | x86_64-pc-windows-gnu | 64-bit Windows (7 or later) |
| Windows | x86_64-pc-windows-msvc | 64-bit Windows (7 or later) |

#### Tier 2 platforms 🥈

| Operating System | Target | Description |
| --- | --- | --- |
| 64-bit Linux | x86_64-unknown-linux-musl | 64-bit Linux (kernel 2.6.32+, musl libc) |
| ARM64 Linux | aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture |
| ARM64 macOS | aarch64-apple-darwin | 64-bit macOS on Apple Silicon |
| ARM64 Windows | aarch64-pc-windows-msvc | 64-bit Windows (aarch64-pc-windows-msvc) |
| ARMv6 Linux | arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) |
| ARMv6 Linux, hardfloat | arm-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ARMv7 Linux, hardfloat | armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| FreeBSD  | x86_64-unknown-freebsd | 64-bit FreeBSD on x86-64 |
| MIPS (LE) Linux | mipsel-unknown-linux-gnu | MIPSel Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS Linux | mips-unknown-linux-gnu | MIPS Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS64 (LE) Linux | mips64el-unknown-linux-gnuabi64 | MIPS64el Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS64 Linux | mips64-unknown-linux-gnuabi64 | MIPS64 Linux (kernel 2.6.32+, glibc 2.11+) |
| NetBSD  | x86_64-unknown-netbsd | 64-bit NetBSD on x86-64 |
| PowerPC Linux | powerpc-unknown-linux-gnu | PowerPC Linux (kernel 3.2, glibc 2.17) |
| PPC64 Linux | powerpc64-unknown-linux-gnu | PowerPC64 Linux (kernel 3.2, glibc 2.17) |
| PPC64LE Linux | powerpc64le-unknown-linux-gnu | PowerPC64le Linux (kernel 3.2, glibc 2.17) |
| RISC-V Linux | riscv64gc-unknown-linux-gnu | RISC-V Linux (kernel 3.2, glibc 2.17) |
| S390x Linux | s390x-unknown-linux-gnu | s390x Linux (kernel 3.2, glibc 2.17) |

The [GitHub Actions][11] shows the platforms in which the `Random (VRD)`
library tests are run.

## Documentation 📚

For detailed documentation, please visit:

- [Website](https://vrdlib.com)
- [Docs.rs](https://docs.rs/vrd)
- [Lib.rs](https://lib.rs/crates/vrd)
- [Crates.io](https://crates.io/crates/vrd)

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain backward
compatibility, `Random (VRD)` follows [semantic versioning][07].

## License 📝

The project is licensed under the terms of both the MIT license and the Apache
License (Version 2.0).

- [Apache License, Version 2.0][02]
- [MIT license][03]

## Contribution 🤝

We welcome all people who want to contribute. Please see the
[contributing instructions][05] for more information.

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to the [Rust's Code of Conduct][12].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Acknowledgements 💙

A big thank you to all the awesome contributors of [vrd][06] for their help and
support.

A special thank you goes to the [Rust Reddit][13] community for providing a
lot of useful suggestions on how to improve this project.

*Made with ❤️ by [Sébastien Rousseau](https://sebastienrousseau.com)*

[00]: https://minifunctions.com/ "MiniFunctions"
[01]: https://vrdlib.com "Random (VRD)"
[02]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[03]: https://opensource.org/licenses/MIT "MIT license"
[04]: https://github.com/sebastienrousseau/vrd/issues "Issues"
[05]: https://github.com/sebastienrousseau/vrd/blob/main/CONTRIBUTING.md "Contributing Instructions"
[06]: https://github.com/sebastienrousseau/vrd/graphs/contributors "Contributors"
[07]: http://semver.org/ "Semantic Versioning"
[08]: https://crates.io/crates/vrd "Crates.io"
[09]: https://docs.rs/vrd "Docs.rs"
[10]: https://lib.rs/crates/vrd "Lib.rs"
[11]: https://github.com/sebastienrousseau/vrd/actions "GitHub Actions"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://reddit.com/r/rust "Rust Reddit"
[14]: https://www.rust-lang.org "The Rust Programming Language"
[15]: https://codecov.io/gh/sebastienrousseau/vrd "Codecov"

[banner]: https://kura.pro/vrd/images/titles/title-vrd.svg 'Random (VRD) Banner'
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/vrd?style=for-the-badge&token=oEisyTucB5 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/vrd.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/vrd.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.4-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/vrd.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'

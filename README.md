<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/vrd/images/logos/vrd.svg"
alt="Random (VRD) logo" width="66" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# Random (VRD)

A Rust library for generating high-quality random numbers based on the Mersenne
Twister algorithm.

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

`Random (VRD)` is a Rust library for generating high-quality random numbers based on the Mersenne Twister algorithm.

The `Random` struct in this library provides a robust interface for generating a variety of random numbers using the Mersenne Twister algorithm. Additionally, the `MersenneTwisterConfig` struct allows for advanced configuration of the algorithm.

`Random (VRD)` generates pseudorandom integers uniformly distributed in 0..(2^32 - 1), starting from any odd seed in 0..(2^32 - 1). The index is incremented after each random number is generated. When the index reaches 624, the array is reinitialized and the index is reset to 0.

## Features

- Create a new random number generator and seed it with a value.
- Designed for speed and efficiency.
- Generate random 32-bit unsigned integers within a given range.
- Provide random numbers of different types, including booleans, bytes, chars, doubles, floats, integers, and unsigned integers.
- Mutate the state of the random number generator.
- Produce pseudorandom number sequences that are different from each other.
- Regulate the randomness of the generated numbers, including the seed value and the number of bits used.
- Select a random element from a slice of values.
- Generate random numbers from various probability distributions, including uniform, normal, exponential, and Poisson.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vrd = "0.0.8"
serde = { version = "1.0.209", features = ["derive"] }
```

## Usage

Here's a quick example on how to use Random (VRD) to generate random numbers:

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

## Documentation

For full API documentation, please visit <https://doc.libyml.com/vrd/> or <https://docs.rs/vrd>.

## Rust Version Compatibility

Compiler support: requires rustc 1.56.0+

## Examples

`Random (VRD)` provides a set of comprehensive examples. You can find them in the `examples` directory of the project. To run the examples, clone the repository and execute the following command in your terminal from the project:

```shell
cargo run --example example
```

## Macros

The `Random (VRD)` library provides a set of macros that simplify the usage of the library. These macros offer a convenient way to generate random numbers of different types and distributions.

Here are some of the available macros:

- `rand_bool!(rng, probability)`: Generate a random boolean with the provided probability.
- `rand_bytes!(rng, length)`: Generate a vector of random bytes with the specified length.
- `rand_char!(rng)`: Generate a random character within the range 'a'..='z'.
- `rand_choose!(rng, values)`: Generate a random element from a slice of values.
- `rand_float!(rng)`: Generate a random float.
- `rand_int!(rng, min, max)`: Generate a random integer within the given range.
- `rand_uint!(rng, min, max)`: Generate a random 32-bit unsigned integer within the given range.
- `rand_double!(rng)`: Generate a random double.
- `rand_string!(rng, length)`: Generate a random string of the specified length.
- `rand_alphanumeric!(rng)`: Generate a random alphanumeric character.
- `rand_shuffle!(rng, slice)`: Shuffle a mutable slice randomly.
- `rand_weighted_choice!(rng, choices, weights)`: Select a random element from a slice based on the provided weights.
- `rand_normal!(rng, mu, sigma)`: Generate a normally distributed random number with the given mean and standard deviation.
- `rand_exponential!(rng, rate)`: Generate a random number from the exponential distribution with the given rate parameter.
- `rand_poisson!(rng, mean)`: Generate a random number from a Poisson distribution with the specified mean parameter.

For more details on how to use these macros, please refer to the [documentation](https://docs.rs/vrd).

## Contributing

Contributions are welcome! Please submit a Pull Request on [GitHub][04].

## Credits and Acknowledgements

A big thank you to all the awesome contributors of [vrd][05] for their help and support.

A special thank you goes to the [Rust Reddit][10] community for providing a lot of useful suggestions on how to improve this project.

## License

Licensed under either of the [Apache License][01] or the [MIT license][02] at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[00]: https://vrdlib.com "Random (VRD)"
[01]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[02]: https://opensource.org/licenses/MIT "MIT license"
[03]: https://github.com/sebastienrousseau/vrd/issues "Issues"
[04]: https://github.com/sebastienrousseau/vrd/blob/main/CONTRIBUTING.md "Contributing Instructions"
[05]: https://github.com/sebastienrousseau/vrd/graphs/contributors "Contributors"
[06]: https://crates.io/crates/vrd "Crates.io"
[07]: https://docs.rs/vrd "Docs.rs"
[08]: https://lib.rs/crates/vrd "Lib.rs"
[09]: https://github.com/sebastienrousseau/vrd/actions?query=branch%3Amain
[10]: https://reddit.com/r/rust "Rust Reddit"
[11]: https://www.rust-lang.org "The Rust Programming Language"
[12]: https://codecov.io/gh/sebastienrousseau/vrd "Codecov"
[13]: https://github.com/sebastienrousseau/vrd/ "GitHub"

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/vrd/release.yml?branch=main&style=for-the-badge&logo=github "Build Status"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/vrd?style=for-the-badge&token=oEisyTucB5 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/vrd.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/vrd.svg?style=for-the-badge 'Docs.rs badge'
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/vrd-8da0cb?style=for-the-badge&labelColor=555555&logo=github "GitHub"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.8-orange.svg?style=for-the-badge 'Lib.rs badge'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'

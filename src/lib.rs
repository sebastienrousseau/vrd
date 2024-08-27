// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//!
//!# Random (VRD)
//!
//!A Rust library for generating high-quality random numbers based on the Mersenne
//!Twister algorithm.
//!
//![![Made With Love][made-with-rust]][11] [![Crates.io][crates-badge]][06] [![lib.rs][libs-badge]][08] [![Docs.rs][docs-badge]][07] [![Codecov][codecov-badge]][12] [![Build Status][build-badge]][09] [![GitHub][github-badge]][13]
//!
//!![divider][divider]
//!
//!<!-- markdownlint-disable MD033 MD041 -->
//!<center>
//!<!-- markdownlint-enable MD033 MD041 -->
//!
//!• [Website][00] • [Documentation][07] • [Report Bug][03]
//!• [Request Feature][03] • [Contributing Guidelines][04]
//!
//!<!-- markdownlint-disable MD033 MD041 -->
//!</center>
//!<!-- markdownlint-enable MD033 MD041 -->
//!
//!## Overview
//!
//!`Random (VRD)` is a Rust library for generating high-quality random numbers based on the Mersenne Twister algorithm.
//!
//!The `Random` struct in this library provides a robust interface for generating a variety of random numbers using the Mersenne Twister algorithm. Additionally, the `MersenneTwisterConfig` struct allows for advanced configuration of the algorithm.
//!
//!`Random (VRD)` generates pseudorandom integers uniformly distributed in 0..(2^32 - 1), starting from any odd seed in 0..(2^32 - 1). The index is incremented after each random number is generated. When the index reaches 624, the array is reinitialized and the index is reset to 0.
//!
//!## Features
//!
//!- Create a new random number generator and seed it with a value.
//!- Designed for speed and efficiency.
//!- Generate random 32-bit unsigned integers within a given range.
//!- Provide random numbers of different types, including booleans, bytes, chars, doubles, floats, integers, and unsigned integers.
//!- Mutate the state of the random number generator.
//!- Produce pseudorandom number sequences that are different from each other.
//!- Regulate the randomness of the generated numbers, including the seed value and the number of bits used.
//!- Select a random element from a slice of values.
//!- Generate random numbers from various probability distributions, including uniform, normal, exponential, and Poisson.
//!
//!## Installation
//!
//!Add this to your `Cargo.toml`:
//!
//!```toml
//![dependencies]
//!vrd = "0.0.8"
//!serde = { version = "1.0.209", features = ["derive"] }
//!```
//!
//!## Usage
//!
//!Here's a quick example on how to use Random (VRD) to generate random numbers:
//!
//!```rust
//!use vrd::random::Random;
//!
//!let mut rng = Random::new();
//!let rand_int = rng.int(1, 10);
//!println!("Random integer between 1 and 10: {}", rand_int);
//!
//!let rand_float = rng.float();
//!println!("Random float: {}", rand_float);
//!
//!let rand_bytes = rng.bytes(10);
//!println!("Random bytes: {:?}", rand_bytes);
//!```
//!
//!## Documentation
//!
//!For full API documentation, please visit <https://doc.libyml.com/vrd/> or <https://docs.rs/vrd>.
//!
//!## Rust Version Compatibility
//!
//!Compiler support: requires rustc 1.56.0+
//!
//!## Examples
//!
//!`Random (VRD)` provides a set of comprehensive examples. You can find them in the `examples` directory of the project. To run the examples, clone the repository and execute the following command in your terminal from the project:
//!
//!```shell
//!cargo run --example
//!```
//!
//!## Macros
//!
//!The `Random (VRD)` library provides a set of macros that simplify the usage of the library. These macros offer a convenient way to generate random numbers of different types and distributions.
//!
//!Here are some of the available macros:
//!
//!- `rand_bool!(rng, probability)`: Generate a random boolean with the provided probability.
//!- `rand_bytes!(rng, length)`: Generate a vector of random bytes with the specified length.
//!- `rand_char!(rng)`: Generate a random character within the range 'a'..='z'.
//!- `rand_choose!(rng, values)`: Generate a random element from a slice of values.
//!- `rand_float!(rng)`: Generate a random float.
//!- `rand_int!(rng, min, max)`: Generate a random integer within the given range.
//!- `rand_uint!(rng, min, max)`: Generate a random 32-bit unsigned integer within the given range.
//!- `rand_double!(rng)`: Generate a random double.
//!- `rand_string!(rng, length)`: Generate a random string of the specified length.
//!- `rand_alphanumeric!(rng)`: Generate a random alphanumeric character.
//!- `rand_shuffle!(rng, slice)`: Shuffle a mutable slice randomly.
//!- `rand_weighted_choice!(rng, choices, weights)`: Select a random element from a slice based on the provided weights.
//!- `rand_normal!(rng, mu, sigma)`: Generate a normally distributed random number with the given mean and standard deviation.
//!- `rand_exponential!(rng, rate)`: Generate a random number from the exponential distribution with the given rate parameter.
//!- `rand_poisson!(rng, mean)`: Generate a random number from a Poisson distribution with the specified mean parameter.
//!
//!For more details on how to use these macros, please refer to the [documentation](https://docs.rs/vrd).
//!
//!## Contributing
//!
//!Contributions are welcome! Please submit a Pull Request on [GitHub][04].
//!
//!## Credits and Acknowledgements
//!
//!A big thank you to all the awesome contributors of [vrd][05] for their help and support.
//!
//!A special thank you goes to the [Rust Reddit][10] community for providing a lot of useful suggestions on how to improve this project.
//!
//!## License
//!
//!Licensed under either of the [Apache License][01] or the [MIT license][02] at your option.
//!
//!Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
//![00]: https://vrdlib.com "Random (VRD)"
//![01]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
//![02]: https://opensource.org/licenses/MIT "MIT license"
//![03]: https://github.com/sebastienrousseau/vrd/issues "Issues"
//![04]: https://github.com/sebastienrousseau/vrd/blob/main/CONTRIBUTING.md "Contributing Instructions"
//![05]: https://github.com/sebastienrousseau/vrd/graphs/contributors "Contributors"
//![06]: https://crates.io/crates/vrd "Crates.io"
//![07]: https://docs.rs/vrd "Docs.rs"
//![08]: https://lib.rs/crates/vrd "Lib.rs"
//![09]: https://github.com/sebastienrousseau/vrd/actions?query=branch%3Amain
//![10]: https://reddit.com/r/rust "Rust Reddit"
//![11]: https://www.rust-lang.org "The Rust Programming Language"
//![12]: https://codecov.io/gh/sebastienrousseau/vrd "Codecov"
//![13]: https://github.com/sebastienrousseau/vrd/ "GitHub"
//!
//![build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/vrd/release.yml?branch=main&style=for-the-badge&logo=github "Build Status"
//![codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/vrd?style=for-the-badge&token=oEisyTucB5 'Codecov'
//![crates-badge]: https://img.shields.io/crates/v/vrd.svg?style=for-the-badge 'Crates.io badge'
//![divider]: https://kura.pro/common/images/elements/divider.svg "divider"
//![docs-badge]: https://img.shields.io/docsrs/vrd.svg?style=for-the-badge 'Docs.rs badge'
//![github-badge]: https://img.shields.io/badge/github-sebastienrousseau/vrd-8da0cb?style=for-the-badge&labelColor=555555&logo=github "GitHub"
//![libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.8-orange.svg?style=for-the-badge 'Lib.rs badge'
//![made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
//!
//!

#![doc(
    html_favicon_url = "https://kura.pro/vrd/images/favicon.ico",
    html_logo_url = "https://kura.pro/vrd/images/logos/vrd.svg",
    html_root_url = "https://docs.rs/vrd"
)]
#![crate_name = "vrd"]
#![crate_type = "lib"]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]
#![doc = "Minimum supported Rust version: 1.56.0"]

use rlg::{log::Log, log_format::LogFormat, log_level::LogLevel};
use std::error::Error;
use std::fmt;

/// The `mersenne_twister` module contains the implementation of the Mersenne Twister algorithm.
pub mod mersenne_twister;

// Re-export MersenneTwisterConfig so it's accessible from outside the crate.
pub use mersenne_twister::MersenneTwisterConfig;

/// The `macros` module contains functions for generating macros.
pub mod macros;

/// The `random` module contains the implementation of the `Random` struct.
pub mod random;

/// Custom error type for the `Random (VRD)` library.
#[derive(Debug)]
pub enum VrdError {
    /// An error occurred while logging.
    LogError(String),
    /// A general error occurred in the library.
    GeneralError(String),
}

impl fmt::Display for VrdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VrdError::LogError(ref err) => {
                write!(f, "Log error: {}", err)
            }
            VrdError::GeneralError(ref err) => {
                write!(f, "General error: {}", err)
            }
        }
    }
}

impl Error for VrdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            VrdError::LogError(_) => None,
            VrdError::GeneralError(_) => None,
        }
    }
}

/// The main entry point for the `Random (VRD)` library.
///
/// This function performs initial setup and checks before the library can be used. It also provides a basic interface for interacting with the library.
///
/// # Returns
/// - `Ok(())` if the library initializes successfully.
/// - `Err(Box<dyn std::error::Error>)` if there is an error during initialization.
///
/// # Examples
/// ```rust
/// use vrd::random::Random;
/// if let Err(e) = vrd::run() {
///     println!("Error initializing Random (VRD) library: {}", e);
/// }
/// ```
///
/// # Errors
/// - Returns a simulated error if the environment variable `VRD_TEST_MODE` is set to "1". This is typically used for testing purposes.
///
/// # Notes
/// - The function prints a welcome message and a brief description of the library.
/// - It checks for the `VRD_TEST_MODE` environment variable to simulate an error, which can be useful for testing error handling in applications using this library.
pub fn run() -> Result<(), Box<dyn Error>> {
    if std::env::var("VRD_TEST_MODE").unwrap_or_default() == "1" {
        return Err(Box::new(VrdError::GeneralError(
            "Simulated error".into(),
        )));
    }
    let name = "vrd";
    println!("Welcome to `{}` 👋!", { name }.to_uppercase());
    println!(
        "A Rust library for generating random and pseudo-random numbers based on the Mersenne Twister algorithm"
    );
    Ok(())
}

/// Create a new log entry with the provided parameters.
///
/// This function constructs a new `Log` instance using the provided parameters.
///
/// # Parameters
///
/// * `uuid` - A string representing the unique identifier for the log entry.
/// * `iso` - A string representing the ISO timestamp for the log entry.
/// * `level` - An enum representing the log level (e.g., `LogLevel::Info`, `LogLevel::Warning`, `LogLevel::Error`).
/// * `message` - A string containing the log message.
///
/// # Returns
///
/// A new `Log` instance with the provided parameters.
pub fn create_log_entry(
    uuid: &str,
    iso: &str,
    level: LogLevel,
    message: &str,
) -> Log {
    Log::new(uuid, iso, &level, "VRD", message, &LogFormat::JSON)
}

/// Asynchronously logs a `Log` entry.
///
/// This function takes a `Log` entry as input and logs it asynchronously.
/// If the logging operation fails, it returns a `VrdError::LogError`.
///
/// # Parameters
///
/// * `entry`: A `Log` instance representing the entry to be logged.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>`:
///   - `Ok(())`: If the logging operation is successful.
///   - `Err(Box<dyn Error>)`: If the logging operation fails, it returns a `VrdError::LogError` containing the error message.
///
/// # Errors
///
/// - Returns a `VrdError::LogError` if logging fails.
pub async fn log_entry_async(entry: Log) -> Result<(), Box<dyn Error>> {
    entry.log().await.map_err(|e| {
        Box::new(VrdError::LogError(format!(
            "Failed to log entry asynchronously: {}",
            e
        )))
    })?;
    Ok(())
}

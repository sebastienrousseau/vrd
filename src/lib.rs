// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//!
//! # `Random (VRD)` 🦀
//!
//! [![vrd](https://kura.pro/vrd/images/banners/banner-vrd.svg)](https://vrdlib.com "Random (VRD) - A Rust library for generating random and pseudo-random numbers based on the Mersenne Twister algorithm")
//!
//! A Rust library for generating high-quality random numbers based on
//! the Mersenne Twister algorithm.
//!
//! *Part of the [Mini Functions][0] family of libraries.*
//!
//! [![Crates.io](https://img.shields.io/crates/v/vrd.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/vrd "Crates.io")
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.4-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/vrd "Lib.rs")
//! [![License](https://img.shields.io/crates/l/vrd.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](https://opensource.org/license/MIT/ "MIT or Apache License, Version 2.0")
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org "Rust")
//!
//! ## Overview
//!
//! The Mersenne Twister is a pseudorandom number generator (PRNG) that
//! is often used in computer simulations and games. It is a fast and
//! reliable PRNG.
//!
//! `Random (VRD)` can be used to generate random numbers based on the
//! Mersenne Twister algorithm. The library generates pseudorandom
//! integers uniformly distributed in 0..(2^32 - 1) starting from any
//! odd seed in 0..(2^32 - 1). The index is incremented after each
//! random number is generated. When the index reaches 624, the array is
//! reinitialized and the index is reset to 0.
//!
//! ## Features ✨
//!
//! - Create new random number generator and seed it with a value.
//! - Design for speed and efficiency in mind.
//! - Generate random 32-bit unsigned integer within a given range.
//! - Provide random numbers of different types, including booleans, bytes, chars, doubles, floats, integers, and unsigned integers.
//! - Mutate the state of the random number generator.
//! - Produce pseudo-random number sequences that are different from each other.
//! - Regulate the randomness of the generated numbers, including the seed value and the number of bits used.
//! - Select a random element from a slice of values.
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! vrd = "0.0.8"
//! serde = { version = "1.0.160", features = ["derive"] }
//! ```
//!
//! ## Examples
//!
//! Check out the examples folder for helpful snippets of code that
//! demonstrate how to use the `Random (VRD)` library. You can also
//! check out the [documentation](https://docs.rs/vrd) for more
//! information on how to use the library.
//!
//! ```
//! use vrd::random::Random;
//!
//! let mut rng = Random::new();
//! let rand_int = rng.int(1, 10);
//! println!("Random integer between 1 and 10: {}", rand_int);
//!
//! let rand_float = rng.float();
//! println!("Random float: {}", rand_float);
//!
//! let rand_bytes = rng.bytes(10);
//! println!("Random bytes: {:?}", rand_bytes);
//!
//! ```
//!
//! # Errors
//!
//! The `Random` struct methods do not currently return errors, but in
//! the future they may return `Result` types to handle situations where
//! the Mersenne Twister algorithm fails to generate a random number or
//! a range of numbers.
//!
//! ## License
//!
//! The project is licensed under the terms of both the MIT license and
//! the Apache License (Version 2.0).
//!
//! - [Apache License, Version 2.0](LICENSE-APACHE.md)
//! - [MIT license](LICENSE-MIT.md)
//!
//! [0]: https://minifunctions.com/vrd
//!
#![cfg_attr(feature = "bench", feature(test))]
#![doc(
    html_favicon_url = "https://kura.pro/vrd/images/favicon.ico",
    html_logo_url = "https://kura.pro/vrd/images/logos/vrd.svg",
    html_root_url = "https://docs.rs/vrd"
)]
#![crate_name = "vrd"]
#![crate_type = "lib"]
use rlg::{log::Log, log_format::LogFormat, log_level::LogLevel};

/// The `mersenne_twister` module contains the implementation of the Mersenne Twister algorithm.
pub mod mersenne_twister;

// Re-export MersenneTwisterConfig so it's accessible from outside the crate
pub use mersenne_twister::MersenneTwisterConfig;

/// The `macros` module contains functions for generating macros.
pub mod macros;

/// The `random` module contains the implementation of the `Random` struct.
pub mod random;

/// The main entry point for the `Random (VRD)` library.
///
/// This function performs initial setup and checks before the library can be used. It also provides a basic interface for interacting with the library.
///
/// # Returns
/// - `Ok(())` if the library initializes successfully.
/// - `Err(Box<dyn std::error::Error>)` if there is an error during initialization.
///
/// # Examples
/// ```
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
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("VRD_TEST_MODE").unwrap_or_default() == "1" {
        return Err("Simulated error".into());
    }
    let name = "vrd";
    println!("Welcome to `{}` 👋!", { name }.to_uppercase());
    println!(
        "A Rust library for generating random and pseudo-random numbers based on the Mersenne Twister algorithm"
    );
    Ok(())
}

/// Create a new log entry.
pub fn create_log_entry(
    uuid: &str,
    iso: &str,
    level: LogLevel,
    message: &str,
) -> Log {
    Log::new(uuid, iso, &level, "VRD", message, &LogFormat::JSON)
}

/// Log an entry asynchronously.
pub async fn log_entry_async(
    entry: Log,
) -> Result<(), Box<dyn std::error::Error>> {
    entry.log().await?;
    Ok(())
}

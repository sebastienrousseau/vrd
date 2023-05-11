// Copyright © 2023 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//!
//! # `Random (VRD)` 🦀
//!
//! [![vrd](https://kura.pro/vrd/images/banners/banner-vrd.svg)](https://vrdlib.one "Random (VRD) - A Rust library for generating random and pseudo-random numbers based on the Mersenne Twister algorithm")
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
//! vrd = "0.0.4"
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
//! use vrd::Random;
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
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://kura.pro/vrd/images/favicon.ico",
    html_logo_url = "https://kura.pro/vrd/images/logos/vrd.svg",
    html_root_url = "https://docs.rs/vrd"
)]
#![crate_name = "vrd"]
#![crate_type = "lib"]

use rand::thread_rng;
use rand::Rng;

/// The `macros` module contains functions for generating macros.
pub mod macros;

/// N is the number of elements in the array used for the Mersenne
/// Twister algorithm. Its value is set to 624 for optimal performance.
const N: usize = 624;

/// M is the number of elements to skip in the array used for the
/// Mersenne Twister algorithm. Its value is set to 397 for optimal
/// performance.
const M: usize = 397;

/// MATRIX_A is a constant value used in the Mersenne Twister algorithm.
const MATRIX_A: u32 = 0x9908b0df;

/// UPPER_MASK is a constant value used in the Mersenne Twister
/// algorithm.
const UPPER_MASK: u32 = 0x80000000;

/// LOWER_MASK is a constant value used in the Mersenne Twister
/// algorithm.
const LOWER_MASK: u32 = 0x7fffffff;

/// TEMPERING_MASK_B is a constant value used in the Mersenne Twister
/// algorithm.
const TEMPERING_MASK_B: u32 = 0x9d2c5680;

/// TEMPERING_MASK_C is a constant value used in the Mersenne Twister
/// algorithm.
const TEMPERING_MASK_C: u32 = 0xefc60000;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// The `Random` struct is used to generate random numbers using the
/// Mersenne Twister algorithm.
///
/// It contains an array of unsigned 32-bit integers and an index used
/// to generate random numbers. The array contains 624 elements and the
/// index is used to generate random numbers from the array.
///
/// The index is incremented after each random number is generated.
/// When the index reaches 624, the array is reinitialized and the index
/// is reset to 0.
pub struct Random {
    /// The array of unsigned 32-bit integers used to generate random
    /// numbers
    pub mt: [u32; N],
    /// The current index of the array used in the generation of random
    /// numbers
    pub mti: usize,
}

impl Random {
    /// Returns a random bool with a probability that can be set.
    pub fn bool(&mut self, probability: f64) -> bool {
        thread_rng().gen_bool(probability)
    }

    /// Returns a vector of random bytes of the given length
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            let byte = self.rand() as u8;
            res.push(byte);
        }
        res
    }

    /// Returns a random char within the range 'a'..='z'
    pub fn char(&mut self) -> char {
        thread_rng().gen_range('a'..='z')
    }

    /// Returns a random element from a slice of values
    pub fn choose<'a, T>(&'a mut self, values: &'a [T]) -> Option<&T> {
        if values.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..values.len());
        Some(&values[index])
    }

    /// Returns a random float.
    pub fn float(&mut self) -> f32 {
        thread_rng().gen::<f64>() as f32
    }

    /// Returns a random integer within the given range
    pub fn int(&mut self, min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..=max)
    }

    /// Returns a random unsigned integer within the given range
    pub fn uint(&mut self, min: u32, max: u32) -> u32 {
        thread_rng().gen_range(min..=max)
    }

    /// Returns a random double.
    pub fn double(&mut self) -> f64 {
        thread_rng().gen::<f64>()
    }

    /// Return the value of the `mti` field
    pub fn mti(&self) -> usize {
        self.mti
    }

    /// Set the value of the `mti` field
    pub fn set_mti(&mut self, value: usize) {
        self.mti = value;
    }

    /// Returns new random number generator
    pub fn new() -> Self {
        let mut rng = Random {
            mt: [0; N],
            mti: N + 1,
        };
        let seed = thread_rng().gen();
        rng.mt[0] = seed;
        for i in 1..N {
            rng.mt[i] = 1812433253u32
                .wrapping_mul(rng.mt[i - 1] ^ (rng.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        rng.mti = N;
        rng
    }

    /// Returns pseudo random number
    pub fn pseudo(&mut self) -> u32 {
        let mut res = self.rand();
        for _ in 0..31 {
            res ^= self.rand();
        }
        res
    }

    /// Returns a random 32-bit unsigned integer.
    pub fn rand(&mut self) -> u32 {
        if self.mti >= N {
            if self.mti == N + 1 {
                self.seed(5489);
            }
            self.twist();
        }

        let mut y = self.mt[self.mti];
        self.mti += 1;
        y ^= y >> 11;
        y ^= (y << 7) & TEMPERING_MASK_B;
        y ^= (y << 15) & TEMPERING_MASK_C;
        y ^= y >> 18;
        y
    }

    /// Returns a random 32-bit unsigned integer within a given range.
    pub fn random_range(&mut self, min: u32, max: u32) -> u32 {
        min + self.rand() % (max - min)
    }

    /// Returns a random number within a given range.
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..=max)
    }

    /// Seeds the random number generator with a given value.
    pub fn seed(&mut self, seed: u32) {
        self.mt[0] = seed;
        for i in 1..N {
            self.mt[i] = 1812433253u32
                .checked_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .map_or(u32::MAX, |val| val + i as u32);
        }
        self.mti = N;
    }

    /// Twists the state of the random number generator.
    pub fn twist(&mut self) {
        for i in 0..N {
            let x = (self.mt[i] & UPPER_MASK)
                + (self.mt[(i + 1) % N] & LOWER_MASK);
            let x_a = x >> 1;
            if x % 2 != 0 {
                self.mt[i] = self.mt[(i + M) % N] ^ x_a ^ MATRIX_A;
            } else {
                self.mt[i] = self.mt[(i + M) % N] ^ x_a;
            }
        }
        self.mti = 0;
    }
}

impl std::fmt::Display for Random {
    /// Returns a formatted string representation of the `Random` struct.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Random {{ mt: {:?}, mti: {:?} }}", self.mt, self.mti)
    }
}

impl Default for Random {
    /// Returns a default random number generator
    fn default() -> Self {
        Self::new()
    }
}

/// This is the main entry point for the `Random (VRD)` library.
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

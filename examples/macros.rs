// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! Example code using macros from the `vrd` crate.
//!
//! This example demonstrates the usage of the macros provided by the `vrd` crate for generating random numbers and manipulating slices.
//!
//! ## Examples
//!
//! ### Basic Macros Usage
//!
//! - Using the `rand_bool!` macro: Generate a random boolean value with a specified probability.
//!
//! - Using the `rand_bytes!` macro: Generate a vector of random bytes of a specified length.
//!
//! - Using the `rand_char!` macro: Generate a random character from a range.
//!
//! - Using the `rand_choose!` macro: Select a random element from a slice.
//!
//! - Using the `rand_int!` macro: Generate a random integer within a specified range.
//!
//! - Using the `rand_alphanumeric!` macro: Generate a random alphanumeric character.
//!
//! - Using the `rand_shuffle!` macro: Shuffle a slice randomly.
//!
//! - Using the `rand_weighted_choice!` macro: Choose an element based on weights.
//!
//! - Using the `rand_normal!` macro: Generate a random number from a normal distribution.
//!
//! - Using the `rand_exponential!` macro: Generate a random number from an exponential distribution.
//!
//! - Using the `rand_poisson!` macro: Generate a random number from a Poisson distribution.

use vrd::{
    rand_alphanumeric, rand_bool, rand_bytes, rand_char, rand_choose,
    rand_double, rand_exponential, rand_float, rand_int, rand_normal,
    rand_poisson, rand_shuffle, rand_uint, rand_weighted_choice,
};

fn main() {
    // Example: Generate a random boolean value with a 50% probability
    let mut rng = vrd::random::Random::new();
    let bool_value = rand_bool!(rng, 0.5);
    println!("🦀 Random boolean (50% true): {}", bool_value);

    // Example: Generate a vector of 10 random bytes
    let bytes = rand_bytes!(rng, 10);
    println!("🦀 Random bytes (10 bytes): {:?}", bytes);

    // Example: Generate a random character in the range 'a' to 'z'
    let random_char = rand_char!(rng);
    println!("🦀 Random character ('a'..='z'): {}", random_char);

    // Example: Select a random element from a slice
    let values = [10, 20, 30, 40, 50];
    let random_value =
        rand_choose!(rng, &values).copied().unwrap_or(-1); // Handle None by returning -1
    println!(
        "🦀 Random element from slice [10, 20, 30, 40, 50]: {}",
        random_value
    );

    // Example: Generate a random integer between 1 and 100
    let random_int = rand_int!(rng, 1, 100);
    println!("🦀 Random integer (1 to 100): {}", random_int);

    // Example: Generate a random 32-bit unsigned integer within the range 1 to u32::MAX
    let random_uint = rand_uint!(rng, 1, u32::MAX); // Ensure min < max
    println!("🦀 Random u32 (1 to u32::MAX): {}", random_uint);

    // Example: Generate a random double-precision floating-point number
    let random_double = rand_double!(rng);
    println!("🦀 Random double: {}", random_double);

    // Example: Generate a random floating-point number between 0.0 and 1.0
    let random_float = rand_float!(rng);
    println!("🦀 Random float (0.0 to 1.0): {}", random_float);

    // Example: Generate a random alphanumeric character
    let random_alphanumeric = rand_alphanumeric!(rng);
    println!(
        "🦀 Random alphanumeric character: {}",
        random_alphanumeric
    );

    // Example: Shuffle a mutable slice randomly
    let mut shuffle_values = [1, 2, 3, 4, 5];
    rand_shuffle!(rng, &mut shuffle_values);
    println!("🦀 Shuffled slice: {:?}", shuffle_values);

    // Example: Choose a random element from a slice based on weights
    let choices = ["apple", "banana", "cherry", "date"];
    let weights = [2, 3, 5, 7];
    let weighted_choice =
        rand_weighted_choice!(rng, &choices, &weights);
    println!(
    "🦀 Random element based on weights from ['apple', 'banana', 'cherry', 'date']: {}",
    weighted_choice
);

    // Example: Generate a random number from a normal distribution (mean 0, standard deviation 1)
    let random_normal = rand_normal!(rng, 0.0, 1.0);
    println!("🦀 Random number from normal distribution (mean 0, std dev 1): {}", random_normal);

    // Example: Generate a random number from an exponential distribution with rate parameter 1.0
    let random_exponential = rand_exponential!(rng, 1.0);
    println!(
        "🦀 Random number from exponential distribution (rate 1.0): {}",
        random_exponential
    );

    // Example: Generate a random number from a Poisson distribution with mean 4.0
    let random_poisson = rand_poisson!(rng, 4.0);
    println!(
        "🦀 Random number from Poisson distribution (mean 4.0): {}",
        random_poisson
    );
}

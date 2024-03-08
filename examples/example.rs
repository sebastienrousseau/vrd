// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! Example code using the `vrd` crate.
//!
//! This example demonstrates the usage of the `vrd` crate for generating random numbers.
//! It covers a variety of functionalities provided by the `vrd` crate, including generating
//! random booleans, integers, floating-point numbers, and more complex operations like seeding
//! and state manipulation of the random number generator (RNG).

extern crate vrd;
use self::vrd::random::Random;
use vrd::*;

fn main() {
    // Generating a random boolean with a 50% chance of being true.
    // This demonstrates the usage of Random::bool method.
    let bool: bool = Random::bool(&mut Random::new(), 0.5);
    println!("🦀 Random::bool():        ✅ {bool}");

    // Initializing a new random number generator (RNG) instance.
    let mut rng = Random::new();
    println!("🦀 Random::new():         ✅ {rng}");

    // Accessing the default RNG provided by the `vrd` crate.
    let default = Random::default();
    println!("🦀 Random::default():     ✅ {default}");

    // Generating a random integer between 0 and the maximum value a u32 can hold.
    let random = rng.rand();
    println!("🦀 Random::random():      ✅ {random}");

    // Seeding the RNG with a specific value to ensure reproducible results.
    let seed_value = 12345;
    let mut rng = Random::new();
    rng.seed(seed_value);
    println!("🦀 Random::seed():        ✅ {}", seed_value);

    // Creating a vector of 1000 random bytes.
    let bytes = Random::bytes(&mut rng, 1000);
    println!("🦀 Random::bytes():       ✅ {bytes:?}");

    // Generating a random floating-point number between 0 and 1.
    let float = rng.rand() as f32 / 0x7FFF as f32;
    println!("🦀 Random::float():       ✅ {float}");

    // Producing a random usize value.
    let int = rng.rand() as usize;
    println!("🦀 Random::int():         ✅ {int}");

    // Generating a random integer within the specified range (0 to 100).
    let mut rng = Random::new();
    let min = 0;
    let max = 100;
    let rand_int = rand_int!(rng, min, max);
    println!("🦀 Random integer between {} and {}: {}", min, max, rand_int);

    // Generating a random floating-point number within a specified range.
    let rand_range = rand_float!(rng) * (max as f32 - min as f32) + min as f32;
    println!("🦀 Random number between 0 and 1: {}", rand_range);

    // Creating a random 32-bit unsigned integer within a specified range.
    let rand_uint = random_range!(rng, 0, u32::max_value());
    println!("🦀 Random u32 between 0 and u32::max_value(): {}", rand_uint);

    // Generating a random boolean with a 50% probability.
    let rand_bool = rand_bool!(rng, 0.5);
    println!("🦀 Random boolean with 50% probability: {}", rand_bool);

    // Creating a vector of 10 random bytes.
    let rand_bytes = rand_bytes!(rng, 10);
    println!("🦀 Random bytes: {:?}", rand_bytes);

    // Generating a random character within the range 'a' to 'z'.
    let rand_char = rand_char!(rng);
    println!("🦀 Random char between 'a' and 'z': {}", rand_char);

    // Picking a random element from a predefined slice of integers.
    let values = &[1, 2, 3, 4, 5];
    let rand_choose = rand_choose!(rng, values);
    println!("🦀 Random element from [1, 2, 3, 4, 5]: {:?}", rand_choose);

    // Generating a random floating-point number.
    let rand_float = rand_float!(rng);
    println!("🦀 Random float: {}", rand_float);

    // Creating a random 32-bit unsigned integer using a pseudo-random number generator (PRNG).
    let rand_pseudo = rand_pseudo!(rng);
    println!("🦀 Random u32 using the PRNG: {}", rand_pseudo);

    // Seeding the PRNG with a specific value to get deterministic outputs.
    rand_seed!(rng, 42);
    let rand_seed = rand_pseudo!(rng);
    println!("🦀 Random u32 using the seeded PRNG: {}", rand_seed);

    // Altering the state of the PRNG to vary its output.
    rand_twist!(rng);
    let rand_twist = rand_pseudo!(rng);
    println!("🦀 Random u32 after twisting the PRNG state: {}", rand_twist);

    // Generating a random double-precision floating-point number.
    let random_double = Random::double(&mut rng);
    println!("🦀 Random double: {}", random_double);

    // Retrieving the current state index (MTI) of the Mersenne Twister RNG.
    let mti_value = Random::mti(&rng);
    println!("🦀 MTI value: {}", mti_value);

    // Generate a random even number.
    let even_number = (0..).map(|_| rng.rand()).find(|&n| n % 2 == 0).unwrap();
    println!("🦀 Random even number: {}", even_number);

    // Pre-generating a large number of random values for performance.
    let mut pre_generated_numbers = Vec::new();
    for _ in 0..1000 {
        pre_generated_numbers.push(rng.rand());
    }
    println!("🦀 Pre-generated random numbers: {:?}", pre_generated_numbers);

    // Comparing `vrd` RNG with Rust's default RNG.
    let default_rng_number = rand::random::<u32>();
    let vrd_rng_number = rng.rand();
    println!("🦀 Default RNG number: {}, `vrd` RNG number: {}", default_rng_number, vrd_rng_number);

}

// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # Macros for the `Random (VRD)` crate.
//!
//! This module contains macros that simplify working with the
//! `Random (VRD)` crate.
//!
//! The macros included in this module allow for quick and easy access
//! to common functionality provided by the Random (VRD) crate.
//!
//! These macros can greatly simplify code that uses the `Random (VRD)`
//! crate, making it easier to read and maintain.
//!
//! ## Macros
//!
//! The following macros are provided by this module:
//!
//! * [`rand_bool!()`](macro.rand_bool.html) - Generate a random boolean with the provided probability
//! * [`rand_bytes!()`](macro.rand_bytes.html) - Generate a vector of random bytes with the provided length
//! * [`rand_char!()`](macro.rand_char.html) - Generate a random char within the range 'a'..='z'
//! * [`rand_choose!()`](macro.rand_choose.html) - Generate a random element from a slice of values
//! * [`rand_float!()`](macro.rand_float.html) - Generate a random float
//! * [`rand_int!()`](macro.rand_int.html) - Generate a random integer within the given range
//! * [`rand_uint!()`](macro.rand_uint.html) - Generate a random 32-bit unsigned integer within the given range
//! * [`rand_double!()`](macro.rand_double.html) - Generate a random double
//! * [`rand_new!()`](macro.rand_new.html) - Generate a new random number
//! * [`rand_pseudo!()`](macro.rand_pseudo.html) - Generate a pseudo random number
//! * [`rand_range!()`](macro.rand_range.html) - Generate a random number within the given range
//! * [`rand_seed!()`](macro.rand_seed.html) - Seed the provided `Random (VRD)` struct with the given value
//! * [`rand_twist!()`](macro.rand_twist.html) - Twist the state of the provided `Random (VRD)` struct
//!

/// Generate a random 32-bit unsigned integer within the given range
/// using the provided `Random (VRD)` struct
#[macro_export]
macro_rules! random_range {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.random_range($min, $max)
    };
}

/// Generate a random boolean with a provided probability.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate vrd;
/// # use vrd::random::Random;
/// # use vrd::rand_bool;
/// # fn main() {
/// # let mut rng = Random::new();
/// // Generates a boolean with 50% probability of being true
/// # let value = rand_bool!(rng, 0.5);
/// # }
/// ```
///
/// # Panics
///
/// Panics if probability is not between 0.0 and 1.0.
#[macro_export]
macro_rules! rand_bool {
    ($rng:expr, $probability:expr) => {{
        let valid_range = 0.0..=1.0;
        assert!(
            valid_range.contains(&$probability),
            "Probability must be between 0.0 and 1.0"
        );
        $rng.bool($probability)
    }};
}

/// Generate a vector of random bytes with the provided length using the
/// provided `Random (VRD)` struct
#[macro_export]
macro_rules! rand_bytes {
    ($rng:expr, $len:expr) => {
        $rng.bytes($len)
    };
}

/// Generate a random char within the range 'a'..='z' using the provided
/// `Random (VRD)` struct
#[macro_export]
macro_rules! rand_char {
    ($rng:expr) => {
        $rng.char()
    };
}

/// Generate a random element from a slice of values using the provided
/// `Random (VRD)` struct
#[macro_export]
macro_rules! rand_choose {
    ($rng:expr, $values:expr) => {
        $rng.choose($values)
    };
}

/// Generate a random float using the provided `Random (VRD)` struct
#[macro_export]
macro_rules! rand_float {
    ($rng:expr) => {
        $rng.float()
    };
}

/// Generate a random integer within the given range using the provided
/// `Random (VRD)` struct
#[macro_export]
macro_rules! rand_int {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.int($min, $max)
    };
}

/// Generate a random 32-bit unsigned integer within the given range
/// using the provided `Random (VRD)` struct
#[macro_export]
macro_rules! rand_uint {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.uint($min, $max)
    };
}

/// Generate a random double using the provided `Random (VRD)` struct
#[macro_export]
macro_rules! rand_double {
    ($rng:expr) => {
        $rng.double()
    };
}

/// Generate a new random number
#[macro_export]
macro_rules! rand_new {
    () => {
        new()
    };
}

/// Generate a random 32-bit unsigned integer using the provided
/// `Random (VRD)` struct
#[macro_export]
macro_rules! rand_pseudo {
    ($rng:expr) => {
        $rng.pseudo()
    };
}

/// Generate a random number within the given range using the provided
/// `Random (VRD)` struct
#[macro_export]
macro_rules! rand_range {
    ($rng:expr, $min:expr, $max:expr) => {
        $rng.range($min, $max)
    };
}

/// Seed the provided `Random (VRD)` struct with the given value
#[macro_export]
macro_rules! rand_seed {
    ($rng:expr, $seed:expr) => {
        $rng.seed($seed)
    };
}

/// Twist the state of the provided `Random (VRD)` struct
#[macro_export]
macro_rules! rand_twist {
    ($rng:expr) => {
        $rng.twist()
    };
}

/// Generates a random alphanumeric character (a-z, A-Z, 0-9).
///
/// # Examples
///
/// ```
/// use vrd::rand_alphanumeric;
/// let mut rng = vrd::random::Random::new();
/// let alphanumeric = rand_alphanumeric!(rng);
/// println!("Random alphanumeric character: {}", alphanumeric);
/// ```
///
/// # Returns
/// A randomly generated alphanumeric character as a `char`.
#[macro_export]
macro_rules! rand_alphanumeric {
    ($rng:expr) => {
        {
            const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            let index = $rng.random_range(0, CHARS.len() as u32) as usize;
            CHARS[index] as char
        }
    };
}

/// Generates a random string of the specified length.
///
/// The generated string can contain lowercase letters (a-z), uppercase letters (A-Z),
/// and digits (0-9).
///
/// # Examples
///
/// ```
/// use vrd::rand_string;
/// let mut rng = vrd::random::Random::new();
/// let random_string = rand_string!(rng, 10);
/// println!("Random string: {}", random_string);
/// ```
///
/// # Arguments
/// * `rng` - A mutable reference to a `Random` instance.
/// * `length` - The desired length of the random string.
///
/// # Returns
/// A randomly generated string of the specified length.
#[macro_export]
macro_rules! rand_string {
    ($rng:expr, $length:expr) => {
        {
            const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            let mut result = String::with_capacity($length);
            for _ in 0..$length {
                let index = $rng.random_range(0, CHARS.len() as u32) as usize;
                result.push(CHARS[index] as char);
            }
            result
        }
    };
}

/// Shuffles a mutable slice randomly.
///
/// # Examples
///
/// ```
/// use vrd::rand_shuffle;
/// let mut rng = vrd::random::Random::new();
/// let mut numbers = [1, 2, 3, 4, 5];
/// rand_shuffle!(rng, &mut numbers);
/// println!("Shuffled numbers: {:?}", numbers);
/// ```
///
/// # Arguments
/// * `rng` - A mutable reference to a `Random` instance.
/// * `slice` - A mutable reference to the slice to be shuffled.
#[macro_export]
macro_rules! rand_shuffle {
    ($rng:expr, $slice:expr) => {
        {
            let len = $slice.len();
            for i in (1..len).rev() {
                let j = $rng.random_range(0, (i + 1) as u32) as usize;
                $slice.swap(i, j);
            }
        }
    };
}

/// Selects a random element from a slice based on the provided weights.
///
/// The weights determine the probability of each element being selected.
/// The probability of an element being selected is proportional to its weight
/// relative to the sum of all weights.
///
/// # Examples
///
/// ```
/// use vrd::rand_weighted_choice;
/// let mut rng = vrd::random::Random::new();
/// let choices = ["A", "B", "C"];
/// let weights = [2, 3, 5];
/// let selected = rand_weighted_choice!(rng, &choices, &weights);
/// println!("Selected element: {}", selected);
/// ```
///
/// # Arguments
/// * `rng` - A mutable reference to a `Random` instance.
/// * `choices` - A reference to the slice of elements to choose from.
/// * `weights` - A reference to the slice of weights corresponding to each element.
///
/// # Panics
/// Panics if `choices` and `weights` have different lengths.
///
/// # Returns
/// A reference to the randomly selected element from `choices`.
#[macro_export]
macro_rules! rand_weighted_choice {
    ($rng:expr, $choices:expr, $weights:expr) => {{
        assert_eq!($choices.len(), $weights.len(), "Choices and weights must have the same length");
        let total_weight: u32 = $weights.iter().sum();
        let mut rnd = $rng.random_range(0, total_weight);
        let mut selected_choice = None;
        for (index, &weight) in $weights.iter().enumerate() {
            if rnd < weight {
                selected_choice = Some(&$choices[index]);
                break; // Exit the loop once the choice is made.
            }
            rnd -= weight;
        }
        selected_choice.expect("Invalid weighted choice")
    }};
}

/// Generate a normally distributed random number with the given mean and standard deviation.
///
/// # Examples
///
/// ```
/// use vrd::rand_normal;
/// let mut rng = vrd::random::Random::new();
/// let normal_number = rand_normal!(rng, 0.0, 1.0);
/// println!("Normal number: {}", normal_number);
/// ```
///
/// # Arguments
/// * `rng` - A mutable reference to a `Random` instance.
/// * `mu` - The mean of the normal distribution.
/// * `sigma` - The standard deviation of the normal distribution.
///
/// # Returns
/// A randomly generated normal distributed number.
#[macro_export]
macro_rules! rand_normal {
    ($rng:expr, $mu:expr, $sigma:expr) => {{
        let u1: f64 = $rng.f64(); // Ensuring f64() method is called on the RNG
        let u2: f64 = $rng.f64(); // Ensuring f64() method is called on the RNG
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        $mu + $sigma * z0
    }};
}

/// Generate a random number from the exponential distribution with the given rate parameter.
///
/// # Examples
///
/// ```
/// use vrd::rand_exponential;
/// let mut rng = vrd::random::Random::new();
/// let exponential_number = rand_exponential!(rng, 2.0);
/// println!("Exponential number: {}", exponential_number);
/// ```
///
#[macro_export]
macro_rules! rand_exponential {
    ($rng:expr, $rate:expr) => {{
        // Ensure the rate parameter is positive.
        if $rate <= 0.0 {
            panic!("The rate parameter must be positive.");
        }

        // Implementation of the inverse CDF method for exponential distribution.
        -1.0 / $rate * (1.0 - $rng.f64()).ln()
    }};
}

/// Generates a random number from a Poisson distribution with the specified mean parameter.
///
/// # Examples
///
/// ```
/// use vrd::rand_poisson;
/// let mut rng = vrd::random::Random::new();
/// let poisson = rand_poisson!(rng, 3.0);
/// println!("Random number from Poisson distribution with mean 3.0: {}", poisson);
/// ```
///
/// # Arguments
/// * `rng` - A mutable reference to a `Random` instance.
/// * `mean` - The mean parameter (lambda) of the Poisson distribution.
///
/// # Returns
/// An `u64` representing a random number from a Poisson distribution.
#[macro_export]
macro_rules! rand_poisson {
    ($rng:expr, $mean:expr) => {
        {
            let mut k = 0;
            let mut p = 1.0;
            let l = f64::exp(-$mean);
            loop {
                k += 1;
                p *= $rng.f64();
                if p < l {
                    break;
                }
            }
            k - 1
        }
    };
}

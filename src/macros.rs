// Copyright © 2023 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
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

/// Generate a random boolean with the provided probability using the
/// provided `Random (VRD)` struct
#[macro_export]
macro_rules! rand_bool {
    ($rng:expr, $probability:expr) => {
        $rng.bool($probability)
    };
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

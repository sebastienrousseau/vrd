// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

use crate::MersenneTwisterConfig;
use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[non_exhaustive]
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
/// The `Random` struct is used to generate random numbers using the Mersenne Twister algorithm.
///
/// This struct maintains an internal state for random number generation and provides methods to generate various types of random numbers.
///
/// # Initialization
/// The random number generator can be initialized with the `new` method, which seeds the generator with a default value.
/// ```
/// use vrd::random::Random;
/// let mut rng = Random::new();
/// ```

/// # Random Number Generation
pub struct Random {
    /// The array of unsigned 32-bit integers used to generate random numbers.
    #[serde(with = "BigArray")]
    pub mt: [u32; 624],
    /// The current index of the array used in the generation of random
    /// numbers
    pub mti: usize,
}

impl Random {
    /// Returns a random bool with a specified probability.
    ///
    /// The `bool` method returns a random boolean value. The probability of returning `true` is determined
    /// by the `probability` parameter. This method is useful for generating random boolean outcomes, like
    /// simulating a coin flip.
    ///
    /// # Arguments
    /// * `probability` - A f64 value representing the probability of the function returning `true`.
    ///                    This should be a value between 0.0 and 1.0, where 0.0 always returns `false` and 1.0 always returns `true`.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_bool = rng.bool(0.5); // 50% chance to get true
    /// ```
    ///
    /// # Panics
    /// Panics if `probability` is not between 0.0 and 1.0.
    pub fn bool(&mut self, probability: f64) -> bool {
        thread_rng().gen_bool(probability)
    }

    /// Generates a vector of random bytes of the specified length.
    ///
    /// # Arguments
    /// * `len` - The length of the byte vector to be generated.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_bytes = rng.bytes(10); // Generates 10 random bytes
    /// println!("Random bytes: {:?}", random_bytes);
    /// ```
    ///
    /// # Returns
    /// A `Vec<u8>` containing `len` randomly generated bytes.
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            let byte = self.rand() as u8;
            res.push(byte);
        }
        res
    }

    /// Generates a random character within the range 'a' to 'z'.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_char = rng.char(); // Generates a random lowercase character
    /// println!("Random char: {}", random_char);
    /// ```
    ///
    /// # Returns
    /// A `char` representing a randomly chosen lowercase letter from 'a' to 'z'.
    pub fn char(&mut self) -> char {
        thread_rng().gen_range('a'..='z')
    }

    /// Selects a random element from a provided slice.
    ///
    /// # Arguments
    /// * `values` - A slice of values from which to select a random element.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let items = [1, 2, 3, 4, 5];
    /// let random_item = rng.choose(&items);
    /// println!("Random item from the array: {:?}", random_item);
    /// ```
    ///
    /// # Returns
    /// An `Option<&T>` which is `Some(&T)` if the slice is not empty, containing a randomly chosen element from the slice.
    /// Returns `None` if the slice is empty.
    ///
    /// # Panics
    /// Does not panic under normal operation.
    pub fn choose<'a, T>(&'a mut self, values: &'a [T]) -> Option<&T> {
        if values.is_empty() {
            return None;
        }
        let mut rng = thread_rng();
        let index = rng.gen_range(0..values.len());
        Some(&values[index])
    }

    /// Generates a random floating-point number in the range [0.0, 1.0).
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_float = rng.float(); // Generates a random float
    /// println!("Random float: {}", random_float);
    /// ```
    ///
    /// # Returns
    /// A `f32` representing a randomly generated floating-point number.
    ///
    /// # Notes
    /// The generated float is inclusive of 0.0 and exclusive of 1.0.
    pub fn float(&mut self) -> f32 {
        thread_rng().gen::<f64>() as f32
    }

    /// Generates a random integer within a specified range.
    ///
    /// # Arguments
    /// * `min` - The lower bound of the range (inclusive).
    /// * `max` - The upper bound of the range (inclusive).
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_int = rng.int(1, 10); // Generates a random integer between 1 and 10
    /// println!("Random integer between 1 and 10: {}", random_int);
    /// ```
    ///
    /// # Returns
    /// An `i32` representing a randomly generated integer within the specified range.
    ///
    /// # Panics
    /// Panics if `min` is greater than `max`.
    pub fn int(&mut self, min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..=max)
    }

    /// Generates a random unsigned integer within a specified range.
    ///
    /// # Arguments
    /// * `min` - The lower bound of the range (inclusive).
    /// * `max` - The upper bound of the range (inclusive).
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_uint = rng.uint(1, 100); // Generates a random unsigned integer between 1 and 100
    /// println!("Random unsigned integer between 1 and 100: {}", random_uint);
    /// ```
    ///
    /// # Returns
    /// A `u32` representing a randomly generated unsigned integer within the specified range.
    ///
    /// # Panics
    /// Panics if `min` is greater than `max`.
    pub fn uint(&mut self, min: u32, max: u32) -> u32 {
        thread_rng().gen_range(min..=max)
    }

    /// Generates a random double-precision floating-point number.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_double = rng.double(); // Generates a random double
    /// println!("Random double: {}", random_double);
    /// ```
    ///
    /// # Returns
    /// A `f64` representing a randomly generated double-precision floating-point number.
    ///
    /// # Notes
    /// The generated double is a number in the range [0.0, 1.0).
    pub fn double(&mut self) -> f64 {
        thread_rng().gen::<f64>()
    }

    /// Returns the current index of the internal state array used in random number generation.
    ///
    /// This method is useful for inspecting the state of the random number generator.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let rng = Random::new();
    /// let current_index = rng.mti();
    /// println!("Current index of the RNG state array: {}", current_index);
    /// ```
    ///
    /// # Returns
    /// The current index (`usize`) of the internal state array (`mt`) used by the Mersenne Twister algorithm.
    pub fn mti(&self) -> usize {
        self.mti
    }

    /// Sets the value of the current index of the internal state array used in random number generation.
    ///
    /// # Arguments
    /// * `value` - The new index value to set for the internal state array.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// rng.set_mti(100); // Sets the current index to 100
    /// assert_eq!(rng.mti(), 100);
    /// ```
    ///
    /// # Notes
    /// - This method allows for manual manipulation of the internal state of the random number generator.
    /// - It should be used with caution, as incorrect values can affect the quality of the generated random numbers.
    pub fn set_mti(&mut self, value: usize) {
        self.mti = value;
    }

    /// Creates a new instance of the `Random` struct, initializing the internal state for random number generation.
    ///
    /// This method seeds the random number generator with a default value obtained from the thread's random number generator.
    ///
    ///
    /// The `new` method initializes the `Random` struct. It sets the initial state of the `mt` array
    /// using a default seed obtained from the system's RNG. This seeding process is crucial for ensuring
    /// that each instance of `Random` produces a unique and unpredictable sequence of numbers.
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new(); // Creates a new instance of Random
    /// let random_number = rng.rand(); // Generates a random number
    /// println!("Random number: {}", random_number);
    /// ```
    ///
    /// # Returns
    /// A new instance of `Random` with its internal state initialized for random number generation.
    ///
    /// # Notes
    /// - The internal state is initialized with a seed value, ensuring that each instance of `Random` produces a unique sequence of random numbers.
    /// - The `new` method ensures that the internal state is appropriately set up for the Mersenne Twister algorithm.
    pub fn new() -> Self {
        const N: usize = 624;
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

    /// Generates a pseudo-random number by combining multiple random number generations.
    ///
    /// This method enhances the randomness by XOR-ing multiple calls to the basic random number generator.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let pseudo_random_number = rng.pseudo(); // Generates a pseudo-random number
    /// println!("Pseudo-random number: {}", pseudo_random_number);
    /// ```
    ///
    /// # Returns
    /// A `u32` representing a pseudo-random number generated by combining multiple random number generations.
    ///
    /// # Notes
    /// - This method is intended to provide a more complex random number by aggregating multiple random number generations.
    /// - It might be useful in scenarios where a single call to the basic random number generator does not provide sufficient randomness.
    pub fn pseudo(&mut self) -> u32 {
        let mut res = self.rand();
        for _ in 0..31 {
            res ^= self.rand();
        }
        res
    }

    /// Generates a random 32-bit unsigned integer using the Mersenne Twister algorithm.
    ///
    /// This method is the core function of the `Random` struct, providing the basic mechanism for generating random numbers.
    ///
    /// The `rand` method generates a random 32-bit number using the current state of the `mt` array.
    /// It applies a series of bitwise transformations for tempering, which refines the output and improves
    /// the statistical properties of the generated numbers.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_number = rng.rand(); // Generates a random 32-bit unsigned integer
    /// println!("Random number: {}", random_number);
    /// ```
    ///
    /// # Returns
    /// A `u32` representing a randomly generated 32-bit unsigned integer.
    ///
    /// # Notes
    /// - This method updates the internal state of the random number generator each time it is called.
    /// - If the internal index (`mti`) reaches the threshold, it automatically reinitializes the internal state array.
    pub fn rand(&mut self) -> u32 {
        let config = MersenneTwisterConfig::default();
        if self.mti >= config.n {
            if self.mti == config.n + 1 + 1 {
                self.seed(5489);
            }
            self.twist();
        }

        let mut y = self.mt[self.mti];
        self.mti += 1;
        y ^= y >> 11;
        y ^= (y << 7) & config.tempering_mask_b;
        y ^= (y << 15) & config.tempering_mask_c;
        y ^= y >> 18;
        y
    }

    /// Generates a random 32-bit unsigned integer within a specified range.
    ///
    /// # Arguments
    /// * `min` - The lower bound of the range (inclusive).
    /// * `max` - The upper bound of the range (exclusive).
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_number = rng.random_range(10, 20); // Generates a random number between 10 (inclusive) and 20 (exclusive)
    /// println!("Random number between 10 and 20: {}", random_number);
    /// ```
    ///
    /// # Returns
    /// A `u32` representing a randomly generated number within the specified range.
    ///
    /// # Panics
    /// Panics if `min` is not less than `max`.
    ///
    /// # Notes
    /// - This method offers a convenient way to specify the range for random number generation.
    pub fn random_range(&mut self, min: u32, max: u32) -> u32 {
        assert!(
            max > min,
            "max must be greater than min for random_range"
        );
        let mut rng = thread_rng(); // Get a thread-local RNG
        rng.gen_range(min..max) // Use the gen_range method for uniform distribution
    }

    /// Generates a random number within a specified range of integer values.
    ///
    /// # Arguments
    /// * `min` - The lower bound of the range (inclusive).
    /// * `max` - The upper bound of the range (inclusive).
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_number = rng.range(1, 100); // Generates a random number between 1 and 100
    /// println!("Random number between 1 and 100: {}", random_number);
    /// ```
    ///
    /// # Returns
    /// An `i32` representing a randomly generated number within the specified range.
    ///
    /// # Panics
    /// Panics if `min` is greater than `max`.
    ///
    /// # Notes
    /// - This method is similar to `int` but allows for a different interface for specifying the range.
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..=max)
    }

    /// Seeds the random number generator with a specified value.
    ///
    /// This method initializes the internal state array of the generator with a given seed, affecting the sequence of random numbers generated.
    ///
    /// The constant 1812433253u32 is used in the seeding process. It's derived from the fractional part
    /// of the square root of 2. This particular value is chosen to provide good statistical properties
    /// for the initial array of numbers.
    ///
    /// # Arguments
    /// * `seed` - A `u32` value used to seed the generator.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// rng.seed(12345); // Seeds the random number generator
    /// let random_number = rng.rand(); // Generates a random number based on the new seed
    /// println!("Random number with seed 12345: {}", random_number);
    /// ```
    ///
    /// # Notes
    /// - Seeding the generator is essential for reproducibility of the random number sequence.
    /// - Different seeds will produce different sequences, while the same seed will always produce the same sequence.
    pub fn seed(&mut self, seed: u32) {
        const N: usize = 624;
        self.mt[0] = seed;
        for i in 1..N {
            self.mt[i] = 1812433253u32
                .checked_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .map_or(u32::MAX, |val| val + i as u32);
        }
        self.mti = N;
    }

    /// Performs the "twisting" operation to update the internal state array of the random number generator.
    ///
    /// This method is a key part of the Mersenne Twister algorithm, and it's called internally when the generator's index exceeds its predefined threshold.
    ///
    /// The `twist` method is a key part of the Mersenne Twister algorithm. It generates a new array of
    /// 624 numbers based on the current array. This method uses bitwise operations and modular arithmetic
    /// to transform the existing numbers into a new set, thereby 'twisting' the current state. This is
    /// essential for maintaining the algorithm's long period and high-quality randomness.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// rng.twist(); // Manually performs a twist operation
    /// ```
    ///
    /// # Notes
    /// - This method modifies the internal state array, ensuring that future random numbers generated are different from the previous ones.
    /// - It is typically not called directly by users of the `Random` struct, as it is automatically managed by the `rand` and other methods.
    pub fn twist(&mut self) {
        let config = MersenneTwisterConfig::default();
        for i in 0..config.n {
            let x = (self.mt[i] & config.upper_mask)
                + (self.mt[(i + 1) % config.n] & config.lower_mask);
            let x_a = x >> 1;
            if x % 2 != 0 {
                self.mt[i] = self.mt[(i + config.m) % config.n]
                    ^ x_a
                    ^ config.matrix_a;
            } else {
                self.mt[i] = self.mt[(i + config.m) % config.n] ^ x_a;
            }
        }
        self.mti = 0;
    }

    /// Generates a random 64-bit signed integer.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_i64 = rng.i64();
    /// println!("Random i64: {}", random_i64);
    /// ```
    ///
    /// # Returns
    /// An `i64` representing a randomly generated 64-bit signed integer.
    pub fn i64(&mut self) -> i64 {
        let high = self.rand() as i64;
        let low = self.rand() as i64;
        (high << 32) | low
    }

    /// Generates a random 64-bit unsigned integer.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_u64 = rng.u64();
    /// println!("Random u64: {}", random_u64);
    /// ```
    ///
    /// # Returns
    /// A `u64` representing a randomly generated 64-bit unsigned integer.
    pub fn u64(&mut self) -> u64 {
        let high = self.rand() as u64;
        let low = self.rand() as u64;
        (high << 32) | low
    }

    /// Generates a random 64-bit floating-point number in the range [0.0, 1.0).
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_f64 = rng.f64();
    /// println!("Random f64: {}", random_f64);
    /// ```
    ///
    /// # Returns
    /// An `f64` representing a randomly generated 64-bit floating-point number.
    pub fn f64(&mut self) -> f64 {
        thread_rng().gen::<f64>()
    }

    /// Generates a random string of the specified length.
    ///
    /// # Arguments
    /// * `length` - The desired length of the random string.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let random_string = rng.string(10);
    /// println!("Random string: {}", random_string);
    /// ```
    ///
    /// # Returns
    /// A `String` representing a randomly generated string of the specified length.
    pub fn string(&mut self, length: usize) -> String {
        let chars: Vec<char> = (0..length)
            .map(|_| {
                let value = self.rand() % 62;
                if value < 10 {
                    (b'0' + value as u8) as char
                } else if value < 36 {
                    (b'a' + value as u8 - 10) as char
                } else {
                    (b'A' + value as u8 - 36) as char
                }
            })
            .collect();
        chars.into_iter().collect()
    }

    /// Generates a random number from a standard normal distribution (mean = 0, stddev = 1).
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let mu = 0.0; // Mean of the standard normal distribution
    /// let sigma = 1.0; // Standard deviation of the standard normal distribution
    /// let normal = rng.normal(mu, sigma);
    /// println!("Random number from standard normal distribution: {}", normal);
    /// ```
    ///
    /// # Returns
    /// An `f64` representing a random number from a standard normal distribution.
    pub fn normal(&mut self, mu: f64, sigma: f64) -> f64 {
        let u1 = self.f64();
        let u2 = self.f64();
        println!("u1: {}", u1);
        println!("u2: {}", u2);
        let z0 = (-2.0 * u1.ln()).sqrt()
            * (2.0 * std::f64::consts::PI * u2).cos();
        mu + sigma * z0
    }

    /// Generates a random number from an exponential distribution with the specified rate parameter.
    ///
    /// # Arguments
    /// * `rate` - The rate parameter (lambda) of the exponential distribution.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let exponential = rng.exponential(1.5);
    /// println!("Random number from exponential distribution with rate 1.5: {}", exponential);
    /// ```
    ///
    /// # Returns
    /// An `f64` representing a random number from an exponential distribution.
    pub fn exponential(&mut self, rate: f64) -> f64 {
        // Implementation of the inverse CDF method
        -1.0 / rate * (1.0 - self.f64()).ln()
    }

    /// Generates a random number from a Poisson distribution with the specified mean parameter.
    ///
    /// # Arguments
    /// * `mean` - The mean parameter (lambda) of the Poisson distribution.
    ///
    /// # Examples
    /// ```
    /// use vrd::random::Random;
    /// let mut rng = Random::new();
    /// let poisson = rng.poisson(3.0);
    /// println!("Random number from Poisson distribution with mean 3.0: {}", poisson);
    /// ```
    ///
    /// # Returns
    /// An `u64` representing a random number from a Poisson distribution.
    pub fn poisson(&mut self, mean: f64) -> u64 {
        let mut k = 0;
        let mut p = 1.0;
        let l = (-mean).exp();
        loop {
            k += 1;
            p *= self.f64();
            if p < l {
                break;
            }
        }
        k - 1
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

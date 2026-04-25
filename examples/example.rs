// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! End-to-end usage example for the `vrd` crate.

use vrd::{
    rand_alphanumeric, rand_bool, rand_bytes, rand_char, rand_choose,
    rand_exponential, rand_float, rand_int, rand_normal, rand_poisson,
    rand_range, rand_seed, rand_shuffle, rand_string, rand_uint,
    rand_weighted_choice, random_range, Random, VrdError,
};

fn demonstrate_basic() {
    println!("\n🦀 Basic generation");
    let mut rng = Random::new();
    println!("  Random::new():       ✅ {rng}");
    println!("  rand():              ✅ {}", rng.rand());
    println!("  u64():               ✅ {}", rng.u64());
    println!("  i64():               ✅ {}", rng.i64());
    println!("  float():             ✅ {}", rng.float());
    println!("  double():            ✅ {}", rng.double());
    println!("  int(1, 10):          ✅ {}", rng.int(1, 10));
    println!("  uint(1, 100):        ✅ {}", rng.uint(1, 100));
    println!("  bool(0.5):           ✅ {}", rng.bool(0.5));
    println!("  char():              ✅ {}", rng.char());
    println!("  bytes(8):            ✅ {:?}", rng.bytes(8));
    println!("  string(10):          ✅ {}", rng.string(10));
}

fn demonstrate_macros() {
    println!("\n🦀 Macro forms");
    let mut rng = Random::new();
    println!("  rand_int!:           ✅ {}", rand_int!(rng, 0, 100));
    println!("  rand_uint!:          ✅ {}", rand_uint!(rng, 0, 100));
    println!("  rand_float!:         ✅ {}", rand_float!(rng));
    println!("  rand_bool!:          ✅ {}", rand_bool!(rng, 0.5));
    println!("  rand_char!:          ✅ {}", rand_char!(rng));
    println!("  rand_bytes!:         ✅ {:?}", rand_bytes!(rng, 4));
    println!("  rand_alphanumeric!:  ✅ {}", rand_alphanumeric!(rng));
    println!("  rand_string!:        ✅ {}", rand_string!(rng, 8));
    println!("  rand_range!:         ✅ {}", rand_range!(rng, 1, 100));
    println!("  random_range!:       ✅ {}", random_range!(rng, 0, 1000));

    let values = &[1, 2, 3, 4, 5];
    println!("  rand_choose!:        ✅ {:?}", rand_choose!(rng, values));

    let choices = ["A", "B", "C"];
    let weights = [2u32, 3, 5];
    println!(
        "  rand_weighted_choice!: ✅ {}",
        rand_weighted_choice!(rng, &choices, &weights)
    );
}

fn demonstrate_shuffle_and_sample() {
    println!("\n🦀 Shuffling and sampling");
    let mut rng = Random::new();
    let pool = &[10, 20, 30, 40, 50, 60];
    println!("  rand_slice(3):       ✅ {:?}", rng.rand_slice(pool, 3));
    println!("  sample(3):           ✅ {:?}", rng.sample(pool, 3));
    println!(
        "  sample_with_repl(3): ✅ {:?}",
        rng.sample_with_replacement(pool, 3)
    );

    let mut to_shuffle = [1, 2, 3, 4, 5, 6, 7, 8];
    rand_shuffle!(rng, &mut to_shuffle);
    println!("  rand_shuffle!:       ✅ {:?}", to_shuffle);
}

fn demonstrate_distributions() {
    println!("\n🦀 Statistical distributions");
    let mut rng = Random::new();
    println!("  normal(0, 1):        ✅ {}", rand_normal!(rng, 0.0, 1.0));
    println!("  exponential(1):      ✅ {}", rand_exponential!(rng, 1.0));
    println!("  poisson(3):          ✅ {}", rand_poisson!(rng, 3.0));
}

fn demonstrate_seeding() {
    println!("\n🦀 Deterministic seeding");
    let mut a = Random::from_u64_seed(0xCAFE_BABE);
    let mut b = Random::from_u64_seed(0xCAFE_BABE);
    println!("  Equal first draws:   ✅ {} == {}", a.rand(), b.rand());

    let mut rng = Random::new();
    rand_seed!(rng, 42);
    println!("  rand_seed!(42) then rand(): ✅ {}", rng.rand());
}

fn demonstrate_mersenne_twister() {
    println!("\n🦀 Optional Mersenne-Twister backend");
    let mut mt = Random::new_mersenne_twister();
    println!("  Random::new_mersenne_twister(): ✅ {mt}");
    println!("  rand():                          ✅ {}", mt.rand());
    println!("  mti():                           ✅ {}", mt.mti());
}

fn demonstrate_errors() {
    println!("\n🦀 Error handling");
    fn produce_error() -> Result<(), VrdError> {
        Err(VrdError::GeneralError("simulated failure"))
    }
    match produce_error() {
        Ok(()) => unreachable!(),
        Err(e) => println!("  Caught:               ✅ {e}"),
    }
}

fn main() {
    demonstrate_basic();
    demonstrate_macros();
    demonstrate_shuffle_and_sample();
    demonstrate_distributions();
    demonstrate_seeding();
    demonstrate_mersenne_twister();
    demonstrate_errors();
}

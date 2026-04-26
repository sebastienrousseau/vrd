// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tour of every macro in the `rand_*` family.
//!
//! Run: `cargo run --example macros`

#[path = "support.rs"]
mod support;

use vrd::{
    rand_alphanumeric, rand_bool, rand_bytes, rand_char, rand_choose,
    rand_double, rand_exponential, rand_float, rand_int, rand_normal,
    rand_poisson, rand_range, rand_seed, rand_shuffle, rand_string,
    rand_uint, rand_weighted_choice, random_range, Random,
};

fn main() {
    support::header("vrd -- macros");

    support::task_with_output("Scalars and ranges", || {
        let mut rng = Random::from_u64_seed(2024);
        vec![
            format!("rand_int!(-10, 10)         = {}", rand_int!(rng, -10, 10)),
            format!("rand_uint!(0, 100)         = {}", rand_uint!(rng, 0, 100)),
            format!("rand_range!(1, 6)          = {}", rand_range!(rng, 1, 6)),
            format!("random_range!(0, 1000)     = {}", random_range!(rng, 0, 1000)),
            format!("rand_float!()              = {:.6}", rand_float!(rng)),
            format!("rand_double!()             = {:.6}", rand_double!(rng)),
        ]
    });

    support::task_with_output("Bools, chars, strings, bytes", || {
        let mut rng = Random::from_u64_seed(2024);
        vec![
            format!("rand_bool!(0.5)            = {}", rand_bool!(rng, 0.5)),
            format!("rand_char!()               = '{}'", rand_char!(rng)),
            format!("rand_alphanumeric!()       = '{}'", rand_alphanumeric!(rng)),
            format!("rand_string!(8)            = {}", rand_string!(rng, 8)),
            format!("rand_bytes!(8)             = {:02x?}", rand_bytes!(rng, 8)),
        ]
    });

    support::task_with_output("Slice operations", || {
        let mut rng = Random::from_u64_seed(2024);
        let pool = [10, 20, 30, 40, 50];
        let mut nums = [1, 2, 3, 4, 5];
        rand_shuffle!(rng, &mut nums);
        let weights = [10u32, 80, 10];
        let labels = ["A", "B", "C"];
        vec![
            format!("rand_choose!(&[{:?}])      = {:?}", pool, rand_choose!(rng, &pool)),
            format!("rand_shuffle! result       = {nums:?}"),
            format!(
                "rand_weighted_choice! (10/80/10) = {}",
                rand_weighted_choice!(rng, &labels, &weights)
            ),
        ]
    });

    support::task_with_output("Distributions", || {
        let mut rng = Random::from_u64_seed(2024);
        vec![
            format!(
                "rand_normal!(0, 1)         = {:.6}",
                rand_normal!(rng, 0.0, 1.0)
            ),
            format!(
                "rand_exponential!(1.0)     = {:.6}",
                rand_exponential!(rng, 1.0)
            ),
            format!(
                "rand_poisson!(3.0)         = {}",
                rand_poisson!(rng, 3.0)
            ),
        ]
    });

    support::task_with_output("Re-seeding via the macro", || {
        let mut rng = Random::from_u64_seed(0);
        let before = rng.rand();
        rand_seed!(rng, 42);
        let after = rng.rand();
        vec![
            format!("before re-seed = {before}"),
            format!("after  re-seed = {after}"),
        ]
    });

    support::summary(5);
}

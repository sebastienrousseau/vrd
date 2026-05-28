// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `Distribution<T>` trait - built-in samplers and a
//! user-defined one. Closes the example-coverage gap for the
//! trait surface added in issue #84.
//!
//! Run: `cargo run --example distribution`

#[path = "support.rs"]
mod support;

use vrd::distribution::{Exponential, Normal, Poisson, Uniform};
use vrd::{Distribution, Random};

/// User-defined distribution: Bernoulli(p) returns true with
/// probability `p`. Any type can be the sample target - bool
/// here; could just as easily be `u8`, a struct, etc.
struct Bernoulli {
    p: f64,
}

impl Distribution<bool> for Bernoulli {
    fn sample(&self, rng: &mut Random) -> bool {
        rng.double() < self.p
    }
}

fn main() {
    support::header("vrd -- distribution");

    support::task_with_output(
        "Built-in: Normal(mu, sigma) via the trait",
        || {
            let mut rng = Random::from_u64_seed(42);
            let n = Normal {
                mu: 0.0,
                sigma: 1.0,
            };
            (0..5)
                .map(|i| {
                    format!("draw {i}: {:>9.5}", n.sample(&mut rng))
                })
                .collect()
        },
    );

    support::task_with_output(
        "Built-in: Exponential / Uniform / Poisson",
        || {
            let mut rng = Random::from_u64_seed(42);
            let e = Exponential { rate: 1.5 };
            let u = Uniform {
                low: -1.0,
                high: 1.0,
            };
            let p = Poisson { mean: 3.0 };
            vec![
                format!("Exponential(1.5) = {:.5}", e.sample(&mut rng)),
                format!(
                    "Uniform(-1, 1)   = {:>8.5}",
                    u.sample(&mut rng)
                ),
                format!("Poisson(3.0)     = {}", p.sample(&mut rng)),
            ]
        },
    );

    support::task_with_output(
        "User-defined: Bernoulli(0.7) sampled 10 times",
        || {
            let mut rng = Random::from_u64_seed(42);
            let b = Bernoulli { p: 0.7 };
            let xs: Vec<bool> = b.samples(&mut rng).take(10).collect();
            let trues = xs.iter().filter(|&&x| x).count();
            vec![
                format!("samples = {:?}", xs),
                format!("trues   = {}/10 (expected ~7)", trues),
            ]
        },
    );

    support::task_with_output(
        "Iterator: .samples(rng).take(n).collect()",
        || {
            let mut rng = Random::from_u64_seed(99);
            let n = Normal {
                mu: 0.0,
                sigma: 1.0,
            };
            let xs: Vec<f64> = n.samples(&mut rng).take(1000).collect();
            let mean: f64 = xs.iter().sum::<f64>() / xs.len() as f64;
            let var: f64 =
                xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
                    / xs.len() as f64;
            vec![
                format!("N = {} samples", xs.len()),
                format!("empirical mean   = {:>8.5}", mean),
                format!("empirical stddev = {:>8.5}", var.sqrt()),
                "(both should be ~0 and ~1 for N(0,1))".to_string(),
            ]
        },
    );

    support::summary(4);
}

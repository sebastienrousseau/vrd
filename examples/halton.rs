// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Halton low-discrepancy sequence for Monte Carlo integration.
//! Side-by-side π estimate against the Random PRNG to show how
//! variance scales differently.
//!
//! Run: `cargo run --example halton --features quasirandom`

#[path = "support.rs"]
mod support;

use vrd::quasirandom::HaltonSequence;
use vrd::Random;

fn main() {
    support::header("vrd -- halton");

    support::task_with_output("First 6 points of Halton(2-D)", || {
        let mut h = HaltonSequence::new(2);
        (0..6)
            .map(|i| {
                let p = h.next_point::<2>();
                format!("point {i}: ({:>10.6}, {:>10.6})", p[0], p[1])
            })
            .collect()
    });

    support::task_with_output(
        "π estimate via Monte Carlo, N = 4096",
        || {
            const N: usize = 4096;

            // Halton
            let mut h = HaltonSequence::new(2);
            let mut h_in = 0;
            for _ in 0..N {
                let p = h.next_point::<2>();
                if p[0] * p[0] + p[1] * p[1] < 1.0 {
                    h_in += 1;
                }
            }
            let pi_h = 4.0 * (h_in as f64) / (N as f64);

            // PRNG (Xoshiro256++)
            let mut rng = Random::from_u64_seed(42);
            let mut r_in = 0;
            for _ in 0..N {
                let x = rng.double();
                let y = rng.double();
                if x * x + y * y < 1.0 {
                    r_in += 1;
                }
            }
            let pi_r = 4.0 * (r_in as f64) / (N as f64);

            vec![
                format!("Halton(2) π = {pi_h:.6}"),
                format!("Random    π = {pi_r:.6}"),
                format!(
                    "|π - true| (Halton) = {:.6}",
                    (pi_h - std::f64::consts::PI).abs()
                ),
                format!(
                    "|π - true| (Random) = {:.6}",
                    (pi_r - std::f64::consts::PI).abs()
                ),
            ]
        },
    );

    support::summary(2);
}

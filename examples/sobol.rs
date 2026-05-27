// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Sobol low-discrepancy sequence — typically the best choice in
//! higher dimensions. Shows the standard convention `point 0 =
//! (0, 0, ..., 0)`.
//!
//! Run: `cargo run --example sobol --features quasirandom`

#[path = "support.rs"]
mod support;

use vrd::quasirandom::SobolSequence;

fn main() {
    support::header("vrd -- sobol");

    support::task_with_output("First 8 points of Sobol(2-D)", || {
        let mut s = SobolSequence::new(2);
        (0..8)
            .map(|i| {
                let p = s.next_point::<2>();
                format!("point {i}: ({:>10.6}, {:>10.6})", p[0], p[1])
            })
            .collect()
    });

    support::task_with_output(
        "First 6 points of Sobol(6-D) — all dims supported",
        || {
            let mut s = SobolSequence::new(6);
            (0..6)
                .map(|i| {
                    let p = s.next_point::<6>();
                    format!(
                        "i={i}: [{:.3}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3}]",
                        p[0], p[1], p[2], p[3], p[4], p[5]
                    )
                })
                .collect()
        },
    );

    support::task_with_output(
        "π estimate via Sobol(2), N = 8192",
        || {
            const N: usize = 8192;
            let mut s = SobolSequence::new(2);
            let mut inside = 0;
            for _ in 0..N {
                let p = s.next_point::<2>();
                if p[0] * p[0] + p[1] * p[1] < 1.0 {
                    inside += 1;
                }
            }
            let pi = 4.0 * (inside as f64) / (N as f64);
            vec![
                format!("Sobol(2) π = {pi:.6}"),
                format!(
                    "|π - true| = {:.6}",
                    (pi - std::f64::consts::PI).abs()
                ),
            ]
        },
    );

    support::summary(3);
}

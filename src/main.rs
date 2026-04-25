// Copyright © 2023-2026 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # vrd CLI
//!
//! Tiny command-line driver. Prints `count` random `u32`s, one per line.
//!
//! ```text
//! vrd            # 1 random u32
//! vrd 16         # 16 random u32s, one per line
//! vrd 8 0xCAFE   # 8 random u32s, deterministically seeded with 0xCAFE
//! ```

use std::env;
use std::io::{self, BufWriter, Write};

use vrd::Random;

fn parse_int(arg: &str) -> Option<u64> {
    if let Some(stripped) = arg.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16).ok()
    } else {
        arg.parse::<u64>().ok()
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let count: usize = args
        .first()
        .and_then(|a| parse_int(a))
        .unwrap_or(1) as usize;

    let mut rng = match args.get(1).and_then(|a| parse_int(a)) {
        Some(seed) => Random::from_u64_seed(seed),
        None => Random::new(),
    };

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for _ in 0..count {
        let _ = writeln!(out, "{}", rng.rand());
    }
}

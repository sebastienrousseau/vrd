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

/// The core logic of the CLI, extracted for testability.
pub fn run_cli(args: Vec<String>, writer: &mut impl Write) -> io::Result<()> {
    let count: usize = args
        .first()
        .and_then(|a| parse_int(a))
        .unwrap_or(1) as usize;

    let mut rng = match args.get(1).and_then(|a| parse_int(a)) {
        Some(seed) => Random::from_u64_seed(seed),
        None => Random::new(),
    };

    let mut out = BufWriter::new(writer);
    for _ in 0..count {
        writeln!(out, "{}", rng.rand())?;
    }
    out.flush()?;
    Ok(())
}

fn main() {
    #[cfg(test)]
    if env::var("VRD_TEST_MAIN").is_err() {
        return;
    }
    let args: Vec<String> = env::args().skip(1).collect();
    if let Err(e) = run_cli(args, &mut io::stdout()) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int("10"), Some(10));
        assert_eq!(parse_int("0xA"), Some(10));
        assert_eq!(parse_int("invalid"), None);
    }

    #[test]
    fn test_run_cli_exhaustive() {
        let mut buffer = Cursor::new(Vec::new());
        // Default
        run_cli(vec![], &mut buffer).unwrap();
        // Count only
        run_cli(vec!["5".to_string()], &mut buffer).unwrap();
        // Count and seed
        run_cli(vec!["1".to_string(), "0x1234".to_string()], &mut buffer).unwrap();
        // Invalid count, invalid seed
        run_cli(vec!["invalid".to_string(), "invalid".to_string()], &mut buffer).unwrap();
    }

    #[test]
    fn test_run_cli_write_error() {
        struct FailingWriter;
        impl Write for FailingWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::Other, "write error"))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut writer = FailingWriter;
        let res = run_cli(vec!["1".to_string()], &mut writer);
        assert!(res.is_err());
    }

    #[test]
    fn test_actual_main_via_env() {
        env::set_var("VRD_TEST_MAIN", "1");
        main();
    }
}

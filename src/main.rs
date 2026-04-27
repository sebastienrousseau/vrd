// Copyright © 2023-2026 vrd. All rights reserved.
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

/// Parses an integer arg, supporting decimal and `0x`-prefixed hex.
fn parse_int(arg: &str) -> Option<u64> {
    if let Some(stripped) = arg.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16).ok()
    } else {
        arg.parse::<u64>().ok()
    }
}

/// The core logic of the CLI, extracted for testability.
pub fn run_cli(
    args: Vec<String>,
    writer: &mut impl Write,
) -> io::Result<()> {
    let count: usize =
        args.first().and_then(|a| parse_int(a)).unwrap_or(1) as usize;

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

/// Computes the process exit code from `run_cli`'s outcome and logs
/// any error to `stderr`. Extracted from `main()` so the error branch
/// is testable without faking `io::stdout()` end-to-end.
fn dispatch(
    args: Vec<String>,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> i32 {
    match run_cli(args, stdout) {
        Ok(()) => 0,
        Err(e) => {
            let _ = writeln!(stderr, "Error: {}", e);
            1
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let code = dispatch(args, &mut io::stdout(), &mut io::stderr());
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Always-failing writer used to drive the error paths in
    /// `run_cli` and `dispatch`.
    struct FailingWriter;
    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write fail"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_parse_int_decimal_hex_invalid() {
        assert_eq!(parse_int("10"), Some(10));
        assert_eq!(parse_int("0xA"), Some(10));
        assert_eq!(parse_int("0xCAFE"), Some(0xCAFE));
        assert_eq!(parse_int("invalid"), None);
        assert_eq!(parse_int("0xZZ"), None);
        assert_eq!(parse_int(""), None);
    }

    #[test]
    fn test_run_cli_default_emits_one_line() {
        let mut buf = Cursor::new(Vec::new());
        run_cli(vec![], &mut buf).unwrap();
        let out = String::from_utf8(buf.into_inner()).unwrap();
        assert_eq!(out.lines().count(), 1);
    }

    #[test]
    fn test_run_cli_explicit_count() {
        let mut buf = Cursor::new(Vec::new());
        run_cli(vec!["5".to_string()], &mut buf).unwrap();
        let out = String::from_utf8(buf.into_inner()).unwrap();
        assert_eq!(out.lines().count(), 5);
    }

    #[test]
    fn test_run_cli_seed_is_deterministic() {
        let mut a = Cursor::new(Vec::new());
        let mut b = Cursor::new(Vec::new());
        run_cli(vec!["3".to_string(), "0x1234".to_string()], &mut a)
            .unwrap();
        run_cli(vec!["3".to_string(), "0x1234".to_string()], &mut b)
            .unwrap();
        assert_eq!(a.into_inner(), b.into_inner());
    }

    #[test]
    fn test_run_cli_invalid_args_use_defaults() {
        let mut buf = Cursor::new(Vec::new());
        run_cli(
            vec!["bogus".to_string(), "alsobogus".to_string()],
            &mut buf,
        )
        .unwrap();
        let out = String::from_utf8(buf.into_inner()).unwrap();
        // Default count when arg is unparseable.
        assert_eq!(out.lines().count(), 1);
    }

    #[test]
    fn test_run_cli_propagates_write_error() {
        let mut writer = FailingWriter;
        let res = run_cli(vec!["1".to_string()], &mut writer);
        assert!(res.is_err());
    }

    #[test]
    fn test_dispatch_ok_returns_zero() {
        let mut out = Cursor::new(Vec::new());
        let mut err = Cursor::new(Vec::new());
        let code = dispatch(vec![], &mut out, &mut err);
        assert_eq!(code, 0);
        assert!(err.get_ref().is_empty());
        assert!(!out.get_ref().is_empty());
    }

    #[test]
    fn test_dispatch_error_returns_one_and_logs() {
        let mut err = Cursor::new(Vec::new());
        let code = dispatch(vec![], &mut FailingWriter, &mut err);
        assert_eq!(code, 1);
        let log = String::from_utf8(err.into_inner()).unwrap();
        assert!(log.starts_with("Error:"), "stderr was: {log}");
    }
}

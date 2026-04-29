// Copyright © 2023-2026 vrd. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Integration smoke tests for the `vrd` CLI binary.
//!
//! Exercises the `fn main()` glue (argv → `dispatch` → `process::exit`)
//! that can't be hit by unit tests because it owns the real process
//! handles. `assert_cmd` builds and runs the binary out of the workspace
//! target directory.

#![cfg(feature = "std")]

use assert_cmd::Command;

fn vrd() -> Command {
    Command::cargo_bin("vrd").expect("vrd binary should build")
}

#[test]
fn cli_default_emits_one_line() {
    let out = vrd().assert().success();
    let stdout = std::str::from_utf8(&out.get_output().stdout).unwrap();
    assert_eq!(stdout.lines().count(), 1, "stdout was: {stdout:?}");
}

#[test]
fn cli_explicit_count_emits_n_lines() {
    let out = vrd().arg("4").assert().success();
    let stdout = std::str::from_utf8(&out.get_output().stdout).unwrap();
    assert_eq!(stdout.lines().count(), 4);
}

#[test]
fn cli_seeded_is_deterministic() {
    let a = vrd()
        .args(["3", "0xCAFE"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let b = vrd()
        .args(["3", "0xCAFE"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    assert_eq!(a, b);
}

#[test]
fn cli_invalid_args_use_defaults() {
    let out = vrd().args(["bogus", "alsobogus"]).assert().success();
    let stdout = std::str::from_utf8(&out.get_output().stdout).unwrap();
    assert_eq!(stdout.lines().count(), 1);
}

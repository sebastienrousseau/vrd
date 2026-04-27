# Contributing to `vrd`

Thanks for considering a contribution. This file documents the workflow
that CI enforces — follow it locally and your PR clears the gates on the
first run.

## Before submitting a PR

Run all four checks. Any failure blocks merge.

```bash
cargo fmt --check                                                 # formatting
cargo clippy --all-targets --all-features -- -D warnings          # lints
cargo test                                                        # unit + doc tests
cargo build --no-default-features                                 # no_std build
```

Optional but recommended:

```bash
cargo audit                                                       # security advisories
cargo run --example all                                           # smoke-run every example
cargo check --target thumbv7em-none-eabihf --no-default-features  # embedded build
```

## Test coverage

Local measurement uses `cargo-llvm-cov`. The canonical command is:

```bash
cargo llvm-cov --all-features --summary-only --ignore-filename-regex='main\.rs'
```

`main.rs` is excluded because it is a thin binary shim
(`run_cli` + `process::exit(dispatch(...))`). The shim's logic is
covered by `dispatch`-level unit tests via a `FailingWriter`; the
`process::exit` glue itself cannot be exercised without faking real
stdio handles end-to-end and adds nothing useful to the coverage
signal.

With `main.rs` excluded, the library sits at **≥97% regions / ≥97% lines / ≥98% on every file except `random.rs`**.

`random.rs` ceilings at **~96.4% regions** on stable. The remaining
gap is `cargo-llvm-cov` accounting on multi-line expressions
(closing braces of `match` arms, multi-line return signatures, a few
`#[cfg]`-gated branches that only one feature matrix exercises). The
covered behavioural surface is ≥99%; the gap is mechanical, not
behavioural. Closing it would require either single-line collapsing
(readability regression) or `#[coverage(off)]` (nightly-only). New
PRs targeting `random.rs` should keep the file at or above 96%.

## Commit conventions

- **Conventional Commits**: prefix with `feat:`, `fix:`, `chore:`,
  `refactor:`, `docs:`, `test:`, `perf:`, `build:`, `ci:`, or `revert:`.
  Append `!` to mark a breaking change (e.g. `feat!: drop Random::pseudo`).
- **Sign every commit**: `git commit -S` (configure `user.signingkey`
  and `commit.gpgsign = true` once and forget it).
- **Single logical unit per commit**. Mechanical refactors and
  behavioural changes go in separate commits so blame stays clean.
- **AI-assisted contributions**: the global `commit-msg` hook
  auto-injects an `Assisted-by:` trailer per the Linux kernel
  coding-assistants standard. Don't add it by hand. Human contributors
  remain accountable for the code regardless of provenance.

## Pull request hygiene

- Title under 70 characters.
- Body must include `## Summary` and `## Test Plan` sections.
- New behaviour ships with new tests in the same PR.
- New dependencies require explicit justification in the PR body.

## Reporting bugs

Open an issue at <https://github.com/sebastienrousseau/vrd/issues> with:

- A descriptive title.
- Minimal reproduction (a few-line snippet beats prose).
- Expected vs. observed output.
- Rust version (`rustc --version`) and `cargo audit` status if relevant.

## License

By contributing, you agree your work is dual-licensed under
[Apache 2.0](LICENSE-APACHE) **or** [MIT](LICENSE-MIT) at the user's
option, matching the rest of the crate.

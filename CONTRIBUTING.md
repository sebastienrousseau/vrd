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
cargo llvm-cov --all-features --summary-only
```

The library sits at **≥98% regions, ≥98% lines, 100% functions**
across every file:

| File | Regions |
| :-- | :-: |
| `lib.rs` | 100% |
| `xoshiro.rs` | 100% |
| `main.rs` | ≥98% |
| `random.rs` | ≥98% |
| `mersenne_twister.rs` | ≥98% |

The few uncovered lines that remain across all files are `cargo-llvm-cov`
accounting artifacts on `#[should_panic]` test bodies (the closing brace
is unreachable by construction) and on multi-line expressions. The
covered behavioural surface is ≥99.5%. New PRs must keep every file at
or above 98% regions.

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

<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Security policy

## Reporting

Email **sebastian.rousseau@gmail.com** with the subject
`vrd security:` and a description of the issue. Do **not**
open a public GitHub issue for security vulnerabilities. PGP
key on request.

You'll get an acknowledgement within 72 hours and a fix /
mitigation timeline within one week.

If a published version is affected, we'll yank it from
crates.io and publish a patched version inside 7 days; CVE
assignment follows.

## Threat model

vrd is a non-cryptographic PRNG by default. Treat the default
backends (`Xoshiro256++`, `MT19937`, `PCG32`, `PCG64`) as
deterministic functions of their seeds - **predictable** to
anyone who observes enough output. They are unsuitable for:

- session tokens, API keys, password reset links
- CSRF tokens, nonces, IVs
- any value an attacker would benefit from predicting

For all of the above, enable the `crypto` feature and construct
via:

```rust
# #[cfg(all(feature = "crypto", feature = "std"))]
# {
use vrd::Random;
let mut rng = Random::new_secure();          // OS-seeded ChaCha20
let token = rng.base64_token(32);            // 256 bits CSPRNG
# let _ = token;
# }
```

The `crypto` backend wraps `rand_chacha::ChaCha20Rng` - the
rand-ecosystem reference ChaCha20 implementation. vrd does
not roll its own crypto; we vendor the audited proof.

## What vrd guarantees

- **Panic-free public API on arbitrary input** (with documented
  exceptions: `low >= high` in bounded sampling; non-finite
  parameters to `normal` / `exponential`; these are programmer
  errors and panic with named-parameter messages). Enforced
  by the libFuzzer harness in `fuzz/`.
- **No `unsafe` outside `src/xoshiro_simd.rs`**. The crate
  root declares `#![deny(unsafe_code)]`; only the SIMD module
  lifts it locally with documented `SAFETY:` comments per
  block.
- **No silent integer overflow / underflow / divide-by-zero**
  in any backend's hot path. `cargo bench --features simd`
  runs under `debug-assertions = false` and `overflow-checks =
  false`; the same code runs cleanly under `debug-assertions =
  true` in `cargo test`.
- **Output stability across patch releases** - see
  [`doc/POLICIES.md`](doc/POLICIES.md). Algorithm swaps bump
  the minor version with a changelog flag.
- **Supply chain**: `cargo deny check` runs on every PR
  (`make deny`); `cargo audit` against the RUSTSEC database;
  `cargo about generate` produces `NOTICE` listing every
  third-party crate vrd redistributes.

## What vrd does NOT guarantee

- **Cryptographic security on the default backend.** Enable
  `crypto`.
- **Constant-time execution.** vrd's backends are not designed
  to resist timing side-channels. ChaCha20 (via rand_chacha) is
  constant-time in its core, but vrd's overall code path is
  not audited for constant-time properties.
- **Forward secrecy on the SIMD path.** The `simd` lane
  derivation reads the scalar state via `state_snapshot()`; if
  the scalar state is compromised, all lanes are derivable.
  For crypto-grade scenarios stay on the scalar `Random::new_secure()`
  path.

## Supported versions

We support the latest released minor version plus the previous
one. Security fixes land on both; older minors are end-of-life
and receive no fixes.

| Version | Status |
| :-- | :-- |
| 0.0.12 | ✓ supported |
| 0.0.11 | ✓ supported (until 0.0.13 ships) |
| 0.0.10 and earlier | ✗ end-of-life |

## Compliance & supply chain

- **REUSE 3.0** - every source file carries an SPDX header.
  See [`REUSE.toml`](REUSE.toml).
- **NOTICE** - generated from `about.toml` + `about.hbs` by
  `cargo about generate` (run `make notice`). Lists every
  third-party crate vrd redistributes with its license text.
- **SBOM** - `make sbom` writes `SBOM.txt` (CycloneDX-style
  dependency tree). Ship inside every release tarball.
- **OpenSSF Security Scorecard** - weekly scan, results badge
  in README. Aim is to keep the score above 8/10.

## Disclosure timeline policy

- **Day 0**: report received, ack within 72 hours.
- **Day ≤7**: fix prepared, advisory drafted, patch version
  prepared.
- **Day 7-14**: coordinated disclosure with affected users
  (Cargo packagers via crates.io advisory, downstream redists
  via direct email if known).
- **Day 14**: public advisory + patched release.

We'll shorten the timeline for actively-exploited issues; if
no known exploitation, we follow the 14-day floor to give
distros time to land the fix.

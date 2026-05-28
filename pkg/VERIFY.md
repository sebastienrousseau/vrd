<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# `pkg/VERIFY.md` - verification cookbook

How downstream consumers verify the integrity and provenance
of vrd's published artefacts. Scaffolded for v0.0.11; the real
signing pipeline activates when `release-binaries.yml` lands
(see [`../PLAN.md`](../PLAN.md), Phase 6).

## crates.io tarball (current)

The crates.io tarball for `vrd v0.0.12` is signed by
crates.io itself; the SHA256 is recorded in `Cargo.lock` for
any project that depends on vrd.

```bash
# Compare local sum against crates.io API:
EXPECTED=$(curl -s https://crates.io/api/v1/crates/vrd/0.0.12 | jq -r '.version.checksum')
LOCAL=$(shasum -a 256 ~/.cargo/registry/cache/index.crates.io-*/vrd-0.0.12.crate | cut -d' ' -f1)
[ "$EXPECTED" = "$LOCAL" ] && echo "match" || echo "MISMATCH"
```

## GitHub release tarball (current)

GitHub generates source tarballs automatically per tag. Verify
against the published SHA256 in the release notes:

```bash
VERSION=0.0.12
gh release download "v${VERSION}" --repo sebastienrousseau/vrd \
    --pattern "*.tar.gz"
shasum -a 256 "vrd-${VERSION}.tar.gz"
```

## cosign keyless (future - Phase 6)

Every binary artefact will ship a `.sig` and `.pem` next to it.
Verify against the GitHub Actions OIDC issuer:

```bash
cosign verify-blob \
    --certificate-identity-regexp 'https://github.com/sebastienrousseau/vrd/' \
    --certificate-oidc-issuer 'https://token.actions.githubusercontent.com' \
    --certificate vrd.pem \
    --signature   vrd.sig \
    vrd
```

The certificate-identity-regexp ensures the signature came
from a workflow run inside the `sebastienrousseau/vrd` repo;
the OIDC issuer pins it to GitHub Actions (no impersonation).

## SLSA L3 build provenance (future - Phase 6)

Each release will carry a SLSA L3 attestation verifiable via
`gh attestation verify`:

```bash
gh attestation verify vrd \
    --owner sebastienrousseau \
    --repo vrd
```

The attestation records the exact workflow run that produced
the binary, the source commit, and the build environment. This
gives downstream consumers cryptographic evidence that the
bytes they're installing came from the vrd repo on a specific
commit.

## Docker image (future - Phase 6)

```bash
docker pull ghcr.io/sebastienrousseau/vrd:0.0.12
cosign verify \
    --certificate-identity-regexp 'https://github.com/sebastienrousseau/vrd/' \
    --certificate-oidc-issuer 'https://token.actions.githubusercontent.com' \
    ghcr.io/sebastienrousseau/vrd:0.0.12
```

## Reproducible builds (intent)

Future goal: same git tag → same SHA256 across runs. We track
this as a constraint on the build environment (pin
toolchain via `rust-toolchain.toml`, pin every dep via
`Cargo.lock`, build with `RUSTFLAGS="-C codegen-units=1 -C
debuginfo=0"`). When `release-binaries.yml` activates, CI will
run each build twice on different runners and compare SHA256;
divergence is a release-blocker.

## What's NOT verifiable today

- No vrd-signed crates.io tarballs (crates.io's own signing
  is the only proof for now).
- No cosign signatures (waiting on `release-binaries.yml`).
- No SBOM attached to releases (waiting on Phase 6;
  `make sbom` generates `SBOM.txt` locally in the meantime).

When Phase 6 lands, this file gets the "(future)" markers
stripped and the activated commands become the truth.

<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# `pkg/PUBLISH.md` - per-channel publish runbook

Step-by-step for each channel in `pkg/`. Read top-to-bottom for
the full publish flow on a fresh release; jump to a section if
you only need to refresh one channel.

## Prerequisites

Before any publish:

1. The crates.io publish has landed (`cargo publish` from
   main on a clean tag). This is the source of truth for every
   downstream channel.
2. `pkg/*/`'s `__VERSION__` / `__SHA256__` / `__COMMIT__`
   placeholders have been rewritten to the actual tag values.
   When `release-binaries.yml` activates, it does this in CI.
   Until then it's manual:

   ```bash
   VERSION=0.0.12
   COMMIT=$(git rev-parse "v${VERSION}")
   SHA=$(curl -sL "https://crates.io/api/v1/crates/vrd/${VERSION}/download" \
         | shasum -a 256 | cut -d' ' -f1)
   sed -i.bak "s/__VERSION__/${VERSION}/g; s/__COMMIT__/${COMMIT}/g; s/__SHA256__/${SHA}/g" \
       pkg/homebrew/vrd.rb pkg/nix/package.nix
   ```

## Homebrew

```bash
# 1. Fork sebastienrousseau/homebrew-tap (one-time).
# 2. Copy the populated formula into the tap:
cp pkg/homebrew/vrd.rb path/to/homebrew-tap/Formula/vrd.rb

# 3. brew audit (catches common formula errors):
brew audit --strict --new-formula vrd

# 4. Test install locally:
brew install --build-from-source ./vrd.rb
vrd --help    # smoke test

# 5. Open the PR against the tap.
```

The future `homebrew-bump` CI job will automate steps 2–5 via
`brew bump-formula-pr`.

## Docker

```bash
# 1. Build both images:
docker build -f pkg/docker/Dockerfile -t ghcr.io/sebastienrousseau/vrd:0.0.12 .
docker build -f pkg/docker/Dockerfile.alpine -t ghcr.io/sebastienrousseau/vrd:0.0.12-alpine .

# 2. Tag the multi-arch manifest (use docker buildx for ARM64):
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    -f pkg/docker/Dockerfile \
    -t ghcr.io/sebastienrousseau/vrd:0.0.12 \
    -t ghcr.io/sebastienrousseau/vrd:latest \
    --push .

# 3. Smoke-test:
docker run --rm ghcr.io/sebastienrousseau/vrd:0.0.12 --help
```

The future `container-publish` CI job will use `docker buildx
build --push` via OIDC to GHCR. No personal tokens required.

## Nix

```bash
# 1. From a clone of the vrd repo:
cd pkg/nix
nix flake update            # refresh deps
nix flake check             # validate
nix build .#vrd             # local smoke build
./result/bin/vrd --help

# 2. To publish, push the updated package.nix to main; the
#    flake URL `github:sebastienrousseau/vrd?ref=v0.0.12` is
#    already addressable by users via `nix run` (no separate
#    nixpkgs PR needed for the flake path).

# 3. nixpkgs PR (optional, for default-channel users):
#    Open against NixOS/nixpkgs with the rewritten
#    pkg/nix/package.nix.
```

`nix run github:sebastienrousseau/vrd` works for any user
without a NixOS install - the flake handles bootstrap.

## Future channels (when `release-binaries.yml` activates)

Per [`../PLAN.md`](../PLAN.md), Phase 6 adds binary builds for
14 platforms with cosign keyless signing + SLSA L3 attestation.
Once that workflow runs, the remaining channels (deb / rpm /
arch / scoop / windows-msi) will activate following the same
template-driven pattern:

1. CI builds the binary per platform.
2. CI generates the SHA256, rewrites `__VERSION__` /
   `__SHA256__` / `__COMMIT__` in the channel's template.
3. CI opens a PR / push to the appropriate downstream repo.
4. Maintainer reviews and merges.

The maintainer-touch step exists to catch any
distro-policy-change regressions; the CI work removes the
copy-paste burden.

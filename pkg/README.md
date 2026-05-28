<!-- SPDX-FileCopyrightText: 2023-2026 vrd contributors -->
<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# `pkg/` - distribution packaging

Per-target packaging artefacts. vrd is library-first; the
`vrd` CLI binary in `src/main.rs` is a demo wrapper and not
the headline deliverable. Phase 6 of [`../PLAN.md`](../PLAN.md)
ships a **subset** of the noyalib packaging matrix - the three
channels with the lowest maintenance overhead and the highest
download share for Rust binaries.

| Directory | Channel | What ships | Where |
|---|---|---|---|
| [`homebrew/`](homebrew/) | macOS / Linuxbrew | `vrd` formula (template) | Manual `brew install` of the formula until the bump job lands |
| [`docker/`](docker/) | GHCR container images | `Dockerfile` (distroless), `Dockerfile.alpine` (smaller) | Manual `docker build` until `release-binaries.yml` activates |
| [`nix/`](nix/) | NixOS / `nix run` | `flake.nix` + `package.nix` | `nix run github:sebastienrousseau/vrd` |
| [`PUBLISH.md`](PUBLISH.md) | - | Per-channel runbook | - |
| [`VERIFY.md`](VERIFY.md) | - | cosign + SLSA verification cookbook | - |

Every template carries `__VERSION__` / `__SHA256__` /
`__COMMIT__` placeholders that will be rewritten by a future
`release-binaries.yml` workflow (tracked in
[`../PLAN.md#phase-6--v010-binary-distribution-target-2027-q1`](../PLAN.md)).
Today the templates are checked in for review; the workflow
activation is the v0.1.0 milestone.

## Why only these three?

| Channel | Coverage | Maintenance | Verdict |
|---|---|---|---|
| Homebrew | macOS + Linuxbrew | Low (formula + tap repo) | ✅ Phase 6 |
| Docker | All Linux servers + CI | Low (Dockerfile + GHCR push) | ✅ Phase 6 |
| Nix | NixOS + nix-the-package-manager users | Low (flake.nix; `nix run` Just Works) | ✅ Phase 6 |
| deb / rpm / arch | Linux distros | High (per-distro maintainer relations) | Deferred to v0.1.0 |
| Scoop / Winget | Windows | Medium (separate bucket repos) | Deferred |
| Snap / Flatpak | Linux universal | High (sandbox manifests + store reviews) | Deferred |
| VS Code | IDE | High (extension API surface) | Out of scope - vrd is a library |
| npm | Node ecosystem | High (wrapper for `npx`-runnability) | Out of scope - vrd has no JS API |

The full 11-channel noyalib matrix is the right pattern for
binary-first crates (noyalib ships `noyafmt` and `noyavalidate`
as flagship deliverables). vrd's CLI is a demo; this subset
matches the actual download story.

## Verification

Future release artefacts will be signed with cosign keyless
and carry SLSA L3 build provenance. See [`VERIFY.md`](VERIFY.md)
for the exact `cosign verify-blob` and `gh attestation verify`
invocations once the binaries publish.

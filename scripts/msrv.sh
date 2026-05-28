#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Verifies vrd builds cleanly on its declared MSRV (read from
# Cargo.toml `rust-version`). Installs the pinned toolchain on
# demand. Used by `make msrv`.

set -euo pipefail

MSRV=$(grep '^rust-version' Cargo.toml | head -1 | tr -d ' "' | cut -d= -f2)

if [[ -z "$MSRV" ]]; then
    echo "error: no rust-version in Cargo.toml" >&2
    exit 2
fi

echo "Verifying MSRV: ${MSRV}"

if ! rustup toolchain list | grep -q "^${MSRV}"; then
    echo "installing ${MSRV} toolchain..."
    rustup toolchain install "${MSRV}" --profile minimal --no-self-update
fi

# Build the library on the pinned MSRV with every feature combo
# that doesn't require nightly. Skip benches (criterion needs
# more recent toolchain features).
cargo "+${MSRV}" check --no-default-features
cargo "+${MSRV}" check --no-default-features --features alloc
cargo "+${MSRV}" check --all-features

echo "MSRV ${MSRV} clean."

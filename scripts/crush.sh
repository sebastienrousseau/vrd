#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Wrapper around `cargo run --release --example crush
# --features crush,pcg,crypto`. Probes for the external
# PractRand `RNG_test` binary and fails loudly with install
# pointers if missing. Used by `make crush`.

set -euo pipefail

if ! command -v RNG_test >/dev/null 2>&1; then
    cat >&2 <<'EOF'
error: PractRand `RNG_test` not found in PATH.

PractRand 0.94+ is required. Build from source:

    git clone https://gitlab.com/pkg.go.dev/practrand.git
    cd practrand
    ./configure && make
    sudo cp RNG_test /usr/local/bin/

Or use the official tarball at
    http://pracrand.sourceforge.net/

Run `make crush` again once `RNG_test --version` works.
EOF
    exit 2
fi

exec cargo run --release --example crush --features crush,pcg,crypto "$@"

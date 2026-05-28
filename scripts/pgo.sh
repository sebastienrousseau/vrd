#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Profile-guided optimisation build. Three-pass dance: instrument
# → profile → optimise. Typical hot-loop win is 5–15%; usually
# enabled only by downstream binary projects, not by the library
# itself. Run from a clean tree.
#
# Outputs an optimised binary at target/release-pgo/vrd plus
# the profile under target/pgo/.

set -euo pipefail

PROFILE_DIR="$(pwd)/target/pgo"
OUTPUT_DIR="$(pwd)/target/release-pgo"

LLVM_PROFDATA="$(rustc --print sysroot)/lib/rustlib/$(rustc -Vv | grep host | cut -d' ' -f2)/bin/llvm-profdata"

if [[ ! -x "$LLVM_PROFDATA" ]]; then
    echo "error: llvm-profdata not found at $LLVM_PROFDATA" >&2
    echo "       install via: rustup component add llvm-tools-preview" >&2
    exit 2
fi

rm -rf "$PROFILE_DIR" "$OUTPUT_DIR"
mkdir -p "$PROFILE_DIR" "$OUTPUT_DIR"

echo "[1/3] instrumented build"
RUSTFLAGS="-Cprofile-generate=$PROFILE_DIR" \
    cargo build --release --target-dir "$OUTPUT_DIR-instr"

echo "[2/3] profiling workload"
"$OUTPUT_DIR-instr/release/vrd" --help >/dev/null 2>&1 || true
# Plug in a representative workload here. For RNGs, exercising
# every public method on Random is a reasonable default:
cargo run --release --example all --target-dir "$OUTPUT_DIR-instr" || true

echo "[3/3] merging profile + optimised rebuild"
"$LLVM_PROFDATA" merge -o "$PROFILE_DIR/merged.profdata" "$PROFILE_DIR"
RUSTFLAGS="-Cprofile-use=$PROFILE_DIR/merged.profdata" \
    cargo build --release --target-dir "$OUTPUT_DIR"

echo "PGO build at: $OUTPUT_DIR/release/"

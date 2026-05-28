#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Prints per-file line coverage from cargo tarpaulin and flags
# every file below the configured threshold. Used by `make
# coverage-gap` and the noyalib-style coverage discipline.
#
# Configure the threshold via VRD_COVERAGE_FLOOR; defaults to 98.

set -euo pipefail

FLOOR="${VRD_COVERAGE_FLOOR:-98}"

command -v cargo-tarpaulin >/dev/null 2>&1 || {
    echo "error: cargo-tarpaulin not installed."
    echo "       cargo install cargo-tarpaulin --locked"
    exit 2
}

# Re-use the project's .tarpaulin.toml excludes; produce the
# raw stdout so we can grep per-file lines.
report=$(cargo tarpaulin --all-features --no-fail-fast --skip-clean 2>&1 | \
         grep -E '^\|\| src/.*: ' || true)

if [[ -z "$report" ]]; then
    echo "no tarpaulin output captured; check that cargo tarpaulin ran."
    exit 2
fi

echo "Per-file coverage (floor = ${FLOOR}%):"
echo
fail=0
while IFS= read -r line; do
    # Lines look like:  || src/foo.rs: 22/22 +0.00%
    file=$(echo "$line" | awk '{print $2}' | tr -d ':')
    frac=$(echo "$line" | awk '{print $3}')
    num=${frac%/*}
    den=${frac#*/}
    [[ "$den" == "0" ]] && continue
    pct=$(( num * 100 / den ))
    if (( pct < FLOOR )); then
        printf "  \033[31m%6d%%\033[0m  %s  (%s)\n" "$pct" "$file" "$frac"
        fail=$((fail + 1))
    else
        printf "  \033[32m%6d%%\033[0m  %s  (%s)\n" "$pct" "$file" "$frac"
    fi
done <<< "$report"

echo
if (( fail > 0 )); then
    echo "$fail file(s) below the ${FLOOR}% floor."
    exit 1
fi
echo "All files at or above the ${FLOOR}% floor."

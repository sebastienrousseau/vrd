# POSIX-compatible Makefile for vrd.
# Works on macOS, Linux, and WSL without modification.
#
# Usage:
#   make            — run check + clippy + test (default)
#   make test       — run all tests under --all-features
#   make clippy     — run clippy lints
#   make fmt        — check formatting (cargo fmt --check)
#   make deny       — run cargo-deny supply-chain checks
#   make doc        — build documentation
#   make bench      — run criterion benchmarks
#   make coverage   — generate tarpaulin report (HTML + Lcov)
#   make coverage-gap — list files below the workspace threshold
#   make notice     — generate NOTICE via cargo-about
#   make sbom       — generate software bill of materials
#   make vendor     — `cargo vendor` for offline / FIPS builds
#   make msrv       — verify MSRV per declared rust-version
#   make pgo        — profile-guided optimisation build
#   make crush      — pipe RNGs through PractRand (needs RNG_test)
#   make fuzz-quick — 60 s per fuzz target (cargo-fuzz)
#   make fuzz-deep  — 24 h per fuzz target
#   make examples   — run every example with [ok]/[fail] status
#   make wasm       — cargo check for wasm32-unknown-unknown
#   make embedded   — cargo check for thumbv7em-none-eabihf
#   make clean      — remove build artifacts

.PHONY: all check clippy test fmt deny doc bench coverage coverage-gap \
        notice sbom vendor msrv pgo crush \
        fuzz-quick fuzz-deep examples wasm embedded clean

all: check clippy test

check:
	cargo check --all-features --all-targets

clippy:
	cargo clippy --all-features --all-targets -- -D warnings

test:
	cargo test --all-features

fmt:
	cargo fmt --check

deny:
	cargo deny check

doc:
	cargo doc --no-deps --all-features

bench:
	cargo bench --features simd,pcg,quasirandom

coverage:
	cargo tarpaulin --all-features --no-fail-fast --skip-clean \
	    --out Html --out Lcov --output-dir target/tarpaulin

coverage-gap:
	./scripts/coverage-gap-report.sh

# `cargo-about generate` — produces NOTICE listing every third-
# party crate vrd redistributes plus its license text. Auto-
# installs cargo-about on demand.
notice:
	@cargo about --version >/dev/null 2>&1 || cargo install cargo-about --locked
	cargo about generate -c about.toml -o NOTICE about.hbs
	@echo "NOTICE written; ship it inside every release tarball."

sbom:
	cargo tree --edges normal --prefix depth --format '{p} {l}' > SBOM.txt
	@echo "SBOM written to SBOM.txt"

# `cargo vendor` for offline / air-gapped / FIPS-bound builds.
vendor:
	cargo vendor --versioned-dirs vendor
	@echo "Vendored to vendor/. Configure via:"
	@echo "  [source.crates-io]"
	@echo "  replace-with = \"vendored\""
	@echo "  [source.vendored]"
	@echo "  directory = \"vendor\""
	@echo "in .cargo/config.toml, then build with \`cargo build --offline\`."

msrv:
	./scripts/msrv.sh

pgo:
	./scripts/pgo.sh

crush:
	./scripts/crush.sh

# Quick fuzz pass — 60 s per target. Catches most regressions;
# use fuzz-deep for proper coverage. Requires cargo-fuzz:
# `cargo install cargo-fuzz`. cargo-fuzz needs nightly.
fuzz-quick:
	@cargo fuzz --version >/dev/null 2>&1 || cargo install cargo-fuzz --locked
	@for t in fuzz_xoshiro_seed fuzz_mt_seed fuzz_pcg_seed \
	          fuzz_fill_array fuzz_uniform_range fuzz_quasirandom ; do \
	    echo "» $$t (60 s)" ; \
	    cargo +nightly fuzz run $$t -- -max_total_time=60 || exit 1 ; \
	done
	@echo "fuzz-quick clean."

fuzz-deep:
	@cargo fuzz --version >/dev/null 2>&1 || cargo install cargo-fuzz --locked
	@for t in fuzz_xoshiro_seed fuzz_mt_seed fuzz_pcg_seed \
	          fuzz_fill_array fuzz_uniform_range fuzz_quasirandom ; do \
	    echo "» $$t (24 h)" ; \
	    cargo +nightly fuzz run $$t -- -max_total_time=86400 || exit 1 ; \
	done
	@echo "fuzz-deep clean."

# Runs every example (including feature-gated ones) and reports
# [ok]/[fail] per line. Mirrors the noyalib examples loop.
examples:
	@for ex in hello basics seed split bytes floats bools chars strings \
	           bounded unbiased \
	           normal exponential poisson distribution \
	           choose shuffle sample slice weighted \
	           xoshiro mersenne backends \
	           traits seedable macros \
	           dice lottery passwords monte \
	           errors config nostd \
	           iterators uuid tokens ; do \
	    printf "  \033[90m%-14s\033[0m " "$$ex" ; \
	    if cargo run --example $$ex --quiet 2>/dev/null 1>/dev/null ; then \
	        printf "\033[32m[ok]\033[0m\n" ; \
	    else \
	        printf "\033[31m[fail]\033[0m\n" ; \
	        exit 1 ; \
	    fi ; \
	done
	@for pair in "pcg:pcg" "secure:crypto" "halton:quasirandom" "sobol:quasirandom" ; do \
	    ex=$${pair%%:*} ; feat=$${pair##*:} ; \
	    printf "  \033[90m%-14s\033[0m " "$$ex" ; \
	    if cargo run --example $$ex --features $$feat --quiet 2>/dev/null 1>/dev/null ; then \
	        printf "\033[32m[ok]\033[0m \033[90m(--features %s)\033[0m\n" "$$feat" ; \
	    else \
	        printf "\033[31m[fail]\033[0m \033[90m(--features %s)\033[0m\n" "$$feat" ; \
	        exit 1 ; \
	    fi ; \
	done
	@printf "\n\033[1;32mAll examples passed.\033[0m\n"

wasm:
	cargo check --target wasm32-unknown-unknown --no-default-features
	cargo check --target wasm32-unknown-unknown --no-default-features --features alloc

embedded:
	cargo check --target thumbv7em-none-eabihf --no-default-features
	cargo check --target thumbv7em-none-eabihf --no-default-features --features alloc

clean:
	cargo clean
	rm -rf vendor SBOM.txt NOTICE target/tarpaulin

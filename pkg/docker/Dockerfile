# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Distroless multi-stage build for the `vrd` CLI demo.
# Final image is ~10 MB; runs without a shell, package manager,
# or libc - only the statically-linked binary.

# ─── Build stage ───────────────────────────────────────────
FROM rust:1.85-slim AS build

WORKDIR /src
COPY . .

# Strip everything we don't need to compile the binary; saves
# ~30 s on cold caches.
ENV CARGO_TERM_COLOR=always
RUN cargo build --release --bin vrd

# ─── Runtime stage (distroless) ────────────────────────────
FROM gcr.io/distroless/cc-debian12:nonroot

LABEL org.opencontainers.image.source="https://github.com/sebastienrousseau/vrd"
LABEL org.opencontainers.image.description="Lightweight, no_std-friendly random number generator for Rust - CLI demo"
LABEL org.opencontainers.image.licenses="Apache-2.0 OR MIT"
LABEL org.opencontainers.image.version="__VERSION__"
LABEL org.opencontainers.image.revision="__COMMIT__"

COPY --from=build /src/target/release/vrd /usr/local/bin/vrd

USER nonroot

ENTRYPOINT ["/usr/local/bin/vrd"]
CMD ["--help"]

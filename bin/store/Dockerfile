# syntax=docker/dockerfile:1.4
# Use a newer Rust image that supports Cargo.lock v4
FROM rust:bookworm AS chef
WORKDIR /usr/src/crdt-workspace
RUN cargo install cargo-chef --locked && cargo install sccache --locked
# Copy the entire workspace so cargo metadata can infer implicit targets
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:bookworm AS builder

COPY --from=chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=chef /usr/local/cargo/bin/sccache /usr/local/cargo/bin/sccache

# Install build dependencies
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev protobuf-compiler pkg-config make clang lld binutils ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set workspace
WORKDIR /usr/src/crdt-workspace

ARG BUILD_JOBS=4
ARG OPT_LEVEL=3
ARG CODEGEN_UNITS=16
ARG BUILD_SWAP_MB=0
ENV CARGO_BUILD_JOBS=${BUILD_JOBS}
ENV CARGO_INCREMENTAL=0
ENV RUSTFLAGS="-C target-cpu=generic -C codegen-units=${CODEGEN_UNITS} -C opt-level=${OPT_LEVEL} -C debuginfo=0 -C embed-bitcode=no -C linker=clang -C link-arg=-fuse-ld=lld -C link-arg=-Wl,--no-keep-memory"
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV SCCACHE_DIR=/root/.cache/sccache
ENV PATH="/usr/local/cargo/bin:/root/.cargo/bin:${PATH}"

# Build dependency layer using cargo-chef for optimal caching
COPY --from=chef /usr/src/crdt-workspace/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,sharing=locked,target=/root/.cache/sccache \
    cargo chef cook --release --recipe-path recipe.json -j "${CARGO_BUILD_JOBS:-1}"

# Copy the full workspace
COPY . .

RUN if [ "${BUILD_SWAP_MB}" != "0" ]; then \
      (fallocate -l ${BUILD_SWAP_MB}M /swapfile 2>/dev/null || dd if=/dev/zero of=/swapfile bs=1M count=${BUILD_SWAP_MB}); \
      chmod 600 /swapfile; \
      mkswap /swapfile; \
      swapon /swapfile || true; \
    fi

# Build the store and place the resulting binary at workspace root
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,sharing=locked,target=/root/.cache/sccache \
    --mount=type=cache,target=/usr/src/crdt-workspace/target \
    set -e; \
    cd bin/store; \
    cargo build --release -j "${CARGO_BUILD_JOBS:-1}"; \
    cp -f /usr/src/crdt-workspace/target/release/store /usr/src/crdt-workspace/store; \
    strip /usr/src/crdt-workspace/store || true

# Final runtime image — slim OS with only runtime deps
FROM debian:bookworm-slim

# Install runtime dependencies (libpq for Postgres), TZ and certs
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && \
    apt-get install -y libpq5 ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary and make it executable
COPY --from=builder /usr/src/crdt-workspace/store /store
COPY --from=builder /usr/src/crdt-workspace/bin/store /bin/store
RUN chmod +x /store

WORKDIR /

# Default environment
ENV RUST_LOG=info TZ=UTC

# Start the store directly
CMD ["/store"]

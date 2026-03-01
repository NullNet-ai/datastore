# syntax=docker/dockerfile:1.4
# Use a newer Rust image that supports Cargo.lock v4
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y libpq-dev protobuf-compiler pkg-config make file clang lld && \
    rm -rf /var/lib/apt/lists/*

# Set workspace
WORKDIR /usr/src/crdt-workspace

# Copy workspace
COPY . .

ARG BUILD_JOBS=1
ARG OPT_LEVEL=2
ARG CODEGEN_UNITS=1
ARG BUILD_SWAP_MB=0

ENV CARGO_BUILD_JOBS=${BUILD_JOBS}
ENV CARGO_INCREMENTAL=0
ENV RUSTFLAGS="-C target-cpu=generic -C codegen-units=${CODEGEN_UNITS} -C opt-level=${OPT_LEVEL} -C debuginfo=0 -C embed-bitcode=no -C linker=clang -C link-arg=-fuse-ld=lld -C link-arg=-Wl,--no-keep-memory"

RUN if [ "${BUILD_SWAP_MB}" != "0" ]; then \
      (fallocate -l ${BUILD_SWAP_MB}M /swapfile 2>/dev/null || dd if=/dev/zero of=/swapfile bs=1M count=${BUILD_SWAP_MB}); \
      chmod 600 /swapfile; \
      mkswap /swapfile; \
      swapon /swapfile || true; \
    fi

# If a prebuilt ./store binary exists at the workspace root, use it.
# Otherwise, build the store and place the resulting binary at workspace root.
RUN --mount=type=secret,id=dnamicro_token,required=false sh -lc '\
    set -eux; \
    export PATH="/usr/local/cargo/bin:/root/.cargo/bin:$PATH"; \
    if [ -f "./store" ]; then \
        if file ./store | grep -qi "ELF"; then \
            echo "Using prebuilt Linux ELF store binary at workspace root"; \
            chmod +x ./store; \
        else \
            echo "Prebuilt binary is not Linux ELF; rebuilding for Linux"; \
            cd bin/store && cargo build --release -j "$CARGO_BUILD_JOBS"; \
            cp -f /usr/src/crdt-workspace/target/release/store /usr/src/crdt-workspace/store; \
        fi; \
    else \
        echo "No prebuilt binary found; building with cargo"; \
        cd bin/store && cargo build --release -j "$CARGO_BUILD_JOBS"; \
        cp -f /usr/src/crdt-workspace/target/release/store /usr/src/crdt-workspace/store; \
    fi'

# Final runtime image — slim OS with only runtime deps
FROM debian:bookworm-slim

# Install runtime dependencies (libpq for Postgres), TZ and certs
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/crdt-workspace/store /usr/local/bin/store
COPY --from=builder /usr/src/crdt-workspace/bin/store /app/bin/store
WORKDIR /app

# Default environment
ENV RUST_LOG=info TZ=UTC

# Start the store directly
CMD ["store"]

FROM scratch AS artifact
COPY --from=builder /usr/src/crdt-workspace/store /store
CMD ["/store"]

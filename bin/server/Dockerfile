# syntax=docker/dockerfile:1.4
FROM rust:bookworm AS chef
WORKDIR /usr/src/crdt-workspace
RUN cargo install cargo-chef --locked && cargo install sccache --locked
COPY . .
RUN cd bin/server && cargo chef prepare --recipe-path recipe.json

FROM rust:bookworm AS builder

COPY --from=chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=chef /usr/local/cargo/bin/sccache /usr/local/cargo/bin/sccache

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev pkg-config make clang lld binutils ca-certificates && \
    rm -rf /var/lib/apt/lists/*

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

COPY --from=chef /usr/src/crdt-workspace/bin/server/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,sharing=locked,target=/root/.cache/sccache \
    cargo chef cook --release --recipe-path recipe.json -j "${CARGO_BUILD_JOBS:-1}"

COPY . .

RUN if [ "${BUILD_SWAP_MB}" != "0" ]; then \
      (fallocate -l ${BUILD_SWAP_MB}M /swapfile 2>/dev/null || dd if=/dev/zero of=/swapfile bs=1M count=${BUILD_SWAP_MB}); \
      chmod 600 /swapfile; \
      mkswap /swapfile; \
      swapon /swapfile || true; \
    fi

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,sharing=locked,target=/root/.cache/sccache \
    --mount=type=cache,target=/usr/src/crdt-workspace/target \
    set -e; \
    cd bin/server; \
    cargo build --release -j "${CARGO_BUILD_JOBS:-1}"; \
    cp -f /usr/src/crdt-workspace/target/release/server /usr/src/crdt-workspace/server; \
    strip /usr/src/crdt-workspace/server || true

FROM debian:bookworm-slim

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && \
    apt-get install -y libpq5 ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/crdt-workspace/server /server
COPY --from=builder /usr/src/crdt-workspace/bin/server /bin/server
RUN chmod +x /server

WORKDIR /

ENV RUST_LOG=info TZ=UTC

CMD ["/server"]

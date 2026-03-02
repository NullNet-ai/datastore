# Use a newer Rust image that supports Cargo.lock v4
FROM rust:latest as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Set workspace
# Install build dependencies
RUN apt-get update && \
# Copy workspace


# Build server
COPY --from=builder /usr/src/crdt-workspace/target/release/server /usr/local/bin/server

# Set the startup command
# Final image
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Set the startup command

# Use a Rust base image
FROM rust:1.70-slim-buster as builder

# Create a new empty shell project
WORKDIR /usr/src/server

# Copy over your manifests and source code
COPY . .

# Build your application
RUN cargo build --release

# Start with a fresh image
FROM debian:buster-slim

# Copy the build artifact from builder
COPY --from=builder /usr/src/server/target/release/server /usr/local/bin/server

# Set the startup command
CMD ["server"]
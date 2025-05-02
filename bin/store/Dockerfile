# Use a Rust base image
FROM rust:1.70-slim-buster as builder

# Create a new empty shell project
WORKDIR /usr/src/store

# Copy over your manifests and source code
COPY . .

# Build your application
RUN cargo build --release

# Start with a fresh image
FROM debian:buster-slim

# Copy the build artifact from builder
COPY --from=builder /usr/src/store/target/release/store /usr/local/bin/store

# Set the startup command
CMD ["store"]
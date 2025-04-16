# Makefile for CRDT workspace

.PHONY: all server store dev clean

# Default target
all: dev

# Run both server and store in parallel
dev:
	@echo "Starting server and store..."
	@make -j 2 server store

# Run the server
server:
	@echo "Starting server..."
	@cd bin/server && cargo run

# Run the store
store:
	@echo "Starting store..."
	@cd bin/store && cargo run

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cd bin/server && cargo clean
	@cd bin/store && cargo clean
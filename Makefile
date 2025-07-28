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

# Run the store clean setup
store-clean-setup:
	@echo "Starting store clean setup..."
	@cd bin/store && cargo make clean-setup

# Run the store in watch mode
store-watch:
	@echo "Starting store in watch mode..."
	@cd bin/store && DEBUG=true cargo watch -x run

# Build the store
store-build:
	@echo "Building store..."
	@cd bin/store && cargo build --release

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cd bin/server && cargo clean
	@cd bin/store && cargo clean

# Diesel Migration with name parameter
db-migrate-generate:
	@if [ -z "$(NAME)" ]; then \
		echo "Usage: make db-migrate-generate NAME=migration_name"; \
		exit 1; \
	fi
	@echo "Generating Diesel migration: $(NAME)..."
	@cd bin/store && diesel migration generate $(NAME)
	
# Run migrations
db-migrate-run:
	@echo "Running database migrations..."
	@cd bin/store && diesel migration run

# Revert last migration
db-migrate-revert:
	@echo "Reverting last migration..."
	@cd bin/store && diesel migration revert
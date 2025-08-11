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
	@cd bin/store && DEBUG=true RUST_BACKTRACE=full cargo watch -x run

# Create store schema
store-generate-migrate:
	@echo "Generating and migrating store schema..."
	@cd bin/store && CREATE_SCHEMA=true cargo run

# Generate store proto
store-generate-proto:
	@echo "Generating store proto..."
	@cd bin/store && GENERATE_PROTO=true GENERATE_GRPC=true GENERATE_TABLE_ENUM=true cargo run

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
db-migrate-up:
	@echo "Running database migrations..."
	@cd bin/store && diesel migration run

db-migrate-down:
	@echo "Reverting database migrations..."
	@cd bin/store && diesel migration revert

# Revert last migration
db-migrate-revert:
	@echo "Reverting last migration..."
	@cd bin/store && diesel migration revert 

# Format Rust code
fmt:
	@echo "Formatting Rust code..."
	@cd bin/store && cargo fmt --all
	@cd bin/server && cargo fmt --all
	@cd libs/hlc && cargo fmt --all
	@cd libs/merkle && cargo fmt --all
	@if [ -d "mcp-proto-generator" ]; then cd mcp-proto-generator && cargo fmt --all; fi
	@echo "✅ Code formatting complete! (Generated files in src/generated/ excluded)"

# Check code formatting
fmt-check:
	@echo "Checking code formatting..."
	@cd bin/store && cargo fmt --all --check
	@cd bin/server && cargo fmt --all --check
	@cd libs/hlc && cargo fmt --all --check
	@cd libs/merkle && cargo fmt --all --check
	@if [ -d "mcp-proto-generator" ]; then cd mcp-proto-generator && cargo fmt --all --check; fi

# Setup git hooks
setup-hooks:
	@./scripts/setup-hooks.sh
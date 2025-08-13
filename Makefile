# Makefile for CRDT workspace

# PHONY targets (targets that don't create files)
.PHONY: all dev clean help \
        server store store-clean-setup store-watch store-build store-generate-migrate store-generate-proto \
        db-migrate-generate db-migrate-up db-migrate-down db-migrate-revert \
        fmt fmt-check setup-hooks

# Default target
all: dev

# Help target
help:
	@echo "Available targets:"
	@echo "  dev                    - Run both server and store in parallel"
	@echo "  server                 - Run the server"
	@echo "  store                  - Run the store"
	@echo "  store-clean-setup      - Run store clean setup"
	@echo "  store-watch            - Run store in watch mode with debug"
	@echo "  store-build            - Build store in release mode"
	@echo "  store-generate-migrate - Generate and migrate store schema"
	@echo "  store-generate-proto   - Generate store proto files"
	@echo "  db-migrate-generate    - Generate new migration (requires NAME=name)"
	@echo "  db-migrate-up          - Run database migrations"
	@echo "  db-migrate-down        - Revert database migrations"
	@echo "  db-migrate-revert      - Revert last migration"
	@echo "  fmt                    - Format Rust code"
	@echo "  fmt-check              - Check code formatting"
	@echo "  setup-hooks            - Setup git hooks"
	@echo "  clean                  - Clean build artifacts"
	@echo "  help                   - Show this help message"

# =============================================================================
# Development targets
# =============================================================================

# Run both server and store in parallel
dev:
	@echo "🚀 Starting server and store..."
	@make -j 2 server store

# Run the server
server:
	@echo "🖥️  Starting server..."
	@cd bin/server && cargo run

# Run the store
store:
	@echo "🗄️  Starting store..."
	@cd bin/store && cargo run

# =============================================================================
# Store-specific targets
# =============================================================================

# Run the store clean setup
store-clean-setup:
	@echo "🧹 Starting store clean setup..."
	@cd bin/store && cargo make clean-setup

# Run the store in watch mode
store-watch:
	@echo "👀 Starting store in watch mode..."
	@cd bin/store && DEBUG=true RUST_BACKTRACE=full cargo watch -x run

# Build the store
store-build:
	@echo "🔨 Building store..."
	@cd bin/store && cargo build --release

# Create store schema
store-generate-migrate:
	@echo "📋 Generating and migrating store schema..."
	@cd bin/store && CREATE_SCHEMA=true cargo run

# Generate store proto
store-generate-proto:
	@echo "🔧 Generating store proto..."
	@cd bin/store && GENERATE_PROTO=true GENERATE_GRPC=true GENERATE_TABLE_ENUM=true cargo run

# =============================================================================
# Database migration targets
# =============================================================================

# Generate new Diesel migration with name parameter
db-migrate-generate:
	@if [ -z "$(NAME)" ]; then \
		echo "❌ Usage: make db-migrate-generate NAME=migration_name"; \
		exit 1; \
	fi
	@echo "📝 Generating Diesel migration: $(NAME)..."
	@cd bin/store && diesel migration generate $(NAME)

# Run migrations
db-migrate-up:
	@echo "⬆️  Running database migrations..."
	@cd bin/store && diesel migration run

# Revert database migrations
db-migrate-down:
	@echo "⬇️  Reverting database migrations..."
	@cd bin/store && diesel migration revert

# Revert last migration
db-migrate-revert:
	@echo "↩️  Reverting last migration..."
	@cd bin/store && diesel migration revert

# =============================================================================
# Code formatting and quality targets
# =============================================================================

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
	@echo "🪝 Setting up git hooks..."
	@./scripts/setup-hooks.sh

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cd bin/server && cargo clean
	@cd bin/store && cargo clean
	@echo "✅ Clean complete!"


# Run the store in watch mode with PG library configurations
jean-store-watch:
	@echo "Starting store in watch mode..."
	@cd bin/store && PQ_LIB_DIR=/opt/homebrew/opt/postgresql@14/lib/postgresql@14 LIBRARY_PATH=/opt/homebrew/opt/postgresql@14/lib/postgresql@14 cargo watch -x run
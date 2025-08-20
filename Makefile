# Makefile for CRDT workspace

# PHONY targets (targets that don't create files)
.PHONY: all dev clean help install verify-install install-macos install-linux \
        server store store-clean-setup store-watch store-build store-prod \
        store-generate-schema store-generate-proto \
        db-migrate-generate db-migrate-up db-migrate-revert \
        fmt fmt-check git-cleanup setup-hooks \
        jean-store-watch store-experimental store-initialize-device \
        pm2-start pm2-stop pm2-restart pm2-status pm2-logs pm2-delete

# Default target
all: dev

# Help target
help:
	@echo "Available targets:"
	@echo "  install                 - Install all dependencies and setup the project"
	@echo "  verify-install          - Verify that all required tools are installed"
	@echo "  dev                     - Run both server and store in parallel"
	@echo "  server                  - Run the server"
	@echo "  store                   - Run the store"
	@echo "  store-clean-setup       - Run store clean setup"
	@echo "  store-watch             - Run store in watch mode with debug"
	@echo "  store-build             - Build store in release mode"
	@echo "  store-prod              - Run store in production mode"
	@echo "  store-initialize-device - Initialize device and wait for PostgreSQL listener"
	@echo "  store-generate-schema   - Generate store schema"
	@echo "  store-generate-proto    - Generate store proto files"
	@echo "  db-migrate-generate     - Generate new migration (requires NAME=name)"
	@echo "  db-migrate-up           - Run database migrations"
	@echo "  db-migrate-revert       - Revert last migration"
	@echo "  fmt                     - Format Rust code"
	@echo "  fmt-check               - Check code formatting"
	@echo "  git-cleanup             - Clean up local branches that no longer exist on remote"
	@echo "  setup-hooks             - Setup git hooks"
	@echo "  clean                   - Clean build artifacts"
	@echo "  pm2-start               - Start all services with PM2"
	@echo "  pm2-stop                - Stop all PM2 services"
	@echo "  pm2-restart             - Restart all PM2 services"
	@echo "  pm2-status              - Show PM2 process status"
	@echo "  pm2-logs                - Show PM2 logs"
	@echo "  pm2-delete              - Delete all PM2 processes"
	@echo "  help                    - Show this help message"

# =============================================================================
# Installation and Setup targets
# =============================================================================

# One-command installer for seamless project setup
install:
	@echo "🚀 Setting up CRDT Workspace - One-command installer"
	@echo "📋 Detecting operating system..."
	@if [ "$$(uname)" = "Darwin" ]; then \
		echo "🍎 macOS detected"; \
		make install-macos; \
	elif [ "$$(uname)" = "Linux" ]; then \
		echo "🐧 Linux detected"; \
		make install-linux; \
	else \
		echo "❌ Unsupported operating system: $$(uname)"; \
		echo "Please install dependencies manually:"; \
		echo "  - Rust (https://rustup.rs/)"; \
		echo "  - PostgreSQL"; \
		echo "  - cargo-make, cargo-watch, diesel_cli"; \
		exit 1; \
	fi
	@# Seeding database
	@echo "🌱 Seeding Store database..."
	@make store-clean-setup
	@echo "✅ Store database seeded!"
	@# Setup git hooks
	@make setup-hooks
	@echo "✅ Installation complete! Run 'make store' to start the project."
	

# macOS installation
install-macos:
	@echo "🔧 Installing dependencies for macOS..."
	@# Check if Homebrew is installed
	@if ! command -v brew >/dev/null 2>&1; then \
		echo "📦 Installing Homebrew..."; \
		/bin/bash -c "$$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"; \
		echo "🔄 Setting up Homebrew environment..."; \
		eval "$$(/opt/homebrew/bin/brew shellenv)"; \
		export PATH="/opt/homebrew/bin:$$PATH"; \
	else \
		echo "✅ Homebrew already installed"; \
	fi
	@# Ensure Homebrew is in PATH for subsequent commands
	@export PATH="/opt/homebrew/bin:$$PATH"; \
	if ! command -v psql >/dev/null 2>&1; then \
		echo "🐘 Installing PostgreSQL..."; \
		brew install postgresql@14; \
		brew services start postgresql@14; \
	else \
		echo "✅ PostgreSQL already installed"; \
	fi
	@# Install Rust if not present
	@if ! command -v rustc >/dev/null 2>&1; then \
		echo "🦀 Installing Rust..."; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		source ~/.cargo/env; \
	else \
		echo "✅ Rust already installed"; \
	fi
	@# Install Rust tools
	@echo "🔨 Installing Rust development tools..."
	@source ~/.cargo/env && cargo install cargo-make cargo-watch || true
	@source ~/.cargo/env && cargo install diesel_cli --no-default-features --features postgres || true
	@# Setup git hooks
	@make setup-hooks
	@echo "🎉 macOS setup complete!"

# Linux installation
install-linux:
	@echo "🔧 Installing dependencies for Linux..."
	@# Detect package manager and install PostgreSQL
	@if command -v apt-get >/dev/null 2>&1; then \
		echo "📦 Using apt-get (Debian/Ubuntu)"; \
		sudo apt-get update; \
		sudo apt-get install -y curl build-essential libssl-dev pkg-config libpq-dev postgresql postgresql-contrib; \
		sudo systemctl start postgresql; \
		sudo systemctl enable postgresql; \
	elif command -v yum >/dev/null 2>&1; then \
		echo "📦 Using yum (RHEL/CentOS)"; \
		sudo yum update -y; \
		sudo yum install -y curl gcc openssl-devel pkgconfig postgresql-devel postgresql-server postgresql-contrib; \
		sudo postgresql-setup initdb; \
		sudo systemctl start postgresql; \
		sudo systemctl enable postgresql; \
	elif command -v pacman >/dev/null 2>&1; then \
		echo "📦 Using pacman (Arch Linux)"; \
		sudo pacman -Syu --noconfirm curl base-devel openssl pkgconf postgresql; \
		sudo systemctl start postgresql; \
		sudo systemctl enable postgresql; \
	else \
		echo "❌ Unsupported package manager. Please install PostgreSQL manually."; \
		exit 1; \
	fi
	@# Install Rust if not present
	@if ! command -v rustc >/dev/null 2>&1; then \
		echo "🦀 Installing Rust..."; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		source ~/.cargo/env; \
	else \
		echo "✅ Rust already installed"; \
	fi
	@# Install Rust tools
	@echo "🔨 Installing Rust development tools..."
	@source ~/.cargo/env && cargo install cargo-make cargo-watch || true
	@source ~/.cargo/env && cargo install diesel_cli --no-default-features --features postgres || true
	@# Setup git hooks
	@make setup-hooks
	@echo "🎉 Linux setup complete!"

# Verify installation
verify-install:
	@echo "🔍 Verifying installation..."
	@echo "Checking Rust..."
	@rustc --version || (echo "❌ Rust not found" && exit 1)
	@echo "Checking Cargo..."
	@cargo --version || (echo "❌ Cargo not found" && exit 1)
	@echo "Checking cargo-make..."
	@cargo make --version || (echo "❌ cargo-make not found" && exit 1)
	@echo "Checking cargo-watch..."
	@cargo watch --version || (echo "❌ cargo-watch not found" && exit 1)
	@echo "Checking diesel CLI..."
	@diesel --version || (echo "❌ diesel CLI not found" && exit 1)
	@echo "Checking PostgreSQL..."
	@export PATH="/opt/homebrew/bin:$$PATH"; \
	if command -v psql >/dev/null 2>&1; then \
		psql --version; \
	elif [ -f "/opt/homebrew/bin/psql" ]; then \
		/opt/homebrew/bin/psql --version; \
		echo "⚠️  PostgreSQL found but not in PATH. Add this to your shell profile:"; \
		echo "   export PATH=\"/opt/homebrew/bin:\$$PATH\""; \
	else \
		echo "❌ PostgreSQL not found" && exit 1; \
	fi
	@echo "✅ All required tools are installed and working!"

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

# Run the store in production mode
store-prod:
	@echo "🚀 Starting store in production mode..."
	@cd bin/store && cargo run --release

# Create store schema
store-generate-schema:
	@echo "📋 Generating store schema..."
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

# Clean up git branches
git-cleanup:
	@echo "Fetching latest changes and pruning deleted remote branches..."
	@git fetch --prune
	@echo "Deleting local branches that no longer exist on remote..."
	@git branch -vv | grep ': gone]' | awk '{print $$1}' | xargs -r git branch -D || true
	@echo "✅ Git cleanup complete!"

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

# =============================================================================
# PM2 Process Management targets
# =============================================================================

# Start all services with PM2
pm2-start:
	@echo "🚀 Starting CRDT workspace with PM2..."
	@pm2 start ecosystem.config.js
	@echo "✅ Services started! Use 'make pm2-status' to check status."

# Stop all PM2 services
pm2-stop:
	@echo "⏹️  Stopping PM2 services..."
	@pm2 stop ecosystem.config.js
	@echo "✅ Services stopped!"

# Restart all PM2 services
pm2-restart:
	@echo "🔄 Restarting PM2 services..."
	@pm2 restart ecosystem.config.js
	@echo "✅ Services restarted!"

# Show PM2 process status
pm2-status:
	@echo "📊 PM2 Process Status:"
	@pm2 status

# Show PM2 logs
pm2-logs:
	@echo "📋 PM2 Logs (press Ctrl+C to exit):"
	@pm2 logs

# Delete all PM2 processes
pm2-delete:
	@echo "🗑️  Deleting PM2 processes..."
	@pm2 delete ecosystem.config.js
	@echo "✅ PM2 processes deleted!"


# Run the store in watch mode with PG library configurations
jean-store-watch:
	@echo "Starting store in watch mode..."
	@cd bin/store && PQ_LIB_DIR=/opt/homebrew/opt/postgresql@14/lib/postgresql@14 LIBRARY_PATH=/opt/homebrew/opt/postgresql@14/lib/postgresql@14 cargo watch -x run

# Run experimental features
store-experimental:
	@echo "Running experimental features..."
	@cd bin/store && EXPERIMENTAL_PERMISSIONS=true cargo run

# Run store initialize device
store-initialize-device:
	@echo "🔧 Initializing device..."
	@cd bin/store && { \
		INITIALIZE_DEVICE=true cargo run > /tmp/store_init.log 2>&1 & \
		PID=$$!; \
		echo "⏳ Waiting for PostgreSQL listener to start..."; \
		for i in $$(seq 1 60); do \
			if grep -q "Started listening on PostgreSQL channels" /tmp/store_init.log 2>/dev/null; then \
				echo "✅ PostgreSQL listener started!"; \
				break; \
			fi; \
			sleep 1; \
		done; \
		kill $$PID 2>/dev/null || true; \
		wait $$PID 2>/dev/null || true; \
		rm -f /tmp/store_init.log; \
	}
	@echo "✅ Device initialization completed! Waiting 1 second before exit..."
	@sleep 1
	@echo "🏁 Exiting store-initialize-device"

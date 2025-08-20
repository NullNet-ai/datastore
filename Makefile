# Makefile for CRDT workspace

# PHONY targets (targets that don't create files)
.PHONY: all dev clean help install verify-install install-macos install-linux \
        server store store-clean-setup store-watch store-build store-prod \
        store-generate-schema store-generate-proto \
        db-migrate-generate db-migrate-up db-migrate-revert \
        fmt fmt-check git-cleanup setup-hooks \
        jean-store-watch store-experimental store-initialize-device \
        pm2-start pm2-stop pm2-restart pm2-status pm2-logs pm2-delete \
        docker-build-ubuntu docker-build-centos docker-build-arch docker-build-all \
        docker-run-ubuntu docker-run-centos docker-run-arch docker-run-all

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
	@echo "  docker-build-ubuntu     - Build Docker image for Ubuntu test"
	@echo "  docker-build-centos     - Build Docker image for CentOS test"
	@echo "  docker-build-arch       - Build Docker image for Arch Linux test"
	@echo "  docker-build-all        - Build Docker images for all distributions"
	@echo "  docker-run-ubuntu       - Run Docker container for Ubuntu testing"
	@echo "  docker-run-centos       - Run Docker container for CentOS testing"
	@echo "  docker-run-arch         - Run Docker container for Arch Linux testing"
	@echo "  docker-run-all          - Run Docker containers for all operating systems"
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
	@export PATH="$$HOME/.cargo/bin:$$PATH" && make store-clean-setup
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
	else \
		echo "✅ Rust already installed"; \
	fi
	@# Install Rust tools
	@echo "🔨 Installing Rust development tools..."
	@export PATH="$$HOME/.cargo/bin:$$PATH" && cargo install cargo-make cargo-watch
	@export PATH="$$HOME/.cargo/bin:$$PATH" && cargo install diesel_cli --no-default-features --features postgres
	@echo "🎉 macOS setup complete!"

# Linux installation
install-linux:
	@echo "🔧 Installing dependencies for Linux..."
	@# Detect package manager and install PostgreSQL
	@if command -v apt-get >/dev/null 2>&1; then \
		echo "📦 Using apt-get (Debian/Ubuntu)"; \
		sudo apt-get update; \
		sudo apt-get install -y curl build-essential libssl-dev pkg-config libpq-dev postgresql postgresql-contrib protobuf-compiler; \
		sudo systemctl start postgresql; \
		sudo systemctl enable postgresql; \
	elif command -v yum >/dev/null 2>&1; then \
		echo "📦 Using yum (RHEL/CentOS)"; \
		sudo yum update -y; \
		sudo yum install -y curl gcc openssl-devel pkgconfig postgresql-devel postgresql-server postgresql-contrib protobuf-compiler; \
		sudo postgresql-setup initdb; \
		sudo systemctl start postgresql; \
		sudo systemctl enable postgresql; \
	elif command -v pacman >/dev/null 2>&1; then \
		echo "📦 Using pacman (Arch Linux)"; \
		sudo pacman -Syu --noconfirm curl base-devel openssl pkgconf postgresql protobuf; \
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
		echo "🔄 Sourcing Rust environment..."; \
		. "$$HOME/.cargo/env"; \
	else \
		echo "✅ Rust already installed"; \
	fi
	@# Install Rust tools
	@echo "🔨 Installing Rust development tools..."
	@. "$$HOME/.cargo/env" && cargo install cargo-make cargo-watch
	@. "$$HOME/.cargo/env" && cargo install diesel_cli --no-default-features --features postgres
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
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/server && cargo run

# Run the store
store:
	@echo "🗄️  Starting store..."
	@# Source Rust environment and check if cargo is installed
	@export PATH="$$HOME/.cargo/bin:$$PATH" && \
	if [ -f "$$HOME/.cargo/env" ]; then \
		. "$$HOME/.cargo/env"; \
	fi && \
	if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi && \
	cd bin/store && cargo run

# =============================================================================
# Store-specific targets
# =============================================================================

# Run the store clean setup
store-clean-setup:
	@echo "🧹 Starting store clean setup..."
	@# Check if cargo-make is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if ! cargo make --version >/dev/null 2>&1; then \
		echo "❌ cargo-make not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && export PATH="$$HOME/.cargo/bin:$$PATH" && { \
		if command -v expect >/dev/null 2>&1; then \
			expect -c ' \
				set timeout 30; \
				spawn cargo make clean-setup; \
				expect "Enter password for database cleanup:"; \
				send "admin\r"; \
				expect { \
					"MessageStreamingService initialized successfully" { \
						puts "\n=== Setup completed successfully! ==="; \
						kill [exp_pid]; \
						exit 0 \
					} \
					"Address already in use" { \
						puts "\n=== Port conflict, setup complete ==="; \
						kill [exp_pid]; \
						exit 0 \
					} \
					timeout { \
						puts "\n=== Timeout - killing process ==="; \
						kill [exp_pid]; \
						exit 1 \
					} \
				}'; \
		else \
			printf "admin\n" | timeout 30 cargo make clean-setup || true; \
		fi; \
	}

# Run the store in watch mode
store-watch:
	@echo "👀 Starting store in watch mode..."
	@# Check if cargo-watch is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if ! cargo watch --version >/dev/null 2>&1; then \
		echo "❌ cargo-watch not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && DEBUG=true RUST_BACKTRACE=full cargo watch -x run

# Build the store
store-build:
	@echo "🔨 Building store..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && cargo build --release

# Run the store in production mode
store-prod:
	@echo "🚀 Starting store in production mode..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && cargo run --release

# Create store schema
store-generate-schema:
	@echo "📋 Generating store schema..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && CREATE_SCHEMA=true cargo run

# Generate store proto
store-generate-proto:
	@echo "🔧 Generating store proto..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
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
	@# Check if diesel CLI is installed
	@if ! command -v diesel >/dev/null 2>&1; then \
		echo "❌ Diesel CLI not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@echo "📝 Generating Diesel migration: $(NAME)..."
	@cd bin/store && diesel migration generate $(NAME)

# Run migrations
db-migrate-up:
	@echo "⬆️  Running database migrations..."
	@# Check if diesel CLI is installed
	@if ! command -v diesel >/dev/null 2>&1; then \
		echo "❌ Diesel CLI not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && diesel migration run

# Revert last migration
db-migrate-revert:
	@echo "↩️  Reverting last migration..."
	@# Check if diesel CLI is installed
	@if ! command -v diesel >/dev/null 2>&1; then \
		echo "❌ Diesel CLI not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && diesel migration revert

# =============================================================================
# Code formatting and quality targets
# =============================================================================

# Format Rust code
fmt:
	@echo "Formatting Rust code..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && cargo fmt --all
	@cd bin/server && cargo fmt --all
	@cd libs/hlc && cargo fmt --all
	@cd libs/merkle && cargo fmt --all
	@if [ -d "mcp-proto-generator" ]; then cd mcp-proto-generator && cargo fmt --all; fi
	@echo "✅ Code formatting complete! (Generated files in src/generated/ excluded)"

# Check code formatting
fmt-check:
	@echo "Checking code formatting..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
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
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
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

# =============================================================================
# Docker Build targets
# =============================================================================

# Build Docker image for Ubuntu test
docker-build-ubuntu:
	@echo "🐳 Building Docker image for Ubuntu test..."
	@docker build --target ubuntu-test -t crdt-ubuntu -f dockerfile-test-os .
	@echo "✅ Ubuntu Docker image built successfully!"

# Build Docker image for CentOS test
docker-build-centos:
	@echo "🐳 Building Docker image for CentOS test..."
	@docker build --target centos-test -t crdt-centos -f dockerfile-test-os .
	@echo "✅ CentOS Docker image built successfully!"

# Build Docker image for Arch Linux test
docker-build-arch:
	@echo "🐳 Building Docker image for Arch Linux test..."
	@docker build --target arch-test -t crdt-arch -f dockerfile-test-os .
	@echo "✅ Arch Linux Docker image built successfully!"

# Build Docker images for all distributions
docker-build-all: docker-build-ubuntu docker-build-centos docker-build-arch
	@echo "✅ All Docker images built successfully!"

# Run Docker container for Ubuntu testing
docker-run-ubuntu:
	@echo "🐳 Running Docker container for Ubuntu testing..."
	@docker run --rm -it crdt-ubuntu

# Run Docker container for CentOS testing
docker-run-centos:
	@echo "🐳 Running Docker container for CentOS testing..."
	@docker run --rm -it crdt-centos

# Run Docker container for Arch Linux testing
docker-run-arch:
	@echo "🐳 Running Docker container for Arch Linux testing..."
	@docker run --rm -it crdt-arch

# Run Docker containers for all operating systems (interactive)
docker-run-all:
	@echo "🐳 Running Docker containers for all operating systems..."
	@echo "Running Ubuntu container..."
	@docker run --rm -it crdt-ubuntu
	@echo "Running CentOS container..."
	@docker run --rm -it crdt-centos
	@echo "Running Arch Linux container..."
	@docker run --rm -it crdt-arch


# Run the store in watch mode with PG library configurations
jean-store-watch:
	@echo "Starting store in watch mode..."
	@# Check if cargo-watch is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if ! cargo watch --version >/dev/null 2>&1; then \
		echo "❌ cargo-watch not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && PQ_LIB_DIR=/opt/homebrew/opt/postgresql@14/lib/postgresql@14 LIBRARY_PATH=/opt/homebrew/opt/postgresql@14/lib/postgresql@14 cargo watch -x run

# Run experimental features
store-experimental:
	@echo "Running experimental features..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/store && EXPERIMENTAL_PERMISSIONS=true cargo run

# Run store initialize device
store-initialize-device:
	@echo "🔧 Initializing device..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
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

# Makefile for CRDT workspace

# PHONY targets (targets that don't create files)
.PHONY: all dev clean help install verify-install install-macos install-linux install-windows \
        server store store-clean-setup store-watch store-build store-prod \
        store-generate-schema store-generate-proto \
        db-migrate-generate db-migrate-up db-migrate-revert \
        fmt fmt-check git-cleanup setup-hooks \
        jean-store-watch store-experimental store-initialize-device \
        pm2-start pm2-stop pm2-restart pm2-status pm2-logs pm2-delete \
        docker-build-ubuntu docker-build-ubuntu-clean docker-build-ubuntu-fresh docker-build-centos docker-build-arch docker-build-all \
docker-run-ubuntu docker-run-centos docker-run-arch docker-run-all docker-ubuntu-memory-optimized \
        docker-compose-up docker-compose-down docker-compose-restart docker-compose-logs docker-compose-ps

# Default target
all: dev

# Help target
help:
	@echo "Available targets:"
	@echo "  install                 - Install all dependencies and setup the project (auto-detects OS)"
	@echo "  install-macos           - Install dependencies specifically for macOS"
	@echo "  install-linux           - Install dependencies specifically for Linux"
	@echo "  install-windows         - Install dependencies specifically for Windows"
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
	@echo "  docker-build-ubuntu-clean - Build Ubuntu Docker image with cleanup"
	@echo "  docker-build-ubuntu-fresh - Build fresh Ubuntu Docker image (no cache)"
	@echo "  docker-build-centos     - Build Docker image for CentOS test"
	@echo "  docker-build-arch       - Build Docker image for Arch Linux test"
	@echo "  docker-build-all        - Build Docker images for all distributions"
	@echo "  docker-run-ubuntu       - Run Docker container for Ubuntu testing"
	@echo "  docker-run-centos       - Run Docker container for CentOS testing"
	@echo "  docker-run-arch         - Run Docker container for Arch Linux testing"
	@echo "  docker-run-all          - Run Docker containers for all operating systems"
	@echo "  docker-ubuntu-memory-optimized - Build and run memory-optimized Ubuntu container"
	@echo "  docker-compose-up       - Start TimescaleDB and Redis services using Docker Compose"
	@echo "  docker-compose-down     - Stop and remove Docker Compose services"
	@echo "  docker-compose-restart  - Restart Docker Compose services"
	@echo "  docker-compose-logs     - Show logs from Docker Compose services"
	@echo "  docker-compose-ps       - Show status of Docker Compose services"
	@echo "  help                    - Show this help message"

# =============================================================================
# Installation and Setup targets
# =============================================================================

# Common setup tasks
setup-env:
	@# Setup environment file
	@if [ "$$(uname -s | cut -c1-10)" = "MINGW32_NT" ] || [ "$$(uname -s | cut -c1-10)" = "MINGW64_NT" ] || [ "$$(uname -s | cut -c1-6)" = "CYGWIN" ] || powershell -Command "exit 0" 2>/dev/null; then \
		powershell -Command "if (!(Test-Path '.env')) { Write-Host '📄 Setting up environment file...'; Copy-Item '.env-sample' '.env'; Write-Host '✅ Environment file created from .env-sample' } else { Write-Host '✅ Environment file already exists' }"; \
	else \
		if [ ! -f ".env" ]; then \
			echo "📄 Setting up environment file..."; \
			cp .env-sample .env; \
			echo "✅ Environment file created from .env-sample"; \
		else \
			echo "✅ Environment file already exists"; \
		fi; \
	fi

install-rust:
	@# Install Rust 1.86.0 specifically
	@if [ "$$(uname -s | cut -c1-10)" = "MINGW32_NT" ] || [ "$$(uname -s | cut -c1-10)" = "MINGW64_NT" ] || [ "$$(uname -s | cut -c1-6)" = "CYGWIN" ] || powershell -Command "exit 0" 2>/dev/null; then \
		powershell -Command "if (!(Get-Command rustc -ErrorAction SilentlyContinue)) { Write-Host '🦀 Installing Rust 1.86.0...'; Invoke-WebRequest -Uri 'https://win.rustup.rs/' -OutFile 'rustup-init.exe'; .\rustup-init.exe -y --default-toolchain 1.86.0; Remove-Item rustup-init.exe } else { Write-Host '🔍 Checking Rust version...'; $$rustVersion = (rustc --version).Split(' ')[1]; if ($$rustVersion -ne '1.86.0') { Write-Host '⚠️  Current Rust version: ' + $$rustVersion + ', required: 1.86.0'; Write-Host '🔄 Installing Rust 1.86.0...'; rustup install 1.86.0; rustup default 1.86.0 } else { Write-Host '✅ Rust 1.86.0 already installed' } }"; \
	else \
		if ! command -v rustc >/dev/null 2>&1; then \
			echo "🦀 Installing Rust 1.86.0..."; \
			curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.86.0; \
			if [ -f "$$HOME/.cargo/env" ]; then \
				. "$$HOME/.cargo/env"; \
			fi; \
		else \
			echo "🔍 Checking Rust version..."; \
			RUST_VERSION=$$(rustc --version | cut -d' ' -f2); \
			if [ "$$RUST_VERSION" != "1.86.0" ]; then \
				echo "⚠️  Current Rust version: $$RUST_VERSION, required: 1.86.0"; \
				echo "🔄 Installing Rust 1.86.0..."; \
				rustup install 1.86.0; \
				rustup default 1.86.0; \
			else \
				echo "✅ Rust 1.86.0 already installed"; \
			fi; \
		fi; \
	fi

install-rust-tools:
	@# Install Rust tools
	@echo "🔨 Installing Rust development tools..."
	@if [ "$$(uname -s | cut -c1-10)" = "MINGW32_NT" ] || [ "$$(uname -s | cut -c1-10)" = "MINGW64_NT" ] || [ "$$(uname -s | cut -c1-6)" = "CYGWIN" ] || powershell -Command "exit 0" 2>/dev/null; then \
		powershell -Command "$$env:PATH += ';' + $$env:USERPROFILE + '\.cargo\bin'; cargo install cargo-make cargo-watch"; \
		powershell -Command "$$env:PATH += ';' + $$env:USERPROFILE + '\.cargo\bin'; cargo install diesel_cli --no-default-features --features postgres"; \
	else \
		export PATH="$$HOME/.cargo/bin:$$PATH"; \
		if [ -f "$$HOME/.cargo/env" ]; then \
			. "$$HOME/.cargo/env"; \
		fi; \
		cargo install cargo-make cargo-watch; \
		export PATH="$$HOME/.cargo/bin:$$PATH"; \
		if [ -f "$$HOME/.cargo/env" ]; then \
			. "$$HOME/.cargo/env"; \
		fi; \
		cargo install diesel_cli --no-default-features --features postgres; \
	fi

start-docker-services:
	@echo "⏳ Waiting 5 seconds before starting Docker services..."
	@sleep 5
	@# Start Docker Compose services
	@echo "🐳 Starting Docker Compose services (TimescaleDB and Redis)..."
	@if [ "$$(uname -s | cut -c1-10)" = "MINGW32_NT" ] || [ "$$(uname -s | cut -c1-10)" = "MINGW64_NT" ] || [ "$$(uname -s | cut -c1-6)" = "CYGWIN" ] || powershell -Command "exit 0" 2>/dev/null; then \
		powershell -Command "if (Test-Path 'bin/store/docker-compose.yml') { if (Get-Command docker -ErrorAction SilentlyContinue) { try { docker info | Out-Null; docker-compose -f bin/store/docker-compose.yml up -d; Write-Host '✅ Docker Compose services started successfully!' } catch { Write-Host '⚠️  Docker daemon is not running. Please start Docker Desktop and try again.'; Write-Host '   You can manually start services later with: make docker-compose-up' } } else { Write-Host '⚠️  Docker not found. Please install Docker Desktop and try again.'; Write-Host '   You can manually start services later with: make docker-compose-up' } } else { Write-Host '⚠️  Docker Compose file not found at bin/store/docker-compose.yml' }"; \
	else \
		if [ -f "bin/store/docker-compose.yml" ]; then \
			if command -v docker >/dev/null 2>&1; then \
				if docker info >/dev/null 2>&1; then \
					docker-compose -f bin/store/docker-compose.yml up -d; \
					echo "✅ Docker Compose services started successfully!"; \
				else \
					echo "⚠️  Docker daemon is not running. Please start Docker Desktop and try again."; \
					echo "   You can manually start services later with: make docker-compose-up"; \
				fi; \
			else \
				echo "⚠️  Docker not found. Please install Docker Desktop and try again."; \
				echo "   You can manually start services later with: make docker-compose-up"; \
			fi; \
		else \
			echo "⚠️  Docker Compose file not found at bin/store/docker-compose.yml"; \
		fi; \
	fi

finalize-setup:
	@# Seeding database
	@echo "🌱 Seeding Store database..."
	@if [ "$$(uname -s | cut -c1-10)" = "MINGW32_NT" ] || [ "$$(uname -s | cut -c1-10)" = "MINGW64_NT" ] || [ "$$(uname -s | cut -c1-6)" = "CYGWIN" ] || powershell -Command "exit 0" 2>/dev/null; then \
		powershell -Command "$$env:PATH += ';' + $$env:USERPROFILE + '\.cargo\bin'; make store-clean-setup"; \
	else \
		export PATH="$$HOME/.cargo/bin:$$PATH" && make store-clean-setup; \
	fi
	@echo "✅ Store database seeded!"
	@# Setup git hooks
	@make setup-hooks
	@echo "✅ Installation complete! Run 'make store' to start the project."

# One-command installer for seamless project setup
install:
	@echo "🚀 Setting up CRDT Workspace - One-command installer"
	@make setup-env
	@echo "📋 Detecting operating system..."
	@if [ "$$(uname)" = "Darwin" ]; then \
		echo "🍎 macOS detected"; \
		make install-macos-deps; \
	elif [ "$$(uname)" = "Linux" ]; then \
		echo "🐧 Linux detected"; \
		make install-linux-deps; \
	elif [ "$$(uname -s | cut -c1-10)" = "MINGW32_NT" ] || [ "$$(uname -s | cut -c1-10)" = "MINGW64_NT" ] || [ "$$(uname -s | cut -c1-6)" = "CYGWIN" ] || powershell -Command "exit 0" 2>/dev/null; then \
		echo "🪟 Windows detected"; \
		make install-windows-deps; \
	else \
		echo "❌ Unsupported operating system: $$(uname)"; \
		echo "Please install dependencies manually:"; \
		echo "  - Rust 1.86.0 (https://rustup.rs/)"; \
		echo "  - PostgreSQL"; \
		echo "  - Protocol Buffers (protoc)"; \
		echo "  - cargo-make, cargo-watch, diesel_cli"; \
		exit 1; \
	fi
	@make install-rust
	@make install-rust-tools
	@make finalize-setup
	

# macOS system dependencies installation
install-macos-deps:
	@echo "🔧 Installing system dependencies for macOS..."
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
	@# Install Protocol Buffers
	@export PATH="/opt/homebrew/bin:$$PATH"; \
	if ! command -v protoc >/dev/null 2>&1; then \
		echo "📦 Installing Protocol Buffers..."; \
		brew install protobuf; \
	else \
		echo "✅ Protocol Buffers already installed"; \
	fi
	@echo "🎉 macOS system dependencies installed!"
	@make start-docker-services

# Legacy target for backward compatibility
install-macos: install-macos-deps install-rust install-rust-tools start-docker-services finalize-setup

# Linux system dependencies installation
install-linux-deps:
	@echo "🔧 Installing system dependencies for Linux..."
	@# Detect package manager and install PostgreSQL and Protocol Buffers
	@if command -v apt-get >/dev/null 2>&1; then \
		echo "📦 Using apt-get (Debian/Ubuntu)"; \
		sudo apt-get update; \
		sudo apt-get install -y curl build-essential libssl-dev pkg-config libpq-dev postgresql postgresql-contrib protobuf-compiler; \
		sudo systemctl start postgresql; \
		sudo systemctl enable postgresql; \
		echo "✅ PostgreSQL and Protocol Buffers installed via apt-get"; \
	elif command -v yum >/dev/null 2>&1; then \
		echo "📦 Using yum (RHEL/CentOS)"; \
		sudo yum update -y; \
		sudo yum install -y curl gcc openssl-devel pkgconfig postgresql-devel postgresql-server postgresql-contrib protobuf-compiler; \
		sudo postgresql-setup initdb; \
		sudo systemctl start postgresql; \
		sudo systemctl enable postgresql; \
		echo "✅ PostgreSQL and Protocol Buffers installed via yum"; \
	elif command -v pacman >/dev/null 2>&1; then \
		echo "📦 Using pacman (Arch Linux)"; \
		sudo pacman -Syu --noconfirm curl base-devel openssl pkgconf postgresql protobuf; \
		sudo systemctl start postgresql; \
		sudo systemctl enable postgresql; \
		echo "✅ PostgreSQL and Protocol Buffers installed via pacman"; \
	elif command -v apk >/dev/null 2>&1; then \
		echo "📦 Using apk (Alpine Linux)"; \
		apk update; \
		apk add --no-cache curl build-base openssl-dev pkgconfig postgresql-dev postgresql postgresql-libs protobuf-dev protoc musl-dev; \
		echo "✅ PostgreSQL and Protocol Buffers installed via apk"; \
	else \
		echo "❌ Unsupported package manager. Please install PostgreSQL and Protocol Buffers manually."; \
		exit 1; \
	fi
	@echo "🎉 Linux system dependencies installed!"

# Legacy target for backward compatibility
install-linux: install-linux-deps install-rust install-rust-tools start-docker-services finalize-setup

# Windows system dependencies installation
install-windows-deps:
	@echo "🔧 Installing dependencies for Windows..."
	@# Check if Chocolatey is installed
	@powershell -Command "if (!(Get-Command choco -ErrorAction SilentlyContinue)) { Write-Host '📦 Installing Chocolatey...'; Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1')) } else { Write-Host '✅ Chocolatey already installed' }"
	@# Install PostgreSQL using Chocolatey
	@powershell -Command "if (!(Get-Command psql -ErrorAction SilentlyContinue)) { Write-Host '🐘 Installing PostgreSQL...'; choco install postgresql14 -y; Write-Host '🔄 Starting PostgreSQL service...'; Start-Service postgresql-x64-14 } else { Write-Host '✅ PostgreSQL already installed' }"
	@# Install Protocol Buffers using Chocolatey
	@powershell -Command "if (!(Get-Command protoc -ErrorAction SilentlyContinue)) { Write-Host '📦 Installing Protocol Buffers...'; choco install protoc -y } else { Write-Host '✅ Protocol Buffers already installed' }"
	@# Install Git if not present
	@powershell -Command "if (!(Get-Command git -ErrorAction SilentlyContinue)) { Write-Host '📦 Installing Git...'; choco install git -y } else { Write-Host '✅ Git already installed' }"

# Legacy target for backward compatibility
install-windows: setup-env install-windows-deps install-rust install-rust-tools start-docker-services finalize-setup

# Verify installation
verify-install:
	@echo "🔍 Verifying installation..."
	@echo "Checking Rust..."
	@if command -v rustc >/dev/null 2>&1 || powershell -Command "rustc --version" 2>/dev/null; then \
		if command -v rustc >/dev/null 2>&1; then \
			rustc --version; \
		else \
			powershell -Command "rustc --version"; \
		fi; \
	else \
		echo "❌ Rust not found" && exit 1; \
	fi
	@echo "Checking Cargo..."
	@if command -v cargo >/dev/null 2>&1 || powershell -Command "cargo --version" 2>/dev/null; then \
		if command -v cargo >/dev/null 2>&1; then \
			cargo --version; \
		else \
			powershell -Command "cargo --version"; \
		fi; \
	else \
		echo "❌ Cargo not found" && exit 1; \
	fi
	@echo "Checking cargo-make..."
	@if command -v cargo >/dev/null 2>&1; then \
		cargo make --version || (echo "❌ cargo-make not found" && exit 1); \
	else \
		powershell -Command "cargo make --version" || (echo "❌ cargo-make not found" && exit 1); \
	fi
	@echo "Checking cargo-watch..."
	@if command -v cargo >/dev/null 2>&1; then \
		cargo watch --version || (echo "❌ cargo-watch not found" && exit 1); \
	else \
		powershell -Command "cargo watch --version" || (echo "❌ cargo-watch not found" && exit 1); \
	fi
	@echo "Checking diesel CLI..."
	@if command -v diesel >/dev/null 2>&1 || powershell -Command "diesel --version" 2>/dev/null; then \
		if command -v diesel >/dev/null 2>&1; then \
			diesel --version; \
		else \
			powershell -Command "diesel --version"; \
		fi; \
	else \
		echo "❌ diesel CLI not found" && exit 1; \
	fi
	@echo "Checking Protocol Buffers compiler..."
	@if command -v protoc >/dev/null 2>&1 || powershell -Command "protoc --version" 2>/dev/null; then \
		if command -v protoc >/dev/null 2>&1; then \
			protoc --version; \
		else \
			powershell -Command "protoc --version"; \
		fi; \
	else \
		echo "❌ Protocol Buffers compiler (protoc) not found" && exit 1; \
	fi
	@echo "Checking PostgreSQL..."
	@if command -v psql >/dev/null 2>&1; then \
		psql --version; \
	elif [ -f "/opt/homebrew/bin/psql" ]; then \
		/opt/homebrew/bin/psql --version; \
		echo "⚠️  PostgreSQL found but not in PATH. Add this to your shell profile:"; \
		echo "   export PATH=\"/opt/homebrew/bin:\$$PATH\""; \
	elif powershell -Command "psql --version" 2>/dev/null; then \
		powershell -Command "psql --version"; \
	else \
		echo "❌ PostgreSQL not found" && exit 1; \
	fi
	@echo "Checking Docker Compose services..."
	@if [ -f "bin/store/docker-compose.yml" ]; then \
		if command -v docker-compose >/dev/null 2>&1; then \
			docker-compose -f bin/store/docker-compose.yml ps; \
		else \
			echo "⚠️  Docker Compose not found, skipping service check"; \
		fi; \
	else \
		echo "⚠️  Docker Compose file not found at bin/store/docker-compose.yml"; \
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
				set timeout 80; \
				spawn cargo make clean-setup; \
				expect "Enter password for database cleanup:"; \
				send "admin\r"; \
				expect { \
					"Store is running" { \
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
# Docker Compose targets
# =============================================================================

# Docker Compose file path
DOCKER_COMPOSE_FILE := bin/store/docker-compose.yml

# Start TimescaleDB and Redis services using Docker Compose
docker-compose-up:
	@echo "🐳 Starting Docker Compose services..."
	@if [ ! -f "$(DOCKER_COMPOSE_FILE)" ]; then \
		echo "❌ Error: Docker Compose file not found at $(DOCKER_COMPOSE_FILE)"; \
		echo "Please ensure the docker-compose.yml file exists in bin/store/"; \
		exit 1; \
	fi
	@if command -v docker >/dev/null 2>&1; then \
		if docker info >/dev/null 2>&1; then \
			docker-compose -f $(DOCKER_COMPOSE_FILE) up -d; \
			echo "✅ Docker Compose services started successfully!"; \
		else \
			echo "⚠️  Docker daemon is not running. Please start Docker Desktop and try again."; \
			echo "   You can manually start services later with: make docker-compose-up"; \
		fi; \
	else \
		echo "⚠️  Docker not found. Please install Docker Desktop and try again."; \
		echo "   You can manually start services later with: make docker-compose-up"; \
	fi

# Stop and remove Docker Compose services
docker-compose-down:
	@echo "🐳 Stopping Docker Compose services..."
	@if [ ! -f "$(DOCKER_COMPOSE_FILE)" ]; then \
		echo "❌ Error: Docker Compose file not found at $(DOCKER_COMPOSE_FILE)"; \
		exit 1; \
	fi
	@if command -v docker >/dev/null 2>&1; then \
		if docker info >/dev/null 2>&1; then \
			docker-compose -f $(DOCKER_COMPOSE_FILE) down; \
			echo "✅ Docker Compose services stopped successfully!"; \
		else \
			echo "⚠️  Docker daemon is not running. Cannot stop services."; \
		fi; \
	else \
		echo "⚠️  Docker not found. Please install Docker Desktop."; \
	fi

# Restart Docker Compose services
docker-compose-restart:
	@echo "🐳 Restarting Docker Compose services..."
	@if [ ! -f "$(DOCKER_COMPOSE_FILE)" ]; then \
		echo "❌ Error: Docker Compose file not found at $(DOCKER_COMPOSE_FILE)"; \
		exit 1; \
	fi
	@docker-compose -f $(DOCKER_COMPOSE_FILE) restart
	@echo "✅ Docker Compose services restarted successfully!"

# Show logs from Docker Compose services
docker-compose-logs:
	@echo "📋 Docker Compose service logs:"
	@if [ ! -f "$(DOCKER_COMPOSE_FILE)" ]; then \
		echo "❌ Error: Docker Compose file not found at $(DOCKER_COMPOSE_FILE)"; \
		exit 1; \
	fi
	@docker-compose -f $(DOCKER_COMPOSE_FILE) logs -f

# Show status of Docker Compose services
docker-compose-ps:
	@echo "📊 Docker Compose service status:"
	@if [ ! -f "$(DOCKER_COMPOSE_FILE)" ]; then \
		echo "❌ Error: Docker Compose file not found at $(DOCKER_COMPOSE_FILE)"; \
		exit 1; \
	fi
	@docker-compose -f $(DOCKER_COMPOSE_FILE) ps

# =============================================================================
# Docker Build targets
# =============================================================================

# Build Docker image for Ubuntu test
docker-build-ubuntu:
	@echo "🐳 Building Docker image for Ubuntu test..."
	@docker build --target ubuntu-test -t crdt-ubuntu -f dockerfile-test-os \
		--build-arg BUILDKIT_INLINE_CACHE=1 \
		--build-arg CARGO_BUILD_JOBS=1 \
		--memory=8g --memory-swap=12g \
		--shm-size=2g .
	@echo "✅ Ubuntu Docker image built successfully!"

# Build Docker image for CentOS test
docker-build-centos:
	@echo "🐳 Building Docker image for CentOS test..."
	@docker build --target centos-test -t crdt-centos -f dockerfile-test-os \
		--memory=8g --memory-swap=12g \
		--shm-size=2g .
	@echo "✅ CentOS Docker image built successfully!"

# Build Docker image for Arch Linux test
docker-build-arch:
	@echo "🐳 Building Docker image for Arch Linux test..."
	@docker build --target arch-test -t crdt-arch -f dockerfile-test-os \
		--memory=8g --memory-swap=12g \
		--shm-size=2g .
	@echo "✅ Arch Linux Docker image built successfully!"

# Build Docker images for all distributions
docker-build-all: docker-build-ubuntu docker-build-centos docker-build-arch
	@echo "✅ All Docker images built successfully!"

# Build Docker image for Ubuntu test with cleanup
docker-build-ubuntu-clean:
	@echo "🧹 Cleaning up previous Ubuntu Docker artifacts..."
	@docker system prune -f --filter "label=stage=ubuntu-test" || true
	@docker rmi crdt-ubuntu || true
	@make docker-build-ubuntu

# Build Docker image for Ubuntu test with no cache
docker-build-ubuntu-fresh:
	@echo "🐳 Building fresh Ubuntu Docker image (no cache)..."
	@docker build --no-cache --target ubuntu-test -t crdt-ubuntu -f dockerfile-test-os \
		--build-arg BUILDKIT_INLINE_CACHE=1 \
		--build-arg CARGO_BUILD_JOBS=1 \
		--memory=8g --memory-swap=12g \
		--shm-size=2g .
	@echo "✅ Fresh Ubuntu Docker image built successfully!"

# Run Docker container for Ubuntu testing
docker-run-ubuntu:
	@echo "🐳 Running Docker container for Ubuntu testing..."
	@docker run --rm -it \
		--memory=8g --memory-swap=12g \
		--cpus=1 \
		--shm-size=2g \
		--ulimit nofile=65536:65536 \
		--ulimit memlock=-1:-1 \
		--env CARGO_BUILD_JOBS=1 \
		--env RUST_BACKTRACE=1 \
		--env RUSTFLAGS="-C target-cpu=generic -C opt-level=1 -C debuginfo=0 -C incremental=false" \
		--env CARGO_PROFILE_DEV_DEBUG=0 \
		--env CARGO_PROFILE_DEV_INCREMENTAL=false \
		crdt-ubuntu

# Run Docker container for CentOS testing
docker-run-centos:
	@echo "🐳 Running Docker container for CentOS testing..."
	@docker run --rm -it \
		--memory=8g --memory-swap=12g \
		--cpus=1 \
		--shm-size=2g \
		--ulimit nofile=65536:65536 \
		--ulimit memlock=-1:-1 \
		--env CARGO_BUILD_JOBS=1 \
		--env RUST_BACKTRACE=1 \
		--env RUSTFLAGS="-C target-cpu=generic -C opt-level=1 -C debuginfo=0 -C incremental=false" \
		--env CARGO_PROFILE_DEV_DEBUG=0 \
		--env CARGO_PROFILE_DEV_INCREMENTAL=false \
		crdt-centos

# Run Docker container for Arch Linux testing
docker-run-arch:
	@echo "🐳 Running Docker container for Arch Linux testing..."
	@docker run --rm -it \
		--memory=8g --memory-swap=12g \
		--cpus=1 \
		--shm-size=2g \
		--ulimit nofile=65536:65536 \
		--ulimit memlock=-1:-1 \
		--env CARGO_BUILD_JOBS=1 \
		--env RUST_BACKTRACE=1 \
		--env RUSTFLAGS="-C target-cpu=generic -C opt-level=1 -C debuginfo=0 -C incremental=false" \
		--env CARGO_PROFILE_DEV_DEBUG=0 \
		--env CARGO_PROFILE_DEV_INCREMENTAL=false \
		crdt-arch

# Run Docker containers for all operating systems (interactive)
docker-run-all:
	@echo "🐳 Running Docker containers for all operating systems..."
	@echo "Running Ubuntu container..."
	@docker run --rm -it crdt-ubuntu
	@echo "Running CentOS container..."
	@docker run --rm -it crdt-centos
	@echo "Running Arch Linux container..."
	@docker run --rm -it crdt-arch

# Optimized Ubuntu Docker build and run for Linux environments with memory constraints
docker-ubuntu-memory-optimized:
	@echo "🐳 Building and running memory-optimized Ubuntu Docker container..."
	@docker build --target ubuntu-test -t crdt-ubuntu-optimized -f dockerfile-test-os \
		--build-arg BUILDKIT_INLINE_CACHE=1 \
		--build-arg CARGO_BUILD_JOBS=1 \
		--memory=12g --memory-swap=16g \
		--shm-size=4g .
	@docker run --rm -it \
		--memory=12g --memory-swap=16g \
		--cpus=1 \
		--shm-size=4g \
		--ulimit nofile=65536:65536 \
		--ulimit memlock=-1:-1 \
		--ulimit stack=8388608 \
		--env CARGO_BUILD_JOBS=1 \
		--env RUST_BACKTRACE=1 \
		--env RUSTFLAGS="-C target-cpu=generic -C opt-level=1 -C debuginfo=0 -C incremental=false -C link-arg=-Wl,--no-keep-memory" \
		--env CARGO_PROFILE_DEV_DEBUG=0 \
		--env CARGO_PROFILE_DEV_INCREMENTAL=false \
		--env CARGO_PROFILE_DEV_OPT_LEVEL=1 \
		crdt-ubuntu-optimized


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
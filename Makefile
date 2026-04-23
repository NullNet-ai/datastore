# Makefile for CRDT workspace

# PHONY targets (targets that don't create files)
.PHONY: all dev clean help install install-seeding verify-install install-macos install-linux install-windows \
        server store store-clean-setup store-clean-setup-lm store-watch store-build \
        store-prod store-build-linux store-build-linux-bx store-build-linux-clean store-build-linux-zig store-build-docker store-build-docker-legacy store-build-docker-nocache store-build-docker-memsafe store-build-docker-memsafe-legacy store-build-docker-auth docker-diagnose \
        store-build-debian-amd64 store-build-debian-arm64 \
        redis-flush counter-service counter-service-test counter-service-test-integration counter-service-test-all \
        store-generate-schema store-generate-proto store-generator-schema store-generator-proto store-generator-all \
        db-migrate-generate db-migrate-up db-migrate-revert \
        fmt fmt-check git-cleanup setup-hooks ensure-hooks \
        jean-store-watch store-experimental store-initialize-device \
        pm2-start pm2-stop pm2-restart pm2-status pm2-logs pm2-delete \
        docker-build-ubuntu docker-build-ubuntu-clean docker-build-ubuntu-fresh docker-build-centos docker-build-arch docker-build-all \
docker-run-ubuntu docker-run-centos docker-run-arch docker-run-all docker-ubuntu-memory-optimized \
        docker-compose-up docker-compose-down docker-compose-restart docker-compose-logs docker-compose-ps docker-prune

# Default target
all: dev

# Help target
help:
	@echo "Available targets:"
	@echo "  install                 - Install all dependencies and setup the project (auto-detects OS)"
	@echo "  install-seeding         - Install all dependencies and finalize setup with seeding (auto-detects OS)"
	@echo "  install-macos           - Install dependencies specifically for macOS"
	@echo "  install-linux           - Install dependencies specifically for Linux"
	@echo "  install-windows         - Install dependencies specifically for Windows"
	@echo "  verify-install          - Verify that all required tools are installed"
	@echo "  dev                     - Run both server and store in parallel"
	@echo "  server                  - Run the server"
	@echo "  store                   - Run the store"
	@echo "  store-clean-setup       - Run store clean setup"
	@echo "  store-clean-setup-lm    - Run store clean setup with minimal memory"
	@echo "  store-watch             - Run store in watch mode with debug"
	@echo "  store-build             - Build store in release mode"
	@echo "  store-prod              - Run store compiled store binary (target/release/store) in production mode"
	@echo "  store-build-linux       - Build Linux store binary in Docker (override LINUX_PLATFORM)"
	@echo "  store-build-linux-bx    - Build Linux store binary via buildx to ./dist then ./store"
	@echo "  store-build-linux-clean - Clean builder caches and pull fresh base image"
	@echo "  store-build-linux-zig   - Cross-compile Linux binary using cargo-zigbuild"
	@echo "  store-build-docker      - Build via Dockerfile artifact stage and copy to ./store"
	@echo "  store-build-docker-legacy - Build with legacy builder (DOCKER_BUILDKIT=0)"
	@echo "  store-build-docker-nocache - Build artifact with --no-cache and copy to ./store"
	@echo "  store-build-docker-memsafe - Build with minimal memory (jobs=1, cgu=1, opt=0)"
	@echo "  store-build-docker-memsafe-legacy - Same as memsafe using legacy builder"
	@echo "  store-build-docker-auth - Build using private registry token via BuildKit secret"
	@echo "  docker-diagnose         - Print docker info, disk usage, builder and buildx status"
	@echo "  store-build-debian-amd64 - Cross-compile Debian-compatible x86_64 Linux binary"
	@echo "  store-build-debian-arm64 - Cross-compile Debian-compatible aarch64 Linux binary"
	@echo "  store-initialize-device - Initialize device and wait for PostgreSQL listener"
	@echo "  store-generate-schema   - Generate store schema"
	@echo "  store-generate-proto    - Generate store proto files"
	@echo "  store-generator-schema  - Generate schema (standalone, no store build)"
	@echo "  store-generator-proto   - Generate proto (standalone, no store build)"
	@echo "  store-generator-all     - Generate all (standalone, no store build)"
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
	@echo "  redis-flush             - Flush Redis using REDIS_URL from env (default redis://127.0.0.1:6379)"
	@echo "  counter-service        - Start the counter-service (gRPC + Redis code generation)"
	@echo "  counter-service-test    - Run counter-service unit tests"
	@echo "  counter-service-test-integration - Run counter-service integration tests (requires Redis)"
	@echo "  counter-service-test-all - Flush Redis, run all counter-service tests (unit + integration)"
	@echo "  remove-schema-table-macros - Remove table macros from schema.rs based on tables dir"
	@echo "  docker-prune            - Prune Docker system data and builder caches"
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
	@# Install Rust 1.91.1 specifically
	@if [ "$$(uname -s | cut -c1-10)" = "MINGW32_NT" ] || [ "$$(uname -s | cut -c1-10)" = "MINGW64_NT" ] || [ "$$(uname -s | cut -c1-6)" = "CYGWIN" ] || powershell -Command "exit 0" 2>/dev/null; then \
		powershell -Command "if (!(Get-Command rustc -ErrorAction SilentlyContinue)) { Write-Host '🦀 Installing Rust 1.91.1...'; Invoke-WebRequest -Uri 'https://win.rustup.rs/' -OutFile 'rustup-init.exe'; .\rustup-init.exe -y --default-toolchain 1.91.1; Remove-Item rustup-init.exe } else { Write-Host '🔍 Checking Rust version...'; $$rustVersion = (rustc --version).Split(' ')[1]; if ([version]$$rustVersion -lt [version]'1.91.1') { Write-Host '⚠️  Current Rust version: ' + $$rustVersion + ', required: >= 1.91.1'; Write-Host '🔄 Installing Rust 1.91.1...'; rustup install 1.91.1; rustup default 1.91.1 } else { Write-Host '✅ Compatible Rust version detected (' + $$rustVersion + ')' } }"; \
	else \
		if ! command -v rustc >/dev/null 2>&1; then \
			echo "🦀 Installing Rust 1.91.1..."; \
			curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.91.1; \
			if [ -f "$$HOME/.cargo/env" ]; then \
				. "$$HOME/.cargo/env"; \
			fi; \
		else \
			echo "🔍 Checking Rust version..."; \
			RUST_VERSION=$$(rustc --version | cut -d' ' -f2); \
			REQ=1.91.1; \
			if [ "$$(printf '%s\n' $$REQ $$RUST_VERSION | sort -V | head -n1)" != "$$REQ" ]; then \
				echo "⚠️  Current Rust version: $$RUST_VERSION, required: >= $$REQ"; \
				echo "🔄 Installing Rust $$REQ..."; \
				rustup install $$REQ; \
				rustup default $$REQ; \
			else \
				echo "✅ Compatible Rust version detected ($$RUST_VERSION)"; \
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
	@if [ ! -f "$(DOCKER_COMPOSE_FILE)" ]; then \
		echo "❌ Error: Docker Compose file not found at $(DOCKER_COMPOSE_FILE)"; \
		exit 1; \
	fi
	@if [ ! -f "$(ENV_STORE_FILE)" ]; then \
		echo "❌ Error: Env file not found at $(ENV_STORE_FILE)"; \
		exit 1; \
	fi
	@if [ ! -f "$(ENV_SYNC_SERVER_FILE)" ]; then \
		echo "❌ Error: Env file not found at $(ENV_SYNC_SERVER_FILE)"; \
		exit 1; \
	fi
	@# Start Docker Compose services
	@echo "🐳 Starting Docker Compose services..."
	@if [ "$$(uname -s | cut -c1-10)" = "MINGW32_NT" ] || [ "$$(uname -s | cut -c1-10)" = "MINGW64_NT" ] || [ "$$(uname -s | cut -c1-6)" = "CYGWIN" ] || powershell -Command "exit 0" 2>/dev/null; then \
		powershell -Command "if (Get-Command docker -ErrorAction SilentlyContinue) { try { docker info | Out-Null; docker-compose -f $(DOCKER_COMPOSE_FILE) up -d; Write-Host '✅ Docker Compose services started successfully!' } catch { Write-Host '⚠️  Docker daemon is not running. Please start Docker Desktop and try again.'; Write-Host '   You can manually start services later with: make docker-compose-up' } } else { Write-Host '⚠️  Docker not found. Please install Docker Desktop and try again.'; Write-Host '   You can manually start services later with: make docker-compose-up' }"; \
	else \
		if command -v docker >/dev/null 2>&1; then \
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
		echo "  - Rust 1.91.1 (https://rustup.rs/)"; \
		echo "  - PostgreSQL"; \
		echo "  - Protocol Buffers (protoc)"; \
		echo "  - cargo-make, cargo-watch, diesel_cli"; \
		exit 1; \
	fi
	@make install-rust
	@make install-rust-tools
	
install-seeding:
	@echo "🚀 Setting up CRDT Workspace - One-command installer (with seeding)"
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
		echo "  - Rust 1.91.1 (https://rustup.rs/)"; \
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
	@# Install Protocol Buffers
	@export PATH="/opt/homebrew/bin:$$PATH"; \
	if ! command -v protoc >/dev/null 2>&1; then \
		echo "📦 Installing Protocol Buffers..."; \
		brew install protobuf; \
	else \
		echo "✅ Protocol Buffers already installed"; \
	fi
	@# Install libpq for PostgreSQL client headers/libs (required by Rust postgres crates at link time)
	@export PATH="/opt/homebrew/bin:$$PATH"; \
	if ! brew list libpq >/dev/null 2>&1; then \
		echo "📦 Installing libpq..."; \
		brew install libpq; \
	else \
		echo "✅ libpq already installed"; \
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
	@if [ -f "$(DOCKER_COMPOSE_FILE)" ]; then \
		if command -v docker-compose >/dev/null 2>&1; then \
			docker-compose -f $(DOCKER_COMPOSE_FILE) ps; \
		else \
			echo "⚠️  Docker Compose not found, skipping service check"; \
		fi; \
	else \
		echo "⚠️  Docker Compose file not found at $(DOCKER_COMPOSE_FILE)"; \
	fi
	@echo "✅ All required tools are installed and working!"

# =============================================================================
# Development targets
# =============================================================================

# Run both server and store in parallel (ensure-hooks runs on first use after clone)
dev: ensure-hooks
	@export PATH="$$HOME/.cargo/bin:$$PATH" || true; \
	echo "🚀 Starting server and store..."; \
	make -j 2 server store

# Run the server
sync-server:
	@echo "🖥️  Starting server..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/server && cargo run

# Build the sync server in production mode
sync-server-build:
	@echo "🖥️  Starting sync server..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/server && cargo build --release

# Run the sync server in production mode
sync-server-prod:
	@echo "🖥️  Starting sync server..."
	@# Check if cargo is installed
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@cd bin/server && cargo run --release

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
	PG_LIB_DIR="$$(pg_config --libdir 2>/dev/null || true)" && \
	PG_INCLUDE_DIR="$$(pg_config --includedir 2>/dev/null || true)" && \
	if [ -z "$$PG_LIB_DIR" ] || [ ! -d "$$PG_LIB_DIR" ]; then PG_LIB_DIR="/opt/homebrew/opt/libpq/lib"; fi && \
	if [ -z "$$PG_INCLUDE_DIR" ] || [ ! -d "$$PG_INCLUDE_DIR" ]; then PG_INCLUDE_DIR="/opt/homebrew/opt/libpq/include"; fi && \
	export PQ_LIB_DIR="$$PG_LIB_DIR" && \
	export LIBRARY_PATH="$$PG_LIB_DIR:$$LIBRARY_PATH" && \
	export DYLD_LIBRARY_PATH="$$PG_LIB_DIR:$$DYLD_LIBRARY_PATH" && \
	export PKG_CONFIG_PATH="/opt/homebrew/opt/libpq/lib/pkgconfig:$$PKG_CONFIG_PATH" && \
	export C_INCLUDE_PATH="$$PG_INCLUDE_DIR:$$C_INCLUDE_PATH" && \
	cd bin/store && RUST_MIN_STACK=16777216 cargo run

# =============================================================================
# Counter service (gRPC + Redis code generation)
# =============================================================================
# REDIS_URL: connection string for Redis. Set in env or in .env (sourced in targets below).

# Flush Redis using REDIS_URL from env or .env (default redis://127.0.0.1:6379)
redis-flush:
	@REDIS_URL=$${REDIS_URL:-redis://127.0.0.1:6379}; \
	if [ -f .env ]; then set -a && . ./.env && set +a; fi; \
	if [ -f bin/counter-service/.env ]; then set -a && . bin/counter-service/.env && set +a; fi; \
	REDIS_URL=$${REDIS_URL:-redis://127.0.0.1:6379}; \
	echo "🧹 Flushing Redis at $$REDIS_URL..."; \
	if command -v redis-cli >/dev/null 2>&1; then \
		redis-cli -u "$$REDIS_URL" FLUSHALL || { echo "❌ redis-cli failed (is Redis running?)"; exit 1; }; \
		echo "✅ Redis flushed."; \
	else \
		echo "❌ redis-cli not found. Install Redis client (e.g. brew install redis) or run via Docker."; \
		exit 1; \
	fi

# Start the counter-service (REDIS_URL, CODE_SERVICE_GRPC_LISTEN from env or .env)
counter-service:
	@echo "🔢 Starting counter-service..."
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if [ -f "$$HOME/.cargo/env" ]; then . "$$HOME/.cargo/env"; fi; \
	if [ -f .env ]; then set -a && . ./.env && set +a; fi; \
	if [ -f bin/counter-service/.env ]; then set -a && . bin/counter-service/.env && set +a; fi; \
	export REDIS_URL=$${REDIS_URL:-redis://127.0.0.1:6379}; \
	export CODE_SERVICE_GRPC_LISTEN=$${CODE_SERVICE_GRPC_LISTEN:-0.0.0.0:50051}; \
	cd bin/counter-service && cargo run

# Run counter-service unit tests (no Redis required)
counter-service-test:
	@echo "🧪 Running counter-service unit tests..."
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if [ -f "$$HOME/.cargo/env" ]; then . "$$HOME/.cargo/env"; fi; \
	cargo test -p counter-service --lib

# Run counter-service integration tests (requires Redis at REDIS_URL)
counter-service-test-integration: redis-flush
	@REDIS_URL=$${REDIS_URL:-redis://127.0.0.1:6379}; \
	if [ -f .env ]; then set -a && . ./.env && set +a; fi; \
	if [ -f bin/counter-service/.env ]; then set -a && . bin/counter-service/.env && set +a; fi; \
	REDIS_URL=$${REDIS_URL:-redis://127.0.0.1:6379}; \
	export REDIS_URL; \
	echo "🧪 Running counter-service integration tests (Redis at $$REDIS_URL)..."; \
	export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if [ -f "$$HOME/.cargo/env" ]; then . "$$HOME/.cargo/env"; fi; \
	cargo test -p counter-service --test integration -- --ignored

# Flush Redis and run all counter-service tests (unit + integration)
counter-service-test-all: redis-flush counter-service-test counter-service-test-integration
	@echo "✅ All counter-service tests finished."

# =============================================================================
# Store-specific targets
# =============================================================================

# Run the store clean setup
# Run the store clean setup
store-clean-setup:
	@echo "🧹 Starting store clean setup (--init-db)..."
	@export PATH="/usr/local/cargo/bin:/root/.cargo/bin:$$HOME/.cargo/bin:$$PATH"; \
	if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi; \
	PG_LIB_DIR="$$(pg_config --libdir 2>/dev/null || true)"; \
	PG_INCLUDE_DIR="$$(pg_config --includedir 2>/dev/null || true)"; \
	if [ -z "$$PG_LIB_DIR" ] || [ ! -d "$$PG_LIB_DIR" ]; then \
		PG_LIB_DIR="/opt/homebrew/opt/libpq/lib"; \
	fi; \
	if [ -z "$$PG_INCLUDE_DIR" ] || [ ! -d "$$PG_INCLUDE_DIR" ]; then \
		PG_INCLUDE_DIR="/opt/homebrew/opt/libpq/include"; \
	fi; \
	export PQ_LIB_DIR="$$PG_LIB_DIR"; \
	export LIBRARY_PATH="$$PG_LIB_DIR:$$LIBRARY_PATH"; \
	export DYLD_LIBRARY_PATH="$$PG_LIB_DIR:$$DYLD_LIBRARY_PATH"; \
	export PKG_CONFIG_PATH="/opt/homebrew/opt/libpq/lib/pkgconfig:$$PKG_CONFIG_PATH"; \
	export C_INCLUDE_PATH="$$PG_INCLUDE_DIR:$$C_INCLUDE_PATH"; \
	cd bin/store && RUST_LOG=info cargo run -- --cleanup --init-db
 
store-clean-setup-lm:
	@echo "🧹 Starting store clean setup (low memory, --init-db)..."
	@export PATH="/usr/local/cargo/bin:/root/.cargo/bin:$$HOME/.cargo/bin:$$PATH"; \
	if ! command -v cargo >/dev/null 2>&1; then \
		echo "❌ Cargo not found. Please run 'make install' first."; \
		exit 1; \
	fi; \
	PG_LIB_DIR="$$(pg_config --libdir 2>/dev/null || true)"; \
	PG_INCLUDE_DIR="$$(pg_config --includedir 2>/dev/null || true)"; \
	if [ -z "$$PG_LIB_DIR" ] || [ ! -d "$$PG_LIB_DIR" ]; then \
		PG_LIB_DIR="/opt/homebrew/opt/libpq/lib"; \
	fi; \
	if [ -z "$$PG_INCLUDE_DIR" ] || [ ! -d "$$PG_INCLUDE_DIR" ]; then \
		PG_INCLUDE_DIR="/opt/homebrew/opt/libpq/include"; \
	fi; \
	export PQ_LIB_DIR="$$PG_LIB_DIR"; \
	export LIBRARY_PATH="$$PG_LIB_DIR:$$LIBRARY_PATH"; \
	export DYLD_LIBRARY_PATH="$$PG_LIB_DIR:$$DYLD_LIBRARY_PATH"; \
	export PKG_CONFIG_PATH="/opt/homebrew/opt/libpq/lib/pkgconfig:$$PKG_CONFIG_PATH"; \
	export C_INCLUDE_PATH="$$PG_INCLUDE_DIR:$$C_INCLUDE_PATH"; \
	cd bin/store && \
	RUSTFLAGS="-C debuginfo=0 -C codegen-units=1" \
	cargo build -j 1 && \
	RUST_LOG=info ../../target/debug/store --cleanup --init-db
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
	@PG_LIB_DIR="$$(pg_config --libdir 2>/dev/null || true)"; \
	PG_INCLUDE_DIR="$$(pg_config --includedir 2>/dev/null || true)"; \
	if [ -z "$$PG_LIB_DIR" ] || [ ! -d "$$PG_LIB_DIR" ]; then PG_LIB_DIR="/opt/homebrew/opt/libpq/lib"; fi; \
	if [ -z "$$PG_INCLUDE_DIR" ] || [ ! -d "$$PG_INCLUDE_DIR" ]; then PG_INCLUDE_DIR="/opt/homebrew/opt/libpq/include"; fi; \
	export PQ_LIB_DIR="$$PG_LIB_DIR"; \
	export LIBRARY_PATH="$$PG_LIB_DIR:$$LIBRARY_PATH"; \
	export DYLD_LIBRARY_PATH="$$PG_LIB_DIR:$$DYLD_LIBRARY_PATH"; \
	export PKG_CONFIG_PATH="/opt/homebrew/opt/libpq/lib/pkgconfig:$$PKG_CONFIG_PATH"; \
	export C_INCLUDE_PATH="$$PG_INCLUDE_DIR:$$C_INCLUDE_PATH"; \
	cd bin/store && cargo build --release && cp -f ../../target/release/store ../../store

# Build a Linux binary in Docker and place it at repo root as ./store
# Usage: make store-build-linux [LINUX_PLATFORM=linux/amd64|linux/arm64]
LINUX_PLATFORM ?= linux/amd64
store-build-linux:
	@echo "🐧 Building Linux store binary using Docker ($(LINUX_PLATFORM))..."
	@docker pull --platform=$(LINUX_PLATFORM) rust:latest || true
	@docker run --rm --platform=$(LINUX_PLATFORM) \
		-e USER_ID=$$(id -u) -e GROUP_ID=$$(id -g) \
		-v "$$PWD":/usr/src/crdt-workspace \
		-w /usr/src/crdt-workspace \
		rust:latest bash -lc '\
			set -euxo pipefail; \
			export PATH="/usr/local/cargo/bin:/root/.cargo/bin:$$PATH"; \
			apt-get update; \
			DEBIAN_FRONTEND=noninteractive apt-get install -y libpq-dev protobuf-compiler pkg-config make; \
			cd bin/store; \
			cargo --version; \
			cargo build --release; \
			test -f /usr/src/crdt-workspace/target/release/store; \
			cp -f /usr/src/crdt-workspace/target/release/store /usr/src/crdt-workspace/store; \
			chmod +x /usr/src/crdt-workspace/store; \
			chown $$USER_ID:$$GROUP_ID /usr/src/crdt-workspace/store || true \
		'
	@echo "✅ Linux binary available at ./store"

# Build using Docker Buildx and export binary locally
store-build-linux-bx:
	@echo "🐧 Building Linux store binary with buildx ($(LINUX_PLATFORM))..."
	@docker buildx build --platform=$(LINUX_PLATFORM) --pull \
		--target artifact -f dockerfile \
		--output type=local,dest=./dist .
	@mv -f ./dist/store ./store
	@chmod +x ./store
	@rm -rf ./dist
	@echo "✅ Linux binary available at ./store (buildx)"

# Build inside Docker using the Dockerfile and extract the artifact to ./store
BUILD_JOBS ?= 1
BUILD_SWAP_MB ?= 0
OPT_LEVEL ?= 2
CODEGEN_UNITS ?= 1
DOCKER_BUILD_ARGS = --build-arg BUILD_JOBS=$(BUILD_JOBS) --build-arg BUILD_SWAP_MB=$(BUILD_SWAP_MB) --build-arg OPT_LEVEL=$(OPT_LEVEL) --build-arg CODEGEN_UNITS=$(CODEGEN_UNITS)
store-build-docker:
	@echo "🐳 Building store binary inside Docker (artifact stage)..."
	@docker build --target artifact -t store-builder -f dockerfile $(DOCKER_BUILD_ARGS) .
	@cid=$$(docker create store-builder /store); \
	docker cp $$cid:/store ./store; \
	docker rm $$cid >/dev/null
	@chmod +x ./store
	@echo "✅ Linux binary available at ./store (docker build)"

store-build-docker-legacy:
	@echo "🐳 Building store binary with legacy builder (BuildKit disabled)..."
	@DOCKER_BUILDKIT=0 docker build --target artifact -t store-builder-legacy -f dockerfile $(DOCKER_BUILD_ARGS) .
	@cid=$$(docker create store-builder-legacy /store); \
	docker cp $$cid:/store ./store; \
	docker rm $$cid >/dev/null
	@chmod +x ./store
	@echo "✅ Linux binary available at ./store (legacy builder)"

store-build-docker-nocache:
	@echo "🐳 Building store binary without cache (artifact stage)..."
	@docker build --no-cache --pull --target artifact -t store-builder-nocache -f dockerfile $(DOCKER_BUILD_ARGS) .
	@cid=$$(docker create store-builder-nocache /store); \
	docker cp $$cid:/store ./store; \
	docker rm $$cid >/dev/null
	@chmod +x ./store
	@echo "✅ Linux binary available at ./store (no-cache build)"

store-build-docker-memsafe:
	@$(MAKE) store-build-docker BUILD_JOBS=1 CODEGEN_UNITS=1 OPT_LEVEL=0 BUILD_SWAP_MB=4096

store-build-docker-memsafe-legacy:
	@$(MAKE) store-build-docker-legacy BUILD_JOBS=1 CODEGEN_UNITS=1 OPT_LEVEL=0 BUILD_SWAP_MB=4096

store-build-docker-auth:
	@echo "🐳 Building store binary with registry auth via BuildKit secret..."
	@if [ -z "$$DNAMICRO_TOKEN" ]; then \
		echo "❌ DNAMICRO_TOKEN environment variable is required for private registry access."; \
		echo "   Example: export DNAMICRO_TOKEN=..."; \
		exit 1; \
	fi
	@tmpfile=$$(mktemp); \
	trap 'rm -f $$tmpfile' EXIT || true; \
	printf "%s" "$$DNAMICRO_TOKEN" > "$$tmpfile"; \
	docker build --target artifact -t store-builder-auth -f dockerfile \
		--secret id=dnamicro_token,src=$$tmpfile $(DOCKER_BUILD_ARGS) .; \
	rm -f "$$tmpfile"; \
	cid=$$(docker create store-builder-auth /store); \
	docker cp $$cid:/store ./store; \
	docker rm $$cid >/dev/null; \
	chmod +x ./store; \
	echo "✅ Linux binary available at ./store (docker build with auth)"

docker-diagnose:
	@echo "🔎 Docker daemon info:"
	@docker info || true
	@echo ""
	@echo "📦 Docker disk usage:"
	@docker system df || true
	@echo ""
	@echo "🔧 Docker builders:"
	@docker builder ls || true
	@echo ""
	@echo "🔧 Docker buildx instances:"
	@docker buildx ls || true

docker-prune:
	@echo "🧹 Pruning Docker system and builder caches..."
	@docker system prune -f || true
	@docker builder prune -af || true
	@echo "✅ Docker system and builder caches cleaned"

# Clean builder caches and pull fresh base image to fix containerd/blob errors
store-build-linux-clean:
	@echo "🧹 Cleaning Docker builder cache and images..."
	@docker builder prune -af || true
	@docker image rm rust:latest || true
	@docker system prune -f || true
	@docker pull rust:latest || true
	@echo "✅ Docker caches pruned and base image refreshed"

# Cross-compile Linux binary using cargo-zigbuild (no Docker)
# Usage: make store-build-linux-zig [LINUX_TARGET=aarch64-unknown-linux-gnu|x86_64-unknown-linux-gnu]
LINUX_TARGET ?= x86_64-unknown-linux-gnu
store-build-linux-zig:
	@echo "🐧 Cross-compiling Linux binary using cargo-zigbuild (target=$(LINUX_TARGET))..."
	@if ! command -v zig >/dev/null 2>&1; then \
		if [ "$$(uname)" = "Darwin" ]; then \
			if command -v brew >/dev/null 2>&1; then \
				echo "📦 Installing zig via Homebrew..."; \
				brew list zig >/dev/null 2>&1 || brew install zig; \
			else \
				echo "❌ Homebrew not found. Please install Homebrew (https://brew.sh) or install zig manually."; \
				exit 1; \
			fi; \
		else \
			echo "❌ zig not found. Please install zig via your package manager."; \
			exit 1; \
		fi; \
	fi
	@if ! command -v cargo-zigbuild >/dev/null 2>&1; then \
		echo "📦 Installing cargo-zigbuild..."; \
		cargo install cargo-zigbuild; \
	fi
	@rustup target add $(LINUX_TARGET) || true
	@cd bin/store && cargo zigbuild --release --target $(LINUX_TARGET)
	@cp -f bin/store/target/$(LINUX_TARGET)/release/store ./store
	@chmod +x ./store
	@echo "✅ Linux binary available at ./store (zigbuild)"

# Convenience targets for Debian-compatible builds
store-build-debian-amd64:
	@$(MAKE) store-build-linux-zig LINUX_TARGET=x86_64-unknown-linux-gnu

store-build-debian-arm64:
	@$(MAKE) store-build-linux-zig LINUX_TARGET=aarch64-unknown-linux-gnu

# Run the compiled store binary directly (no cargo)
store-prod:
	@echo "▶️  Running compiled store binary..."
	@if [ ! -x "./target/release/store" ]; then \
		echo "🔨 Building store (release) first..."; \
		cd bin/store && cargo build --release || exit 1; \
	fi
	@os=$$(uname); \
	case "$$os" in \
		Darwin) out="store-mac-os" ;; \
		Linux) out="store-linux" ;; \
		MINGW*|MSYS*|CYGWIN*) out="store-windows.exe" ;; \
		*) out="store-unknown" ;; \
	esac; \
	cp -f ./target/release/store ./$$out && chmod +x ./$$out || true; \
	echo "✅ Binary copied to ./$$out"; \
	if [ "$$os" = "Darwin" ]; then \
		echo "🔐 Fixing macOS security attributes..."; \
		xattr -d com.apple.quarantine ./$$out 2>/dev/null || true; \
		codesign -s - ./$$out 2>/dev/null || true; \
	fi; \
	./$$out

# Build store binary without running it (useful for permission fixes)
store-build-only:
	@echo "🔨 Building store binary..."
	@cd bin/store && cargo build --release
	@os=$$(uname); \
	case "$$os" in \
		Darwin) out="store-mac-os" ;; \
		Linux) out="store-linux" ;; \
		MINGW*|MSYS*|CYGWIN*) out="store-windows.exe" ;; \
		*) out="store-unknown" ;; \
	esac; \
	cp -f ./target/release/store ./$$out && chmod +x ./$$out; \
	echo "✅ Binary built and copied to ./$$out"; \
	if [ "$$os" = "Darwin" ]; then \
		echo "🔐 Fixing macOS security attributes..."; \
		xattr -d com.apple.quarantine ./$$out 2>/dev/null || true; \
		codesign -s - ./$$out 2>/dev/null || true; \
	fi

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

# Standalone store-generator (no store build dependency - use for fresh clone bootstrap)
STORE_GEN = store-generator
store-generator-schema:
	@printf "Checkpoint name (timestamp appended): " && read name && \
	timestamp=$$(date -u +%Y%m%d-%H%M%S) && \
	fullname="$${name}_$${timestamp}" && \
	gen_files=$$(find bin/store/src/generated -type f \( -name '*.rs' -o -name '*.proto' \) 2>/dev/null) && \
	mig_files=$$(find bin/store/migrations -type f -name '*.sql' 2>/dev/null) && \
	all_files=$$(echo "$$gen_files" "$$mig_files" | tr ' \n' '\n' | grep -v '^$$' | tr '\n' ',' | sed 's/,$$//') && \
	CKPT_CMD=$$(command -v ckpt 2>/dev/null || echo "cargo run -p ckpt --release --"); \
	$$CKPT_CMD init 2>/dev/null || true; \
	if [ -n "$$all_files" ]; then \
		$$CKPT_CMD save --name "$$fullname" --files "$$all_files" && \
		echo "✅ Checkpoint saved: $$fullname"; \
	else \
		echo "⚠️  No store generated files found to checkpoint."; \
	fi && \
	echo "📋 Generating store schema (standalone generator)..." && \
	STORE_DIR=bin/store CREATE_SCHEMA=true $(STORE_GEN)

store-generator-proto:
	@printf "Checkpoint name (timestamp appended): " && read name && \
	timestamp=$$(date -u +%Y%m%d-%H%M%S) && \
	fullname="$${name}_$${timestamp}" && \
	gen_files=$$(find bin/store/src/generated -type f \( -name '*.rs' -o -name '*.proto' \) 2>/dev/null) && \
	mig_files=$$(find bin/store/migrations -type f -name '*.sql' 2>/dev/null) && \
	all_files=$$(echo "$$gen_files" "$$mig_files" | tr ' \n' '\n' | grep -v '^$$' | tr '\n' ',' | sed 's/,$$//') && \
	CKPT_CMD=$$(command -v ckpt 2>/dev/null || echo "cargo run -p ckpt --release --"); \
	$$CKPT_CMD init 2>/dev/null || true; \
	if [ -n "$$all_files" ]; then \
		$$CKPT_CMD save --name "$$fullname" --files "$$all_files" && \
		echo "✅ Checkpoint saved: $$fullname"; \
	else \
		echo "⚠️  No store generated files found to checkpoint."; \
	fi && \
	echo "🔧 Generating store proto (standalone generator)..." && \
	STORE_DIR=bin/store GENERATE_PROTO=true GENERATE_GRPC=true GENERATE_TABLE_ENUM=true $(STORE_GEN)

store-generator-all:
	@echo "🔧 Generating all store files (standalone generator)..."
	@STORE_DIR=bin/store CREATE_SCHEMA=true GENERATE_PROTO=true GENERATE_GRPC=true GENERATE_TABLE_ENUM=true $(STORE_GEN)

remove-schema-table-macros:
	@echo "🗑️  Removing orphaned table macros from schema.rs based on migration files..."
	@cd bin/store && mkdir -p target && rustc src/script/remove_orphaned_schema_macros.rs -o target/remove_orphaned_schema_macros && ./target/remove_orphaned_schema_macros

# Run the store update_counters binary
update-counters:
	@echo "🔄  Updating counters..."
	@cd bin/store && cargo run --bin update_counters -- $(ARGS)

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
	@export PATH="/usr/local/cargo/bin:/root/.cargo/bin:$$HOME/.cargo/bin:$$PATH"; \
	if ! command -v diesel >/dev/null 2>&1; then \
		echo "❌ Diesel CLI not found. Please run 'make install' first."; \
		exit 1; \
	fi; \
	cd bin/store && diesel migration run

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

# Setup git hooks (uses core.hooksPath → versioned scripts/)
setup-hooks:
	@echo "🪝 Setting up git hooks..."
	@./scripts/setup-hooks.sh

# Idempotent: ensure hooks are configured (runs automatically in dev/install)
ensure-hooks: setup-hooks

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
DOCKER_COMPOSE_FILE := docker-compose-template.yml
ENV_STORE_FILE := .env-store
ENV_SYNC_SERVER_FILE := .env-sync-server

# Start TimescaleDB and Redis services using Docker Compose
docker-compose-up:
	@echo "🐳 Starting Docker Compose services..."
	@if [ ! -f "$(DOCKER_COMPOSE_FILE)" ]; then \
		echo "❌ Error: Docker Compose file not found at $(DOCKER_COMPOSE_FILE)"; \
		echo "Please ensure $(DOCKER_COMPOSE_FILE) exists in the project root."; \
		exit 1; \
	fi
	@if [ ! -f "$(ENV_STORE_FILE)" ]; then \
		echo "❌ Error: Env file not found at $(ENV_STORE_FILE)"; \
		exit 1; \
	fi
	@if [ ! -f "$(ENV_SYNC_SERVER_FILE)" ]; then \
		echo "❌ Error: Env file not found at $(ENV_SYNC_SERVER_FILE)"; \
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

# Publish store-generator to dnamicro registry
publish-store-generator:
	@echo "📦 Publishing store-generator to 'dnamicro' registry..."
	@cd bin/store-generator && cargo publish --registry dnamicro


code-eval: 
	@echo "Running code evaluation..."
	@cd bin/store && cargo clippy

code-eval-fix:
	@echo "Running code evaluation fix..."
	@cd bin/store && cargo clippy --fix

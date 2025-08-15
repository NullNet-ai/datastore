# CRDT Rust Implementation

A Rust implementation of Conflict-Free Replicated Data Types (CRDTs) using Merkle trees and Hybrid Logical Clocks.

## Project Structure

```plaintext
crdt-rust/
├── libs/
│   ├── hlc/        # Hybrid Logical Clock implementation
│   └── merkle/     # Merkle Tree implementation
├── bin/
│   └── main/       # Main executable
```

## Quick Setup (One Command)

🚀 **New developers can set up the entire project with just one command:**

```bash
make install
```

This will automatically install:
- Rust (if not already installed)
- PostgreSQL
- cargo-make, cargo-watch, diesel_cli
- Set up git hooks
- Configure the development environment

### Verify Installation

After installation, verify everything is working:

```bash
make verify-install
```

### Start Development

Once installed, start the development servers:

```bash
make dev
```

## Manual Prerequisites (if not using installer)

- Rust (latest stable version)
- PostgreSQL
- cargo-make: `cargo install cargo-make`
- cargo-watch: `cargo install cargo-watch`
- diesel_cli: `cargo install diesel_cli --no-default-features --features postgres`

## Getting Started (Manual Setup)

  1. Clone the repository:

      ```bash
      git clone https://github.com/yourusername/crdt-rust.git
      cd crdt-rust
      ```

  2. Install dependencies:

      ```bash
      make install
      ```

  3. Start development:

      ```bash
      make dev
      ```

## Available Make Commands

The project includes a comprehensive Makefile with the following commands:

### Setup & Installation
- `make install` - One-command installer for all dependencies (auto-detects macOS/Linux)
- `make install-macos` - Install dependencies specifically for macOS
- `make install-linux` - Install dependencies specifically for Linux
- `make verify-install` - Verify that all required tools are installed
- `make setup-hooks` - Setup git hooks for code quality

### Development
- `make dev` - Run both server and store in parallel
- `make server` - Run the server only
- `make store` - Run the store only
- `make store-watch` - Run store in watch mode with debug
- `make store-build` - Build store in release mode
- `make store-clean-setup` - Run store clean setup
- `make store-initialize-device` - Initialize device and wait for PostgreSQL listener

### Store Operations
- `make store-generate-schema` - Generate store schema
- `make store-generate-proto` - Generate store proto files
- `make store-experimental` - Run experimental features
- `make jean-store-watch` - Run store in watch mode with PostgreSQL library configurations

### Database Management
- `make db-migrate-generate NAME=migration_name` - Generate new migration
- `make db-migrate-up` - Run database migrations
- `make db-migrate-revert` - Revert last migration

### Code Quality
- `make fmt` - Format Rust code across all projects
- `make fmt-check` - Check code formatting across all projects

### Git & Version Control
- `make git-cleanup` - Clean up local branches that no longer exist on remote

### Utilities
- `make clean` - Clean build artifacts
- `make help` - Show all available commands

For a complete list of commands, run:
```bash
make help
```

## Development Workflow

  1. Create a new branch for your feature or bug fix:

      ```bash
      git checkout -b feature/feature-name
      ```

  2. Make your changes in the appropriate library:

     - HLC changes go in libs/hlc
     - Merkle Tree changes go in libs/merkle
     - Main application changes go in bin/main

  3. Format your code:

      ```bash
      cargo fmt
      ```

## Contributing Guidelines

  1. Follow Rust's coding conventions
  2. Write clear commit messages
  3. Include tests for new functionality
  4. Update documentation as needed
  5. Make sure all tests pass before submitting PR


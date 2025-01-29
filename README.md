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

## Prerequisites

- Rust (latest stable version)
- Protocol Buffers compiler
- Cargo (Rust's package manager)

## Getting Started

  1. Clone the repository:

      ```bash
      git clone https://github.com/yourusername/crdt-rust.git
      cd crdt-rust
      ```

  2. Build the project:. Build the project:

      ```bash
      cargo build
      ```

  3. Run the main executable:. Run the main executable:

      ```bash
      cargo run
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


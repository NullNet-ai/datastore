# Schema Generator with Hypertable Support

The Schema Generator automatically creates Rust models, updates `schema.rs`, and generates database migrations based on table definition files. It provides a Diesel-based schema generator with built-in support for TimescaleDB hypertables, comprehensive validation, and supports both comment-based definitions and modern struct-based definitions using actual Diesel types.

## Features

- **Hypertable Support**: Define time-series tables with automatic validation
- **Compile-time Validation**: Ensures hypertable requirements are met
- **Migration Safety**: Prevents duplicate index and foreign key creation
- **Type Safety**: Full Rust type system integration
- **Smart Re-runs**: Only generates new migrations when actual schema changes are detected
- **Automatic Generation**: Creates Rust models, schema definitions, and migrations

## How it Works

1. **Table Definitions**: Create `.rs` files in `src/database/schema/tables/` with table field definitions
2. **Automatic Generation**: Run with `CREATE_SCHEMA=true cargo run` to generate:
   - Rust model structs in `src/database/models/`
   - Updated `schema.rs` with new table definitions
   - Database migration files in `migrations/`
3. **Smart Re-runs**: Only generates new migrations when actual schema changes are detected

## System Fields Macros

This folder contains `system_fields!()`, `system_indexes!()`, and `system_foreign_keys!()` macros used in table definitions. The store-generator reads these when parsing table files.

## Integration

The schema generator runs via store-generator. Use: `make store-generator-all` or `STORE_DIR=bin/store CREATE_SCHEMA=true cargo run -p store-generator`.

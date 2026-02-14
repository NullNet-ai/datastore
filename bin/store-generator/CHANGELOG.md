# Changelog

All notable changes to the store-generator crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.6
### Author
Kashan

### Added
- `authors` field in `Cargo.toml`.

### Fixed
- `test_reads_from_store_file` path resolution when run from store-generator directory.
- Added `../store` candidate and guard to ensure store's reserved_keywords.rs is read, not store-generator's own file.

## 0.1.5
### Author
Kashan

### Added
- **Reserved keywords from store**:
  - `reserved_keywords` now reads from the store's `reserved_keywords.rs` file at runtime (same pattern as `system_tables`).
  - Added `RESERVED_KEYWORDS_FILE` to paths.
  - Parses the store's `RESERVED_KEYWORDS` array; no longer duplicated in store-generator.
- **Tests for reserved_keywords**:
  - `test_parse_reserved_keywords_from_rust` – parses array with multiple words (columns, box, type, ref).
  - `test_parse_empty_array`, `test_parse_malformed_returns_empty`.
  - `test_reads_from_store_file` – integration test that reads store's file and asserts columns/box are present.

## 0.1.4
### Author
Kashan

### Changed
- Published to dnamicro registry.

## 0.1.3
### Author
Kashan

### Added
- **Table validation**:
  - Table names must be plural (or uncountable).
  - File names must match table names.
  - Index names must follow `idx_{table}_{column}`.
  - Foreign key names must follow `fk_{table}_{column}`.
- **Uncountable words**:
  - Maintained list of uncountable words (progress, information, data, etc.) for plural validation.
  - Tables like `progress` and `user_progress` pass validation.
- Comprehensive tests in `table_validator.rs`.

## 0.1.2
### Author
Kashan

### Changed
- Store-generator refactored into standalone crate.
- Makefile targets: `store-generator-schema`, `store-generator-proto`, `store-generator-all`.
- Configuration via `store-generator.toml`, `STORE_DIR`, and other env vars.
- Reads `system_tables`, `system_fields`, and other store files from disk.

## 0.1.1
### Author
Kashan

### Added
- Initial standalone store-generator crate.
- Code-generation builders: schema, migration, model, proto, table_enum, grpc_controller.
- Extracted from store crate (store 0.2.21).
- Publish support for dnamicro registry.

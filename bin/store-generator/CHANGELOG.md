# Changelog

All notable changes to the store-generator crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.11
### Author
Kashan

### Added
- **Migration up.sql order for new hypertables**: For each new table, up SQL is now emitted in order: CREATE TABLE → create_hypertable (if hypertable) → CREATE INDEX for that table, so indexes are created on the hypertable. Comment "Convert to hypertable before creating indexes" added before create_hypertable.
- Test: `test_up_sql_hypertable_indexes_after_create_hypertable` – asserts CREATE TABLE, create_hypertable, and CREATE INDEX order and comment presence.

## 0.1.10
### Author
Kashan

### Added
- **JSONB default validation**:
  - JSONB columns with a default must use format including `::jsonb` (e.g. `default: "'[]'::jsonb"` or `default: "'{\"k\":1}'::jsonb"`).
  - Empty-array default (`'[]'::jsonb`) is rejected; omit default (array is empty by default when no value).
  - `validate_jsonb_default()` in `table_validator`; invoked during field extraction in `generator_service`.
  - Tests: `test_validate_jsonb_default_ok_no_default`, `test_validate_jsonb_default_ok_with_cast`, `test_validate_jsonb_default_err_missing_cast`, `test_validate_jsonb_default_err_empty_array_default`, `test_validate_jsonb_default_err_empty_array_no_quotes`.

## 0.1.9
### Author
Kashan

### Added
- **Duplicate validation (aborts on error)**:
  - `validate_no_duplicate_fields`: Errors if a field name appears twice or user redefines a system field (organization_id, created_by, tags, etc.).
  - `validate_no_duplicate_index_names`: Errors if an index name appears twice (e.g. user redefines an index from system_indexes!).
  - `validate_no_duplicate_foreign_keys`: Errors if an FK constraint name appears twice or user adds an FK for a column already in system_foreign_keys! (organization_id, created_by, updated_by, deleted_by, requested_by).
- `validate_table_file` now accepts `field_names` and runs all duplicate checks before format validation.
- Tests: `test_validate_no_duplicate_fields`, `test_validate_no_duplicate_index_names`, `test_validate_no_duplicate_foreign_keys`.

### Changed
- Duplicate field handling in `extract_fields_from_macro`: now returns `Err` and aborts instead of replacing or skipping.

## 0.1.8
### Author
Kashan

### Fixed
- **Validation failure abort**: When any table fails validation (e.g. invalid index name), discovery now returns `Err` and aborts immediately instead of continuing with other tables and creating migrations.
- **Index type in migrations**: `USING btree" }("column")` regression – migration generator now sanitizes `index_type` to strip stray `" }` from malformed extraction. `extract_quoted_value_after` also trims trailing ` }`, `},`.

### Added
- `test_validation_failure_aborts_discovery` – ensures discovery aborts on validation failure.
- `test_organizations_style_index_sql_no_invalid_chars` – migration regression test for malformed index type.
- `test_extract_indexes_organizations_style_single_line` – extraction regression for organizations-style single-line indexes.

## 0.1.7
### Author
Kashan

### Fixed
- **Index/foreign key extraction – format-agnostic parsing**:
  - Single-line index format `type: "btree" }` no longer captures ` }` in the type value; generates valid SQL.
  - Added `extract_quoted_value_after()` and `extract_bracketed_quoted_values()` helpers for content-based parsing.
  - Index and foreign key extraction now works regardless of formatting (single-line, multi-line, extra spacing, trailing commas).
- Foreign key column parsing when both `columns:` and `foreign_columns:` appear on the same line.

### Added
- Tests for indexes: single-line, multi-line, multiple formats, extra spacing.
- Tests for foreign keys: single-line, multi-line, multiple formats.
- `test_postgres_channels_index_sql_no_invalid_chars` migration regression test.

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

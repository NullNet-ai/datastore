# Changelog

All notable changes to the store-generator crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.15
### Author
Kashan

### Changed
- **JSONB default validation**: `validate_jsonb_default` in `table_validator.rs` now allows empty-array default `'[]'::jsonb`. JSONB columns (e.g. `allowed_grades`, `allowed_ages`) may use `default: "'[]'::jsonb"` in table definitions; the previous rule that rejected empty-array defaults has been removed. Validation still requires the `::jsonb` cast when a default is set.

## 0.1.14
### Author
Kashan

### Fixed
- **Index definition comparison vs existing migrations**: When an index already exists in migrations, the generator compares the table definition’s index SQL with the existing migration SQL. Existing migrations sometimes use unquoted column names (e.g. `btree(tombstone)`) while the generator emits quoted names (e.g. `btree("tombstone")`). In `schema_generator.rs`, `normalize_index_sql_for_compare` now normalizes the column list inside `USING btree(...)` to a single form (unquoted), so both styles compare equal and the generator no longer errors with "Index '...' was already created with a different definition" when the only difference is column quoting.

## 0.1.13
### Author
Kashan

### Added
- **Model generator: timestamp/timestamptz types and chrono imports**:
  - When any field uses `timestamp()` or `timestamptz()`, generated models now include `use chrono::{DateTime, NaiveDateTime, Utc};` and use short type names in the struct (`DateTime<Utc>`, `NaiveDateTime`) instead of fully qualified paths. `RUST_TYPE_MAPPINGS` already mapped `Timestamptz` → `chrono::DateTime<chrono::Utc>` and `Timestamp` → `chrono::NaiveDateTime`.
  - New helpers: `ModelGenerator::chrono_import_line()`, `ModelGenerator::rust_type_for_display()`.
  - Unit tests in `utils.rs` for `parse_diesel_type` and `diesel_to_rust_type` (timestamp, timestamptz, nullable variants); unit tests in `model_generator.rs` for chrono import and type shortening.
- **Index validation: index and where-clause columns must exist in table**:
  - `WhereExpr::column_names()` in `diesel_schema_definition.rs` collects all column names referenced in a partial index WHERE expression.
  - `validate_table_file` in `table_validator.rs` now checks that every index column and every column referenced in an index WHERE clause exists in the table’s field list; otherwise it returns a clear error (e.g. "Index '...' where clause references column 'status' which is not found in table '...'").
  - Unit tests: `test_validate_index_with_where_clause_columns_found`, `test_validate_index_with_where_clause_column_not_found`, `test_validate_index_column_not_found`; `test_where_expr_column_names_*` in diesel_schema_definition; `test_extract_index_with_where_clause_school_admins_style` in generator_service.

### Changed
- **Makefile**: `STORE_GEN` now uses `cargo run -p store-generator --` so `store-generator-schema`, `store-generator-proto`, and `store-generator-all` run the local workspace package instead of a `store-generator` binary on PATH.

## 0.1.12
### Author
Kashan

### Added
- **Partial indexes (WHERE clause)**:
  - Index definitions support an optional `where` clause for PostgreSQL partial indexes. Use the idiomatic block format with unquoted keys (e.g. `where: { and: [ { op: "=", column: "status", value: "Active" }, { op: "=", column: "name", value: "John Doe" } ] }`).
  - Supported expression shapes: single predicate `{ op, column, value }`, `and: [ ... ]`, `or: [ ... ]`, `not: { ... }`. Supported ops: `=`, `!=`, `<`, `<=`, `>`, `>=`, `IN`, `NOT IN`, `LIKE`, `ILIKE`, `IS`, `IS NOT`. Values can be strings, numbers, booleans, `null`, or arrays for `IN`/`NOT IN`.
  - Migrations emit `CREATE [UNIQUE] INDEX ... ON table USING type (columns) WHERE <predicate>;` when a where clause is present.
  - **Existing index check**: If an index already exists in migrations but the table definition (columns, type, unique, where) differs, the generator errors and shows the existing SQL; existing indexes cannot be modified.
  - Parser accepts both block format and legacy JSON string format (`where: r#"{"op":"=",...}"#` or `where: "..."`). Index block keys `where` and `not` are not treated as new index names; only lines starting with `idx_` begin a new index.
  - Documentation and full sample table (`index_demos`) with 12 index scenarios in `src/builders/generator/README.md` and reference in root README.

### Fixed
- **Fields section parsing (schema/migrations only had id)**: The end of the `fields: { ... }` block was found by counting `{` and `}` without skipping braces inside string literals. If any default or value contained `'{}'`, `"}"`, etc., the parser could close the section too early or include the indexes section, leading to missing fields and schema/migrations with only the `id` column. Brace counting in `extract_fields_from_macro` now ignores characters inside single- and double-quoted strings (with escape handling).

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

# Changelog

All notable changes to the CRDT Store project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.5

### Author
Kashan

### Added
- Added automatic periodic channel refresh functionality to `PgListenerService`
  - Implemented 30-second interval refresh cycle for PostgreSQL channels
  - Added background task that automatically discovers new channels from `postgres_channels` table
  - Enhanced channel management with smart refresh logic that respects service state (running/paused)

### Enhanced
- **Channel Discovery**: Channels are now automatically refreshed every 30 seconds without requiring service restart
- **Resource Efficiency**: Periodic refresh only runs when service is active and not paused
- **Error Resilience**: Individual refresh failures don't stop the refresh cycle, with comprehensive error logging

### Technical Details
- **New Functionality**: 
  - Added `tokio::time::interval` import for periodic task scheduling
  - Implemented background task in `PgListenerService::new()` method
  - Smart first-tick skip to avoid immediate refresh after startup
  - Enhanced CRDT sync service with `sync_status` field management for connections table
  - Added automatic `sync_status` field insertion for insert and update operations
  - Implemented PostgreSQL trigger support with conditional execution based on `sync_status` values
- **Performance**: 
  - Non-blocking periodic refresh that doesn't interfere with notification processing
  - Conditional refresh based on service state to minimize unnecessary database queries
- **Reliability**: 
  - Automatic synchronization with database changes
  - Robust error handling with detailed logging
  - Enhanced error handling in message service with proper logging and error propagation
- **CRDT Synchronization**:
  - Modified `sync_service.rs` to automatically add `sync_status` field to connections table records
  - Insert operations: `sync_status` set to "complete" and positioned as last field
  - Update operations: `sync_status` set to "consumed" and positioned as first field
  - Enhanced `message_service.rs` with proper error handling for HLC timestamp generation
  - Added support for PostgreSQL triggers with `AFTER INSERT OR UPDATE` and conditional execution
  - Trigger conditions: `WHEN (NEW.sync_status = 'complete')` for targeted execution
- **Code Safety**: Removed unsafe unwraps from the code

---

## 0.1.4

### Author
Kashan

### Added
- Added comprehensive PostgreSQL function management system
  - Created `PgFunctionService` for managing PostgreSQL function operations
  - Implemented `FunctionValidator` with security and syntax validation
  - Added endpoint `POST /api/listener/function` for creating PostgreSQL functions with triggers
  - Added endpoint `GET /api/listener/{function_name}` for retrieving function and trigger information
  - Added endpoint `DELETE /api/listener/{function_name}` for removing functions and triggers
  - Added endpoint `POST /api/listener/test` for testing function syntax without creation
  - Comprehensive validation including balanced parentheses/quotes, dangerous command detection, and database syntax testing

### Enhanced
- **Function Creation**: Enhanced to extract channel names from function strings and store them properly in `postgres_channels` table
- **Function Retrieval**: Implemented querying of PostgreSQL system tables (`pg_proc`, `information_schema.triggers`) to retrieve function definitions and trigger information
- **Function Deletion**: Added transactional deletion ensuring atomicity across multiple operations (database record deletion, function dropping, trigger dropping)
- **Error Handling**: Comprehensive error handling with detailed success/error messages for all operations

### Technical Details
- **New Files**: 
  - `/src/controllers/pg_functions/function_validators.rs` - Validation logic
  - Updated `/src/controllers/pg_functions/pg_listener_controller.rs` - Controller implementation
- **Database Operations**:
  - Function string validation with regex-based extraction
  - Channel name consistency validation and extraction
  - Transaction-based syntax testing with rollback
  - Transactional deletion with automatic rollback on failure
  - Security checks for dangerous SQL commands
  - Raw PostgreSQL connection usage for dynamic DDL operations
- **API Endpoints**:
  - `POST /api/listener/function` - Create function with trigger
  - `GET /api/listener/{function_name}` - Retrieve function and trigger details
  - `DELETE /api/listener/{function_name}?table_name={table_name}` - Delete function and trigger
  - `POST /api/listener/test` - Test function syntax
- **Data Models**: Added `FunctionRow` and `TriggerRow` structs for PostgreSQL system table queries

## 0.1.3

### Author
Chaosumaru

### Fixed
- Fixed PostgreSQL connection issues in containerized environments
- Resolved issue with missing `POSTGRES_x` environment variable in Dockerized deployments
### Changed
- Updated database connection initialization to prioritize environment variables
- Modified connection string parsing to handle various PostgreSQL URL formats
- Enhanced error handling for database connection failures

### Technical Details
- **Environment Variables**: Enhanced support for `DATABASE_URL`, `POSTGRES_HOST`, `POSTGRES_PORT`, `POSTGRES_DB`, `POSTGRES_USER`, `POSTGRES_PASSWORD`

---

## 0.1.2

### Author
Eriberto

### Added
- Enhanced Makefile with database migration targets
  - Added `db-migrate-generate` target with parameter support for creating named migrations
  - Added `db-migrate-run` target for running pending migrations
  - Added `db-migrate-revert` target for reverting last migration
- Improved Diesel configuration for better portability
  - Updated `diesel.toml` to use relative paths instead of hardcoded absolute paths
  - Enhanced migration directory configuration for cross-platform compatibility

### Changed
- Modified diesel.toml migration directory path from absolute to relative path
- Updated Makefile database targets to use consistent working directory approach

### Technical Details
- **Makefile**: Enhanced with comprehensive database operation targets
- **Diesel Config**: `/bin/store/diesel.toml` - Updated for dynamic path resolution
- **Usage**: `make db-migrate-generate NAME=migration_name` for parameterized migration generation

---

## 0.1.1

### Author
Kashan

### Added
- Added `sensitivity_level` column to the application tables table in migration `2025-06-11-230646_initial_migration`
  - Column definition: `"sensitivity_level" integer DEFAULT 1000`
  - Added corresponding btree index: `table_sensitivity_level_idx`

### Fixed
- Fixed missing `sensitivity_level` column in application tables table to match schema definition
- Ensured consistency between database schema and migration files

### Technical Details
- **Migration File**: `/bin/store/migrations/2025-06-11-230646_initial_migration/up.sql`
- **Schema File**: `/bin/store/src/schema.rs`

---

## Notes

This changelog tracks changes made to ensure database schema consistency across the CRDT Store project.
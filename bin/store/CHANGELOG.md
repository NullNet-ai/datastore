# Changelog

All notable changes to the CRDT Store project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.7

### Author
Kashan

### Added
- **CRDT Messages Performance Indexes**: Added comprehensive database indexes for `crdt_messages` table to significantly improve query performance
  - Individual indexes on `dataset`, `column`, `row`, and `timestamp` columns
  - Composite index on `(dataset, column, row)` for optimizing `find_existing_messages` function
  - Composite index on `(dataset, column, row, timestamp DESC)` for ordered queries
  - Timestamp descending index for `get_messages_since` function optimization

### Enhanced
- **Database Performance**: Dramatically improved `apply_messages` function performance by addressing database query bottlenecks
- **Query Optimization**: Enhanced CRDT message comparison and retrieval operations with targeted indexing strategy

### Technical Details
- **Migration**: Updated initial migration file with new indexes using `IF NOT EXISTS` for safe deployment
- **Index Strategy**: Implemented both single-column and composite indexes to cover various query patterns in CRDT synchronization
- **Performance Impact**: Resolved performance issues where `apply_messages` was taking excessive time due to unindexed database queries

---

## 0.1.6

### Author
Eriberto
### Added
- Added mulitple sorting capabilities
- Added group advance filters
- Added has no value capabilities
- Added match pattern capabilities
- Added custom match pattern capabilities
### Changed
- Changed the way the group advance filters are parsed to be more flexible and allow for more complex filters
- Changed the way the sorting is parsed to be more flexible and allow for more complex sorting
- Change the way the concatenate fields are parsed to be more flexible and allow for more complex concatenation
---



## 0.1.6

### Author
Eriberto
### Added
- Implemented lifetime parameters ('a) and mutability for Validation and SQLConstructor structs to ensure proper memory management and data modification capabilities
- Added temporary slower approach, creating DynamicQueryResult struct to handle dynamic query results
- Added separate construct for the following:
  - selections
  - joins
  - where clauses
  - order by
  - group by
  - offset
  - limit
- Added join selections
- Added concatenation selections
- Added date format wrapper
- Added pluck group object as a string - and group by id will be set automatically due to aggregation.
  
### Changed
- Revise get_by_filter validations to make it more restrictive and are separate as a module
### Fixed
- fix Structs for Advance Filter > type "Criteria" or "Operator"
- ignore all valid warnings for unfinished features


## 0.1.5

### Author
Kashan

### Added
- **Message Streaming System**: Comprehensive real-time message streaming architecture
  - `MessageStreamingService` for message routing and channel operations
  - `TokenBucket` system for rate limiting and backpressure management
  - `StreamQueueService` for persistent message queuing during backpressure
  - Socket.IO gateway with JWT authentication and organization-based client management
  - Real-time dashboard for monitoring token buckets and message flow
- **Automatic Channel Management**: 
  - 30-second interval refresh cycle for PostgreSQL channels in `PgListenerService`
  - Background task for automatic channel discovery from `postgres_channels` table
  - Smart refresh logic respecting service state (running/paused)
- **Enhanced Batch Processing System**: Introduced `is_batch` system field for optimized operations
  - Added `is_batch` boolean column to `connections` and `temp_connections` tables
  - Enhanced batch operations to automatically set `is_batch` to `true`
  - Added automatic `is_batch` field insertion during batch operations in `common_controller.rs`
  - Enhanced batch records to include `sync_status` field set to "complete" for proper sync handling
  - Implemented conditional sync_status value assignment in `message_service.rs` based on `is_batch` flag
  - Modified sync service to skip `sync_status` processing for batch records
  - **CRITICAL**: Prevents duplicate streaming records by setting proper sync_status values

### Removed
- **Architecture Cleanup**: Eliminated redundant local memory queues
  - Removed `BrokerService` and local memory queue management
  - Simplified architecture to use only database queues for backpressure handling
  - Eliminated duplicate message routing logic and complex queue cleanup tasks

### Enhanced
- **Real-time Communication**: Organization-based client authentication, automatic channel creation, WebSocket broadcasting, JWT validation
- **Performance Optimizations**: 
  - Increased message processing batch size from 100 to 500 messages
  - Implemented fair database access with automatic re-queuing
  - Enhanced backpressure handling with refined message deletion logic
  - FIFO queue management using `tokio::sync::Semaphore`
- **CRDT Synchronization**: 
  - Automatic `sync_status` field management for connections table
  - PostgreSQL trigger support with conditional execution
  - Enhanced error handling and HLC timestamp generation

### Technical Implementation
- **Core Files**: 
  - `/src/message_stream/streaming_service.rs` - Message routing and channel management
  - `/src/message_stream/token_bucket.rs` - Rate limiting and backpressure control
  - `/src/message_stream/stream_queue_service.rs` - Persistent message queuing
  - `/src/message_stream/gateway.rs` - Socket.IO gateway with JWT authentication
- **Database**: Added `stream_queues` and `stream_queue_items` tables, `is_batch` column migration
- **Security**: JWT-based authentication, organization-based filtering, secure token validation
- **Monitoring**: Real-time dashboard with dynamic token bucket capacity adjustment
- **Batch Operation Enhancement**: Updated batch processing logic in `common_controller.rs`:
  ```rust
  if let Some(obj) = request_body.record.as_object_mut() {
      obj.insert("is_batch".to_string(), serde_json::Value::Bool(true));
      obj.insert("sync_status".to_string(), serde_json::Value::String("complete".to_string()));
  }
  ```
- **Conditional Sync Status**: Modified `message_service.rs` to set sync_status to "consumed" for batch operations (`is_batch = true`) and "complete" for regular operations
- **Race Condition Fix**: Enhanced `set_tokens` function in `token_bucket.rs` to validate shared state before triggering drain notifications, preventing simultaneous processing paths during capacity changes

### Fixed
- **Race Condition in Message Streaming**: Resolved duplicate record issue during high watermark changes
  - Fixed race condition in `token_bucket.rs` where capacity increases could trigger simultaneous message processing
  - Enhanced state coordination between token bucket and shared streaming state
  - Implemented atomic state transitions to prevent duplicate message processing during backpressure recovery
- Removed infinite loop test data senders causing performance issues
- Updated message flow to only create channels for authenticated organizations
- Simplified client connection flow by removing manual subscription requirements

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

---

## 0.1.3

### Author
Eriberto

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
# Changelog

All notable changes to the CRDT Store project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.13

### Author
Kashan

### Fixed
- **AggregationFilter Parameter Handling**: Fixed aggregation filter method to correctly extract table name from `request.body.entity` instead of non-existent `params.table`
  - Updated `generate_aggregation_filter_method` macro in `grpc_macros.rs` to use `request.body.entity` for table extraction
  
- **Process Record Method Signatures**: Updated all `process_record` method calls to include required `table` parameter
  - Modified `process_record` signature in `structs.rs` to accept `table: &str` parameter
  - Updated calls in `store_controller.rs`, `grpc_macros.rs`, and `table_enum_macros.rs` to pass table name

### Enhanced
- **Organization ID Handling**: Implemented conditional organization_id assignment during update operations
  - Added `forbidden_tables` module with `FORBIDDEN_TABLES` constant and `is_forbidden_table()` function
  - Enhanced `add_common_fields` method to conditionally set `organization_id` based on:
    - Existing `organization_id` presence in request body
    - `is_root_account` flag status
    - Table inclusion in forbidden tables list
  - Updated `process_record` method to pass `table` parameter for forbidden table checks

- **Protocol Buffer Type Field**: Added `type` field to all request messages for root request handling
  - Enhanced proto generation to include `type` field in request structures
  - Improved request classification and routing based on request type

- **Table Enum Generator Enhancement**: Enhanced `get_by_id` method to support root account access
  - Modified `generate_get_by_id_match` macro to accept `is_root_account` and `organization_id` parameters
  - Implemented conditional organization_id filtering based on root account status
  - Root accounts can now access records without organization_id restrictions
  - Non-root accounts continue to have organization-based access control enforced

### Technical Implementation
- **Core Files**:
  - `/src/grpc_macros.rs` - Fixed aggregation filter table extraction and process_record calls
  - `/src/structs/structs.rs` - Enhanced organization_id handling and process_record signature
  - `/src/schema/forbidden_tables.rs` - New forbidden tables management system
  - `/src/templates/table_enum/table_enum_macros.rs` - Fixed macro repetition and variable scoping
  - `/src/controllers/store_controller.rs` - Updated process_record calls with table parameter
  - `/src/structs/sql_constructor.rs` - Fixed entity extraction from request body

---

## 0.1.12

### Author
Kashan

### Added
- **Entity Initializer System**: Comprehensive initialization system for database entities with dynamic data generation
  - Added `initial_entity_data` module with initialization functions for connections and packets
  - Implemented `get_initial_connections()` function in `connections.rs` for generating sample connection data
  - Implemented `get_initial_packets()` function in `packets.rs` for generating sample packet data
  - Added `init.rs` with `test_error_resilient_initialization` function for validating initialization data
  - Created dynamic timestamp generation using `chrono::Utc` for current date and time formatting

### Enhanced
- **Dynamic Data Generation**: Enhanced initialization data with real-time values
  - `created_date` and `created_time` fields now use current UTC date/time formatted as "YYYY-MM-DD" and "HH:MM:SS"
  - `timestamp` and `hypertable_timestamp` fields use ISO 8601 format with microseconds ("%Y-%m-%dT%H:%M:%S%.6f+00:00")
  - `organization_id` dynamically retrieved from `DEFAULT_ORGANIZATION_ID` environment variable with fallback
  - All timestamp fields generate current values instead of using hardcoded dates

- **Environment Integration**: Enhanced configuration management
  - Added `std::env::var` integration for `DEFAULT_ORGANIZATION_ID` retrieval
  - Implemented fallback mechanism for missing environment variables
  - Ensured consistency between initialization data and environment configuration

- **Test Coverage**: Comprehensive validation for initialization data
  - Added assertions for presence of `created_date`, `created_time`, `timestamp`, and `hypertable_timestamp` fields
  - Implemented format validation for date ("YYYY-MM-DD") and time ("HH:MM:SS") fields
  - Added ISO 8601 timestamp format validation with UTC timezone verification
  - Verified `organization_id` matches environment variable configuration

### Technical Implementation
- **Core Files**:
  - `/src/initializers/initial_entity_data/connections.rs` - Connection initialization with dynamic timestamps
  - `/src/initializers/initial_entity_data/packets.rs` - Packet initialization with dynamic timestamps
  - `/src/initializers/initial_entity_data/init.rs` - Test validation for initialization data
- **Data Generation**: Real-time timestamp generation using `chrono::Utc::now()`
- **Format Standards**: ISO 8601 timestamps, separate date/time fields, environment-based organization IDs
- **Schema Compliance**: All generated fields match database schema definitions for `connections` and `packets` tables
- **Error Resilience**: Comprehensive error handling and validation in initialization process

---

## 0.1.11

### Author
Kashan

### Added
- **Root Controller System**: Implemented comprehensive root controller functionality using macro-based architecture
  - Added controller type checking logic to all store controller functions (`create_record`, `get_by_id`, `update_record`, `delete_record`, `batch_update_records`, `batch_delete_records`, `get_by_filter`, `upsert`)
  - Implemented controller type extraction from request extensions with conditional logic based on `is_root_controller` flag
  - Added comprehensive logging for root vs simple controller operations
  - Enhanced authentication middleware with root account validation logic

### Enhanced
- **Authentication Structs**: Made critical authentication fields optional to handle null values in JWT tokens
  - Modified `Claims` struct in `auth/structs.rs`:
    - Changed `role_name` from `String` to `Option<String>` with `#[serde(default)]`
    - Changed `sensitivity_level` from `u32` to `Option<u32>` with `#[serde(default)]`
  - Modified `Account` struct in `auth/structs.rs`:
    - Changed `role_id` from `String` to `Option<String>` with `#[serde(default)]`
  - Updated `auth_middleware.rs` to handle optional fields with default values:
    - `role_name` defaults to empty string using `unwrap_or_default()`
    - `sensitivity_level` defaults to 1000 using `unwrap_or(1000)`
    - `role_id` defaults to empty string using `unwrap_or_default()`
- **Authentication Middleware**: Enhanced `auth_middleware.rs` with root controller validation
  - Added logic to extract controller type from request path (`/api/store/{type}` pattern)
  - Implemented validation to prevent root accounts from accessing non-root endpoints and vice versa

### Fixed
- **JSON Deserialization**: Resolved "JSON error: invalid type: null, expected a string" errors during token verification
- **Token Caching**: Added error handling for cached token data deserialization with automatic cache cleanup for invalid entries
- **Authentication Flow**: Improved robustness of authentication middleware to handle incomplete JWT token data

### Technical Implementation
- **Core Files**:
  - `/src/auth/structs.rs` - Updated `Claims` and `Account` structs with optional fields
  - `/src/controllers/store_controller.rs` - Enhanced all controller functions with root controller logic
  - `/src/middlewares/auth_middleware.rs` - Enhanced to handle optional authentication fields and added controller type validation and path parsing
  - `/src/auth/auth_service.rs` - Added error handling for token cache deserialization
- **Controller Type Logic**: All store controller functions now extract `controller_type` from request extensions and implement conditional behavior
- **Security Enhancement**: Middleware-level validation ensures proper controller type usage based on account permissions
- **Default Values**: Implemented sensible defaults for missing authentication data
- **Backward Compatibility**: Changes maintain compatibility with existing tokens while handling edge cases
- **Logging**: Comprehensive logging added to differentiate between root and simple controller operations

---

## 0.1.10

### Author
Bert

### Fixes
- Fixed nested join functionality to properly handle multi-level table relationships
- Enhanced base WHERE clause with improved root condition handling and validation
- Optimized pluck_object selection logic to efficiently process joined table iterations
### Merged 
- latest development changes
### Added
- make command for store build release
---

## 0.1.9

### Author
Kashan

### Added
- **gRPC Aggregation Filter System**: Complete gRPC implementation for aggregation filtering
  - Added `AggregationFilter` RPC service definition in `proto_generator.rs`
  - Implemented comprehensive protobuf message definitions for aggregation operations
  - Added `generate_aggregation_filter_method!` macro for gRPC controller generation
  - Created `grpc_struct_converter.rs` with conversion functions for protobuf to internal structs

### Enhanced
- **Protocol Buffer Definitions**: Extended proto generator with aggregation filter support
  - Added `AggregationType` enum (SUM, AVG, COUNT, MIN, MAX, STDDEV, VARIANCE, ARRAY_AGG)
  - Added `FilterOperator` enum with 16 different operators (EQUAL, NOT_EQUAL, GREATER_THAN, etc.)
  - Added `LogicalOperator` enum (AND, OR) for complex filter combinations
  - Added `MatchPattern` enum (EXACT, PREFIX, SUFFIX, CONTAINS_PATTERN, CUSTOM)
  - Implemented `Aggregation`, `AggregationOrder`, `RelationEndpoint`, `FieldRelation` messages
  - Added `Join`, `FilterCriteria`, `CriteriaFilter`, `LogicalOperatorFilter` messages
  - Created `AggregationFilterRequest` and `AggregationFilterResponse` messages

- **gRPC Controller Generator**: Automated generation of gRPC service implementations
  - Enhanced `grpc_controller_generator.rs` with proper import statements
  - Added automatic generation of aggregation filter methods
  - Implemented macro-based code generation for consistent gRPC service patterns
  - Fixed syntax errors and improved code generation reliability

- **gRPC Macros System**: Comprehensive macro library for gRPC operations
  - Created `generate_aggregation_filter_method!` macro for aggregation endpoint generation
  - Enhanced existing CRUD macros with improved error handling
  - Implemented authentication integration for all gRPC methods
  - Added support for organization-based filtering in aggregation queries

- **Type Conversion System**: Robust protobuf to internal struct conversion
  - Implemented `convert_filter_criteria()` for FilterCriteria conversion
  - Added `convert_join()` for Join operation conversion
  - Created `convert_aggregation()` for Aggregation struct conversion
  - Implemented `convert_aggregation_order()` for ordering conversion
  - Added comprehensive enum mapping with fallback defaults

### Technical Implementation
- **Core Files**:
  - `/src/templates/proto_generator.rs` - Extended with aggregation filter proto definitions
  - `/src/templates/grpc_controller/grpc_controller_generator.rs` - Enhanced gRPC controller generation
  - `/src/templates/grpc_controller/grpc_macros.rs` - New aggregation filter macro implementation
  - `/src/structs/grpc_struct_converter.rs` - Complete protobuf conversion system
  - `/proto/store.proto` - Updated with aggregation filter definitions

- **gRPC Service**: `rpc AggregationFilter(AggregationFilterRequest) returns (AggregationFilterResponse)`
- **Authentication**: JWT-based authentication with organization-based filtering
- **Query Construction**: Integration with existing SQLConstructor for aggregation queries
- **Response Format**: JSON-serialized aggregation results with flexible data structure

---

## 0.1.8

### Author
Kashan

### Added
- **Aggregation Filter System**: Comprehensive aggregation functionality for data analysis
  - Added `get_by_aggregation_filter` endpoint in `store_controller.rs` for handling aggregation requests
  - Implemented `construct_aggregation` method in `sql_constructor.rs` for building complex aggregation SQL queries
  - Added `ARRAY_AGG` support to `AggregationType` enum for array aggregation operations
  - Enhanced aggregation SQL construction with table-qualified column names for improved clarity

### Enhanced
- **SQL Constructor**: Enhanced aggregation query construction with proper table prefixing
  - Modified `construct_aggregation` to include table-qualified column names in SELECT clause
  - Aggregation fields now formatted as `{agg_type}({entity}.{field}) AS {bucket_name}`
  - Improved SQL clarity, consistency, and debugging capabilities for generated aggregation queries
- **Generic Architecture**: Refactored SQL constructor to use generic type parameters
  - Implemented `SQLConstructor<T: QueryFilter>` for trait-based polymorphism
  - Created unified `QueryFilter` trait interface for both `GetByFilter` and `AggregationFilter`
  - Enhanced code reusability and maintainability through generic programming
- **Data Structures**: Extended `AggregationType` enum with `ArrayAgg` variant
  - Added `#[serde(rename = "ARRAY_AGG")]` attribute for proper JSON serialization
  - Ensures correct mapping from JSON "ARRAY_AGG" to PostgreSQL `ARRAY_AGG()` function

### Fixed
- **Request Body Parsing**: Fixed aggregation filter to use `entity` field from request body instead of URL path parameters
  - Updated `get_by_aggregation_filter` to extract table name from `parameters.entity.clone()`
  - Resolved unused variable warnings by prefixing unused path parameters with underscores
  - Ensured consistency between aggregation and regular filter endpoints

### Technical Implementation
- **Core Files**:
  - `/bin/store/src/controllers/store_controller.rs` - Aggregation endpoint implementation
  - `/bin/store/src/sql_constructor.rs` - Enhanced aggregation SQL construction
  - `/bin/store/src/structs/structs.rs` - Extended `AggregationType` enum
- **API Endpoint**: `POST /api/store/aggregation` - Aggregation filter endpoint
- **Supported Aggregations**: `Sum`, `Avg`, `Count`, `Min`, `Max`, `StdDev`, `Variance`, `ArrayAgg`
- **Example Usage**:
  ```json
  {
    "entity": "products",
    "aggregations": [{
      "aggregation": "ARRAY_AGG",
      "aggregate_on": "name",
      "bucket_name": "all_names"
    }]
  }
  ```
- **Generated SQL**: `SELECT ARRAY_AGG(products.name) AS all_names FROM products`

---

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
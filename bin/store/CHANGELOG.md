# Changelog

All notable changes to the CRDT Store project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0
### Author
Bert
### Added
- ***System Lifecycle***:
  - **LifecycleManager**: Central orchestrator for application startup, runtime, and shutdown phases
  - **StartupManager**: Handles database initialization, service setup, and component configuration
  - **RuntimeManager**: Manages health monitoring, service operations, and error handling during runtime
  - **ShutdownManager**: Ensures graceful shutdown with proper resource cleanup and service termination
  - **HealthService**: Comprehensive health monitoring with `/health` and `/health/live` endpoints
  - **LifecycleLogger**: Specialized logging system for lifecycle events and state transitions
  - **EnvConfig Integration**: Centralized configuration management replacing direct environment variable reads
  - **Memory Safety Fixes**: Resolved `RuntimeManager::default()` panic by replacing `std::mem::take` with proper swap patterns
  - **Lifecycle Documentation**: Added comprehensive README sections and visual SVG diagram for system architecture
- ***Unit tests***:
  - **Authentication Service Tests** (`auth_service_test.rs`):
    - Password hashing consistency with bcrypt format and different salts
    - Different password verification and cross-validation failure handling
    - Empty string password hashing and verification
    - Special character password handling
    - Unicode password support across different character sets
  - **Organizations Service Tests** (`organizations_test.rs`):
    - Register struct creation with all required fields
    - Register struct default implementation with empty/None values
    - AccountType parsing from string values with error handling
    - AccountType Display trait formatting for enum variants
    - AuthData struct creation with authentication credentials
    - AuthDto JSON serialization functionality
  - **Permissions Service Tests** (`permissions_test.rs`):
    - DataPermissions default implementation with empty structures
    - SchemaItem creation with database schema field mapping
    - PermissionQueryParams default value implementation
    - Permissions query generation for data access control
  - **Query Services Tests**:
    - **Search Suggestion Tests** (`search_suggestion_test.rs`):
      - AliasedJoinedEntity creation and cloning with entity aliasing
      - FieldExpression creation, cloning, and JSON serialization
      - ConcatenatedExpressions HashMap operations and type handling
      - FormatFilterResponse creation with formatted filters and search terms
      - FieldFiltersResult creation with optional field filters
      - SearchSuggestionCache hash string generation and consistency
      - Field filter matching and non-matching criteria handling
      - Concatenated expressions generation with SQL and custom separators
      - Filter formatting with and without aliased entities
      - ConcatenateField creation with custom properties and separators
      - Debug representation formatting for all struct types
      - Default search pattern fallback and multiple entity handling
    - **Find Validations Tests** (`validations_test.rs`):
      - Table validation with valid and empty table names
      - Pluck validation with valid fields and empty arrays
      - Conflicting filters validation for advance_filters and group_advance_filters
      - Concatenated fields validation with valid config and error cases
      - Distinct_by validation with valid fields, non-existent fields, and None values
      - Entity name normalization between singular and plural forms
      - Complete validation pipeline with valid and invalid configurations
    - **Aggregation Filter Tests** (`aggregation_filter_test.rs`):
      - AggregationFilter default implementations and trait testing
      - QueryFilter and AggregationQueryFilter trait implementations
      - AggregationSQLConstructor creation and organization ID handling
      - Aggregation SQL construction with success and error scenarios
      - Missing aggregations, bucket_size, and entity error handling
      - AggregationFilterWrapper creation and trait implementations
      - Debug and Clone trait implementations for filter structures
    - **Batch Update Tests** (`batch_update_test.rs`):
      - BatchUpdateFilterWrapper creation and QueryFilter trait implementation
      - BatchUpdateSQLConstructor creation with table names and organization IDs
      - WHERE clause construction with empty filters and filter criteria
      - Organization ID filtering for non-root contexts
      - Batch update SQL construction with basic and complex parameters
      - Complex SET clause handling with conditional logic and functions
      - Multiple update operations and various filter criteria combinations
      - Debug and Clone trait implementations for wrapper structures
    - Valid pass keys query generation
    - Group by field record permissions query generation
    - Role permissions query generation
    - PermissionQueryResult creation with all fields
    - ValidPassKeyResult creation functionality
    - GroupByFieldRecordPermissionsResult creation
    - RolePermissionResult creation
    - PermissionsContext creation and validation
    - PermissionQueryParams variants creation and matching
    - PermissionQueryType enum variants handling
    - DataPermissions population with schema items
    - DataPermissions JSON serialization
    - SchemaItem JSON serialization
    - PermissionQueryResult optional fields handling
  - **Message Stream Tests** (`message_stream_test.rs`):
    - SharedStreamingState initialization with empty collections
    - Channel registration functionality with valid parameters
    - Duplicate channel registration handling
    - Backpressure management functionality
    - Flushing management functionality
    - Processing queue management
    - Processing management functionality
    - Channel organization retrieval
    - Channel retrieval functionality
    - TokenBucket initialization with specified parameters
    - Token bucket creation without consumer
    - Message reception with available tokens
    - Message reception without available tokens (rejection)
    - Message emission functionality
    - Message emission from empty buffer handling
    - Token capacity increase functionality
    - Token capacity decrease functionality
    - Token setting with buffer overflow handling
    - FlushConnectionLimiter initialization
    - FlushConnectionLimiter capacity management
    - Message creation with proper field values
    - Message debug formatting
    - Message cloning functionality
    - Token bucket backpressure recovery
    - Shared state channel lifecycle management
    - Multiple organizations and channels management
  - **Storage Service Tests** (`minio_test.rs`):
    - Storage disable functionality with DISABLE_STORAGE environment variable
    - Storage enable functionality with various non-true values
    - Default storage behavior without environment variable
    - Valid bucket name generation (basic functionality)
    - Valid bucket name generation with organization ID
    - Bucket name sanitization for special characters
    - Bucket name length limits and truncation
    - Bucket name edge cases handling
    - Bucket name generation with empty organization ID
    - Complex bucket name scenarios with hyphens and numbers
    - AppState structure compilation validation
    - NoCertificateVerification struct compilation
    - Organization ID pattern generation
    - Bucket name first character extraction
---

## 0.1.74
### Author
Kashan Ali Khalid
### Fixes
- ***schema field ordering***: 
  - Reordered fields in `user_roles` table schema to match `UserRoleModel` struct field order
  - Moved `role`, `entity`, and `sensitivity_level` fields to appear after `timestamp` and before `sync_status` and `is_batch`
  - Improved consistency between database schema and model definitions for better maintainability
---

## 0.1.73
### Author
Jean
### Fixes
- ***sql constructor for selections***: Updating the selections and looking up from `pluck` first before the `pluck_oject`, so the join_selections will override the main pluck selections.

## 0.1.72
### Author
Kashan Ali Khalid
### Fixes
- ***timestamp handling***: 
  - Fixed inconsistent timestamp processing between `structs.rs` and `store_driver.rs`
  - RFC3339 compliant timestamps are now consistently converted to UTC in `structs.rs`
  - All timestamps are now normalized to UTC format without timezone suffixes for database consistency
  - Improved timestamp precision in session middleware to include microseconds (`%.3f`)
  - Enhanced sync reliability by ensuring consistent timestamp formatting across the codebase
---

## 0.1.71
### Author
Jean
### Added
- ***password verification***: Added new `/api/password/verify` endpoint for password validation
- ***structs***: Added `VerifyPasswordParams` struct for password verification requests
- ***organization service***: Added `verify_password` function for password validation logic
- ***organization controller***: Add `verify_password` function to standardize the response from the validation service
---

## 0.1.70
### Author
Kashan Ali Khalid
### Added
- ***schema***: 
  - Added `user_roles` table to the schema with role and entity fields
- ***gitignore***: 
  - Added three generated files to .gitignore: `grpc_controller.rs`, `store.rs`, and `table_enum.rs`
### Fixes
- ***schema_generator***: 
  - Fixed duplicate index creation issue in migration generator
  - Enhanced `index_exists_in_schema` function to detect both old and new index naming formats
  - Now properly checks for existing indexes in formats: `{table_name}_{field}_idx` and `idx_{table_name}_{field}`
  - Prevents creation of duplicate indexes when similar indexes already exist in older migration files
  - Added `extract_field_from_index_name` helper function to handle both naming conventions
---

## 0.1.69
### Author
Bert
### Fixes
- ***sql_constructor***: 
  - Fixed issue where field aliases were not being used in GROUP BY clauses, leading to incorrect query results
  - Fixed issue where field aliases were not being used in ORDER BY clauses, leading to incorrect query results
  - Auto pluralize all the entities to maintain consistency.
  - Fix sort issue where it was not sorting the data correctly.
---

## 0.1.68
### Author
Kashan
### Added
- ***session middleware***: Added validation to reject non-login requests without session ID
- ***session middleware***: Standardized all error responses to use ApiResponse format for consistent error handling
- ***session middleware***: Enhanced security by requiring session ID for all non-authentication routes
### Fixes

## 0.1.67
### Author
Kashan
### Added
- ***update controller***: Enhanced update function to check if processed_record contains all fields specified in pluck_fields
- ***update controller***: Added automatic complete record retrieval when pluck_fields contain missing fields from update body
- ***update controller***: Improved field validation logic to ensure pluck_fields always return requested data
### Fixes

## 0.1.66
### Author
Kashan
### Added
- ***schema validation***: Added validation in schema generator to detect and prevent foreign key constraints on TimescaleDB hypertables
- ***schema validation***: Schema generator now exits with error message when hypertable tables have foreign key constraints
- ***system fields***: Set default value of 0 for version field in all table system fields
### Fixes

## 0.1.65
### Author
Kashan
### Added
- ***database***: Added `hypertable_timestamp` column of type `text` to `signed_in_activities` table
- ***database***: Converted `signed_in_activities` table to TimescaleDB hypertable with composite primary key (`id`, `timestamp`)
- ***database***: Added TimescaleDB hypertable creation with 1-day chunk interval and `if_not_exists` option
### Fixes
- ***timestamp parsing***: Fixed timestamp parsing errors for space-separated timestamp formats (e.g., "2025-08-20 21:44:41.082307")
- ***timestamp parsing***: Enhanced RFC3339 timestamp conversion to handle both T-separated and space-separated formats
- ***timestamp parsing***: Applied consistent timestamp formatting logic across both `hypertable_timestamp` and regular timestamp parsing functions
---

## 0.1.64
### Author
Bert
### Added
- ***makefile***: added seamless installation for linux environment
- added test docker file of LINUX OS environment
---

## 0.1.63
### Author
Bert
### Added
- ***joins***: Added filters in field relation "to" property only.
- ***validation***: Added validation to check that filters are not used on 'from' RelationEndpoint.
### Fixes
- ***validation***: Fixed the validations for advance_filters to priorities the pluck, pluck_object,pluck_group_object,concatenated_fields and group_by than join's fields
---

## 0.1.62
### Author
Kashan

### Features
- ***Session Management***: Enhanced session middleware with intelligent session reuse for login routes
  - Added login route detection for POST requests to /auth endpoint
  - Implemented request body extraction and parsing to get account_id from login data
  - Added session comparison logic to reuse existing sessions when account_id matches
  - Ensured request body restoration for downstream controllers consumption
  - Added actix-http dependency for payload restoration functionality
  - Optimized session creation to avoid unnecessary new sessions for same user logins
---

## 0.1.61
### Author
Kashan

### Features
- ***Authentication***: Added sessionID to LoginResponse structure
  - Updated LoginResponse struct in organizations/structs.rs to include session_id field
  - Modified auth_service.rs to populate session_id in all LoginResponse instances
  - Updated organization_controller.rs to use sessionID from LoginResponse
  - Added LoginRequest and LoginResponse proto messages to store.proto
  - Enhanced authentication flow to consistently include session information
---

## 0.1.60
### Author
Kashan

### Features
- ***session_core***: Added automatic code generation for sessions using generate_code function
- ***session_core***: Added automatic code generation for signed_in_activities using generate_code function
- ***session_core***: Improved error handling for code generation with proper logging instead of panicking
---

## 0.1.59
### Author
Bert

### Fixes
- ***validations***: Fixed issue with validations on Find.
  - Filter fields cannot reference JOIN 'to' fields, their aliases, or entities
---

## 0.1.58
### Author
Jean

### Changes
- ***Timezone***: Added timezone conversion on the query of Find.
  - Converted from the server timezone to the client's request timezone.
- ***Date and Time fields***: Fix issue on querying date and time fields on Find.
---

## 0.1.57
### Author
Kashan

### Changes
- ***Root Controller Fix***: Fixed macro argument mismatch in root_controller.rs
  - Added missing query parameter to root_delete_record macro call to match delete_record function signature
  - Resolved compilation errors where functions expected 3 arguments but only 2 were provided
- ***Proto Generator Enhancement***: Added float type support to proto generator
  - Added Float4 type mapping to "float" for single-precision floating-point numbers
  - Added Float8 type mapping to "double" for double-precision floating-point numbers
  - Enhanced type conversion capabilities for database schema to protobuf generation
- ***Status Update***: Changed deleted status value to "Deleted"
  - Updated status representation for better clarity and consistency
---

## 0.1.56
### Author
Kashan

### Changes
- ***Session Architecture Migration***: Complete migration from Session struct to SessionModel for improved consistency
  - Replaced nested User and Origin structs with flattened fields in SessionModel
  - Updated session_middleware.rs to use SessionModel with direct field access (user_role_id, user_is_root_user, user_account_id)
  - Modified organization_controller.rs to construct SessionModel with flattened origin fields (origin_user_agent, origin_host, origin_url)
  - Fixed session_core.rs to properly handle SessionModel field access and cloning
  - Updated session_to_signed_in_activity function to work with SessionModel structure
---

## 0.1.55
### Author
Jean

### Changes
- ***Session field values saved***: Field values will lookup from the request headers.
  - Additional headers from request, `x-forwarded-location` for `location`, `x-authentication-method` for `authentication_method`
  - Update to prioritize from the request headers, `x-forwarded-for` for `ip_address` instead of getting from TCP Connection
---

## 0.1.54
### Author
Bert

### Fixes
- ***Find SQL Constructor***: issue with the JOIN selections for order where there's a missing order in JSON_AGG wrapper
  - using JSON_AGG when accessing jsonb must specifically access without ambigous selection of column
  - fix group_by selections conflict with the join selections
---

## 0.1.53
### Author
Kashan

### Changes
- ***Authentication Enhancement***: Improved authentication system to capture account_organization_id even on failed login attempts
  - Modified `auth` and `root_auth` functions to return LoginResponse with account_organization_id instead of ApiError on authentication failure
  - Added helper functions `get_account_info` and `get_root_account_info` to extract account lookup logic
  - Updated organization_controller to properly handle new authentication response format
  - Fixed session persistence to include account_organization_id in saved session data
  - Enhanced signed_in_activity logging to correctly distinguish between successful and failed authentication attempts
  - Improved security logging and audit trail capabilities

## 0.1.52
### Author
Kashan

### Changes
- ***Database Schema***: Updated table and field names for consistency
  - Renamed `signed_in_activity` table to `signed_in_activities` (pluralized)
  - Changed `account_profile_id` field to `account_organization_id` in sessions and signed_in_activities tables
  - Updated foreign key constraints to reflect new table name
  - Updated all related models, controllers, and generated files

## 0.1.51
### Author
Kashan

### Features
- ***Session Management***: Flattened session schema, from jsonb to independent tables
- ***Sign-in Activity Tracking***: Added comprehensive sign-in activity logging with database persistence
- ***Database Schema***: Added `signed_in_activity` table with proper foreign key relationships
- ***Organization Controller***: Integrated sign-in activity saving using sync_service on user authentication


## 0.1.50
### Author
Jean

### Features
- ***Account Model Schema Alignment***: Fixed UTF-8 deserialization errors in account queries
- ***Schema Accounts table***: Change `timestamp` field to Nullable<Timestamptz>
- ***FilterCriteria***: Added missing `is_search` and `has_group_count` fields to `FilterCriteria` in store.proto
- ***GRPC Struct Converter***: Added `is_search` and `has_group_count` fields to FilterCriteria
- ***Proto Generator***: Added `is_search` and `has_group_count` fields to FilterCriteria
- ***Find SQL Constructor***: Updated `get_field` to be public
- ***Search Suggestion SQL Constructor***: 
  - Search Suggestions implementations 
  - Pass main table to the `get_field` on `construct_group_by` function

## 0.1.49
### Author
Bert

### Fixes
- ***sql_constructor***: Fixed issue with like operator for pluralized fields where the passed field is a concatenated
  - CONTAINS, LIKE
---

## 0.1.48
### Author
Kashan

### Fixes
- ***Account Model Schema Alignment***: Fixed UTF-8 deserialization errors in account queries
  - Added missing `image_url` field to `AccountModel` struct to match database schema
  - Updated `timestamp` field type from `Option<String>` to `Option<chrono::NaiveDateTime>` for proper type alignment
  - Added `use chrono::NaiveDateTime;` import to support the corrected timestamp field type
  - Resolved `DeserializationError(Utf8Error)` issues during account registration and queries
- ***gRPC Controller Generator Enhancement***: Added diesel async imports
  - Added `use diesel_async::RunQueryDsl;` import to grpc controller generator to support batch update and batch delete
---

## 0.1.47
### Author
Kashan

### Features
- ***Register API Enhancement***: Added support for nested JSON data structure in register endpoint
  - Modified organization controller to accept data wrapped in a `data` field
  - Created RegisterDto struct to handle the nested structure
---

## 0.1.46
### Author
Bert

### Fixes
- ***Schema Generator Field Ordering***: Fixed field ordering issue in schema generator
  - Modified `rebuild_entire_table_in_schema` function to use proper field ordering logic
  - Ensures system fields appear before application fields in generated schema
  - Maintains consistency between schema.rs and model definitions during field rename operations
- ***Field Validation***: Enhanced data type validation for arrays and objects
  - Added validation in `store_driver.rs` to reject invalid array formats
  - Added validation in `db.rs` DatabaseTypeConverter for proper type formatting
  - Prevents automatic conversion of simple strings into arrays or objects
  - Returns proper error messages for invalid data type formats
- ***Batch Update Standardization***: Moved batch update functionality to providers
  - Standardized filter handling approach consistent with find operations
  - Updated proto FilterOperator definitions
  - Changed operators to lowercase for consistency
- ***Code Cleanup***: Removed unused methods and resolved compilation warnings
  - Cleaned up unused code in common_controller.rs, sql_constructor.rs, and batch_update modules
  - Reduced compilation warnings from 19 to minimal proto-related warnings
---

## 0.1.45
### Author
Bert

### Fixes
- ***sql_constructor***: Fixed issue with like operator for pluralized fields
  - CONTAINS, LIKE
--- 

## 0.1.44
### Author
Bert
### Fixes
- ***Timezone Conversion***: Fixed timezone conversion issue in `time_format_wrapper`
  - Added explicit handling of field aliases to avoid confusion with table names
  - Ensured correct timezone conversion for time fields in SQL queries
  - ensure checking main table for time fields
- ***Date Formatting***: Fixed date formatting issue in `date_format_wrapper`
  - Added explicit handling of field aliases to avoid confusion with table names
  - Ensured correct date formatting for date fields in SQL queries
---

## 0.1.43
### Author
Bert

### Added
- ***Hypertable Support***: Added support for hypertable tables
  - Enhanced `get_by_filter` function to handle hypertable-specific GROUP BY clause
  - Modified `construct_group_by` to include timestamp in GROUP BY for hypertable queries
---

## 0.1.42
### Author
Bert

### Fixes
- ***create_record*** controller
  - Added an Optional app_state for storage bucket creation
  - Fixed bucket creation condition check
---

## 0.1.41
### Author
Kashan

### Fixes
- ***Record Status Assignment***: Fixed conditional status field assignment in record creation
  - Modified `add_common_fields` function to only set status to "Active" if the record doesn't already contain a status field
  - Preserves existing status values from incoming records while providing default "Active" status for records without status
  - Prevents overwriting of explicitly provided status values during record creation
---

## 0.1.40
### Author
Kashan

### Improvements
- ***Error Logging***: Enhanced error logging across all controller functions
  - Added comprehensive error logging to `get_by_id` function with table name and record ID context
  - Enhanced `batch_insert_records` with detailed error logs for record processing, CSV conversion, database connection, and COPY command failures
  - Added error logging to `batch_update_records` for batch update operation failures
  - Enhanced `batch_delete_records` with error logging for batch delete operation failures
  - Added error logging to `upsert` function for upsert operation failures with table context
  - Enhanced `delete_record` with error logging including table name and record ID context
  - Added error logging to `get_by_filter` function for query execution failures
  - Enhanced `aggregation_filter` with error logging for aggregation query execution failures
  - All error logs now include relevant context (table names, record IDs) and use consistent formatting for better debugging and monitoring
---

## 0.1.39
### Author
Kashan

### Features
- ***Git Hooks***: Implemented automatic code formatting enforcement
  - Added pre-push hook that runs `cargo fmt --check` before every push
  - Added post-checkout hook for automatic setup on clone/branch switch
  - Created setup script for easy hook installation
  - Added Makefile targets: `fmt`, `fmt-check`, and `setup-hooks`
  - Self-sustaining system ensures all developers get formatting enforcement automatically
---

## 0.1.38
### Author
Kashan

### Fixes
- ***Schema Generator***: Fixed formatting issues in schema generation
  - Enhanced `detect_field_indentation` function to properly detect existing indentation patterns
  - Fixed `add_fields_to_existing_table` to ensure proper table structure reconstruction
  - Corrected closing brace indentation in table definitions
  - Resolved formatting issues when adding new fields to existing tables
  - All table fields now consistently use 8-space indentation with proper 4-space closing braces
---

## 0.1.37
### Author
Kashan

### Fixes
- ***Schema Generator***: Fixed field ordering in schema and model generation
  - System fields now correctly appear first in generated schemas
  - Fixed VARCHAR parsing issue causing double parentheses in migrations
  - Manually corrected field order in `files` and `test_hypertable` schemas
---

## 0.1.36
### Author
Kashan

### Added
- ***Schema Generator***: Enhanced VARCHAR type handling in schema generation
  - Added support for preserving VARCHAR lengths in migrations while converting to Text in schema
  - New tables: `account_phone_numbers`, `account_signatures`
  - Added fields to `account_profile`: phone number and signature support
  - Improved field type conversion for better Diesel compatibility
---

## 0.1.35
### Author
Bert
### Fixes
- ***Order By***: Fixed issue with empty order by fields
  - Added check for empty order by fields
  - Skipped order by construction if fields are empty
- ***Group By***: Fixed issue with empty group by fields
  - Added check for empty group by fields
  - Skipped group by construction if fields are empty
- ***Distinct By***: Fixed issue with empty distinct by fields
  - Added check for empty distinct by fields
  - Skipped distinct by construction if fields are empty
---
## 0.1.34
### Author
Kashan

### Fixes
- ***Schema Generator***: Fixed bigint type mapping issue in schema generator
  - Added missing `bigint()` and `nullable(bigint())` type mappings in generator service
  - Bigint fields now correctly map to `Int8` instead of defaulting to `Text`
  - Ensures proper i64 type generation in Rust models
---

## 0.1.33
### Author
Bert

### Added
- ***get file by id***: Added endpoint to retrieve file content by file ID
  - Endpoint: `GET /file/:id`
  - Returns file content with appropriate content type and disposition
- ***download file by id***: Added endpoint to download file content by file ID
  - Endpoint: `GET /file/:id/download`
  - Returns file content with appropriate content type and disposition which behaves like a preview
  - add token as query 't' to provide token
---

## 0.1.32
### Author
Bert

### Fixes
- ***Concatenated Fields***: Fixed issue with concatenated fields not being properly shown in selections
  - Adjusted `sql_constructor.rs` to handle concatenated fields in SELECT clause
  - Ensured concatenated fields appear correctly in query results
  - Fixed field name resolution for concatenated columns
  - Added proper aliasing for concatenated field selections
---

## 0.1.31
### Author
Bert

### Fixes
- ***Timezone Handling***: Fixed timezone conversion issue in `sql_constructor.rs`
  - Adjusted timestamp conversion logic to handle timezone offsets correctly
  - Ensured consistent timezone handling across all database operations
  - revise the cast `_time` field text type cast to time type
  - Concatenated fields can now be filtered
  - Duplicate concatenated fields are now handled correctly

---

## 0.1.30
### Author
Bert

### Added 
- ***File Storage***: Implemented file storage system using AWS S3
  - can be disabled `DISABLE_STORAGE`
  - `upload_file` able to upload file to Minio and save to database
---

## 0.1.29

### Author
Kashan Ali Khalid

### Fixed
- ***Foreign Key Generation***: Fixed foreign key constraint generation where referenced columns were empty
  - Corrected parsing logic in `generator_service.rs` to properly distinguish between `columns:` and `foreign_columns:`
  - Foreign key constraints now correctly reference the target table's `id` column
---

## 0.1.28

### Author
Kashan Ali Khalid

### Added
- ***Switch Organization Feature***: Added ability for users to switch between organizations
  - Implemented `switch_account` endpoint in `src/controllers/store_controller.rs`
  - Added `SwitchAccountRequest` and `SwitchAccountData` structs in `src/structs/structs.rs`
  - Added root controller endpoint `root_switch_account` in `src/controllers/root_controller.rs`
  - Token verification and generation for organization switching

### Fixed
- ***System Indexes***: Updated system indexes to be prefixed with table names to prevent duplicate index conflicts
  - Modified `system_indexes` macro to accept table name parameter
  - Updated all table definitions to use parameterized macro syntax
  - Enhanced migration generator to handle prefixed index names
- ***Migration Generator***: Fixed default value quoting for TEXT fields in SQL migrations
  - Added proper single quote wrapping for string default values
  - Resolved "cannot use column reference in DEFAULT expression" errors
---

## 0.1.27

### Author
Kashan Ali Khalid

### Added
- ***Hypertable Support***: Added TimescaleDB hypertable creation and validation
- ***Separate System Fields Macro***: Extracted system fields into dedicated macro for better organization
- ***Field Override Logic***: System fields are now overridden by explicit schema definitions
- ***Schema Defaults***: Fixed default value handling in schema generation
- ***File-based Table Naming***: Table names now derive from file names for consistency
- ***Migration Nullable Option***: Added nullable configuration support in schema definitions
- ***Field Order Preservation***: Maintained field order consistency between schema and model generation
- ***Composite Primary Keys***: Added support for multi-column primary key definitions
- ***Hypertable Array Management***: Automatic addition of hypertables to schema configuration array

### Fixed
- ***Field Type Validation***: Resolved timestamp field type parsing issues in hypertable validation
- ***System Field Synchronization***: Fixed conflicts between system fields and explicit field definitions
---

## 0.1.26

### Author
Kashan Ali Khalid

### Fixed
- ***Message Queue Processing***: Resolved critical issues with message dequeuing and processing
  - Fixed race condition where multiple processes could dequeue the same message items simultaneously
  - Eliminated duplicate message processing by ensuring messages are marked as consumed immediately after being sent to token bucket
  - Moved `consumed_ids.push(item.id)` to execute right after `bucket.receive_message(msg).await` in streaming service
- ***Timestamp Formatting***: Fixed timestamp formatting issues in record creation
  - Corrected timestamp serialization to prevent malformed data entries
---

## 0.1.25

### Author
Kashan Ali Khalid

### Added
- ***Token Verification Route***: Added new `/api/token/verify` endpoint for JWT token validation
  - Implemented `verify_token` method in `OrganizationsController` for standalone token verification
  - Supports token extraction Authorization header (`Bearer <token>`)
  - Returns decoded token claims and account information without access control restrictions
  - Provides consistent error handling for invalid or missing tokens
  - Enables client applications to verify token validity independently of protected routes
  - Configured as GET endpoint under `/api/token` scope with session middleware
---

## 0.1.24

### Author
Kashan Ali Khalid

### Refactored
- ***Type Conversion Simplification***: Removed complex type conversion logic and unnecessary intermediate structures
  - Eliminated redundant `DatabaseTypeConverter` usage in direct database insertion paths
  - Simplified data flow from `store_driver.rs` to database operations
  - Leveraged Diesel ORM's native type conversion capabilities for `serde_json::Value` to PostgreSQL types
  - Removed unnecessary struct intermediaries in favor of direct JSON-to-model conversion using `serde_json::from_value`
  - Streamlined upsert operations to use macro-generated code with built-in type handling
  - Improved code maintainability by reducing conversion complexity while maintaining full type safety
---

## 0.1.23

### Author
Kashan Ali Khalid

### Enhanced
- ***Migration Generator***: Added statement-breakpoint functionality to automatically generated migrations
  - Modified `migration_generator.rs` to include `--> statement-breakpoint` comments between SQL statements
  - Updated both `generate_up_sql` and `generate_down_sql` functions to add breakpoints
  - Improved readability and consistency with existing migration format
  - Enhanced tool compatibility for database migration tools and IDEs
  - Added breakpoints between CREATE TABLE, ALTER TABLE, CREATE INDEX, and foreign key constraint statements
- ***Schema Automation***: Enhanced automated schema generation system
  - Integrated foreign key extraction and processing in `generator_service.rs`
  - Updated `schema_generator.rs` to handle foreign key schema changes with `NewForeignKey` type
  - Automated generation of foreign key constraints in migration files
  - Complete end-to-end automation: table definition files → model generation → schema updates → migration creation
  - Supports complex relationships including foreign keys with ON DELETE/UPDATE actions
---

## 0.1.22
### Author
Bert

### Features
- **Schema Generator**: Added comprehensive schema generator feature that allows users to define database schemas through simple configuration files
  - Automatically generates Rust model files with Diesel traits
  - Updates `schema.rs` with new table definitions
  - Creates database migrations with proper up/down SQL
  - Supports intelligent re-runs without duplicating existing fields
  - Interactive migration naming with conflict detection
  - Supports all major Diesel types including arrays, nullable types, and foreign keys
  - Integrated with main application via `CREATE_SCHEMA` environment variable

### Fixes
- ***validations.rs*** revise validation for distinct_by that causes the app to crash
- Main table selections are not shown if there's a join
- set `aliased_entity` to optional in `concatenate_fields`
- fix concatenated field names validation for main fields
- fix the concatenated fields selections for main fields
---

## 0.1.21

### Author
Kashan Ali Khalid

### Fixed
- ***Separation of Concerns***: Corrected improper mixing of aggregation and find SQL constructors
  - Fixed `get_by_filter` function in `store_controller.rs` that incorrectly used `AggregationSQLConstructor` instead of `SQLConstructor` from the find module
  - Changed method call from `construct_aggregation()` to `construct()` to match the find module's API
  - Established clear architectural boundary: aggregation module handles aggregation queries, find module handles regular filtering

### Refactored
- ***aggregation_filter module***: Consolidated `query_filter.rs` and `sql_constructor.rs` into single file
  - Moved `AggregationFilter` and `AggregationFilterWrapper` trait implementations from `query_filter.rs` to `sql_constructor.rs`
  - Removed `query_filter.rs` file and updated `mod.rs` exports
  - Kept all `QueryFilter` and `AggregationQueryFilter` implementations intact
- ***grpc_controller_generator***: Removed unused `SQLConstructor` import
  - Eliminated `use crate::providers::find::SQLConstructor;` line that was removed from manual `grpc_controller.rs`
  - Maintained consistency between generated and manually maintained controller files

### Technical Debt
- ***AggregationQueryFilter trait***: Added default implementations to eliminate dead code warnings
  - Provided default implementations for `get_advance_filters()`, `get_joins()`, `get_date_format()`, `get_order_by()`, and `get_order_direction()`
  - Added `#[allow(dead_code)]` attributes to suppress compiler warnings for unused trait methods
  - Maintained backward compatibility while cleaning up compilation warnings
---

## 0.1.20

### Author
Kashan Ali Khalid

### Fixed
- ***grpc_controller_generator***: Fixed syntax errors in gRPC controller generator
  - Corrected malformed `writeln!` macro calls with proper string formatting
  - Fixed delimiter mismatches and string escaping issues
  - Resolved compilation errors in server initialization code
  - Ensured proper dynamic service name usage throughout the generator

### Changed
- ***code_prefix_init***: Updated prefix initialization to use upsert behavior
  - Changed from `on_conflict_do_nothing()` to `on_conflict().do_update()`
  - Modified initialization to update `prefix` and `default_code` fields when entity conflicts occur
  - Added proper Diesel upsert syntax using `diesel::upsert::excluded()`
  - Updated function documentation and log messages to reflect upsert behavior
  - Ensured configuration changes are properly applied to existing counter records
---

## 0.1.19

### Author
Kashan Ali Khalid

### Changed
- ***core***: Migrated all UUID fields to ULID for better performance and sortability
  - Updated all database schemas and models to use ULID instead of UUID
  - Modified ID generation throughout the codebase
  - Updated CRDT libraries to use ULID instead of UUID
  - Ensured backward compatibility during migration

### Added
- ***parsers***: Enhanced SQL parsing capabilities
  - Updated parsers for SQL to row conversion
  - Updated parsers for row to SQL conversion
  - Added support for additional PostgreSQL types in CRDT insert record operations
  - Improved type safety and conversion accuracy
- ***database***: Added system fields to all tables
  - Added `is_batch` field to track batch operations
  - Added `sync_status` field to monitor synchronization state
  - Created corresponding database migrations for system fields

### Fixed
- ***initialization***: Moved initialization logic to background services
  - Moved code prefix initialization from main.rs to background_services_init.rs
  - Moved initial entity data initialization from main.rs to background_services_init.rs
  - Ensured initializers run consistently on every store startup
  - Fixed recursion issues in initialization process
---

## 0.1.18

### Author
Bert

### Added
- ***find***: Implemented distinct_by query parameter
  - Added `get_distinct_by` method to `FindRequest` struct
  - Modified `construct_selections` method to handle distinct_by
  - Updated `construct_query` method to include distinct_by in query construction
  - Update `construct_order_by` method to handle distinct_by
---

## 0.1.17

### Author
Kashan Ali Khalid

### Fixed
- ***grpc_macros***: Modified `generate_get_method` macro in `grpc_macros.rs` to extract `pluck_fields` from gRPC query
  - Added query extraction logic: `let query = request.query.ok_or_else(|| Status::invalid_argument("Query is required"))?;`
  - Added pluck_fields parsing: `let pluck_fields = if !query.pluck.is_empty() { Some(query.pluck.split(',').map(|s| s.trim().to_string()).collect()) } else { Some(vec!["id".to_string()]) };`
  - Updated `process_and_get_record_by_id` call to pass extracted `pluck_fields` instead of hardcoded `None`
  - Changed default behavior from returning `None` to returning `["id"]` when no fields specified
- ***grpc_macros***: **SECURITY FIX** - Added missing root access validation to `generate_get_method`, `generate_aggregation_filter_method`, `generate_batch_insert_method`, and `generate_batch_delete_method`
  - Fixed critical security vulnerability where non-root tokens with root type could bypass authentication
  - Added `validate_grpc_request_with_root_access` calls to all affected methods
  - Standardized parameter extraction pattern: `let params = match request.get_ref().params { Some(ref p) => p.clone(), None => return Err(Status::invalid_argument("Params are required")), };`
  - Implemented consistent root access validation: `let (auth_data, _claims) = crate::middlewares::auth_middleware::validate_grpc_request_with_root_access(&request, &params.r#type)?;`
- ***session_middleware***: Integrated `with_session_management!` macro wrapper across all gRPC method macros
  - Added session loading: `load_and_populate_session_for_grpc(&request)`
  - Added session extension insertion: `request.extensions_mut().insert(session.clone())`
  - Added session persistence: `save_session_after_request(&session)`
  - Applied to `generate_create_method`, `generate_update_method`, `generate_get_method`, `generate_upsert_method`, `generate_delete_method`, and `generate_batch_delete_method`
- ***auth_middleware***: Enhanced gRPC request validation to work seamlessly with session management
  - Updated authentication flow to extract session data from request extensions
  - Maintained compatibility with existing `validate_grpc_request_with_root_access` function calls
---

## 0.1.16

### Author
Bert

### Improvements
- validations for advance_filters - validates filter criteria and field existence
- validations for group_advance_filters - validates grouped filter criteria and logical operators
- validations for concatenated_fields - validates field existence in specified entities and ensures non-empty field arrays
- validations for group_by - validates that all group by fields exist in the target table
- validations for joins - validates join types (LEFT, SELF), field relations, and nested join sequences
- validations for order_by_format - validates order by field format and field existence
- validations for order_direction - validates sort direction values (asc, desc)
- validations for date_format - validates date format strings against allowed patterns
- validations for multiple_sort - validates multiple sort criteria including field existence and direction
- validations for limit_offset - validates limit and offset parameter constraints
---

## 0.1.15
### Author
Bert

### Fixed
- ***organization_controllers***: Fixed issue with `get_organization` function not returning `role_id` `account_id` and `is_root_user` in `user` property
- ***auth_service***: Fixed issue with `login` function not returning `role_id` in `LoginResponse` struct
- Fixed missing `origin` property in `Session` struct during login process
  - Added `origin` field to `Session` struct definition
  - Enhanced `login` function to properly set the `origin` value
  - Ensured `origin` is consistently populated during session creation
---

## 0.1.14

### Author
Bert

### Fixed
- ***construct_join_selections***: Fixed nested join selection issue in `construct_join_selections` function
  - Updated `construct_join_selections` to handle nested join selections more effectively
  - Ensured that join selections are constructed correctly for complex query structures`
### Added
- ORDER BY clause in `construct_join_selections` function for join selections
  - Added `multiple_sort` or `order` parameter to `construct_join_selections` function
  - Implemented ORDER BY clause construction for join selections
  - Ensured that join selections are ordered correctly based on `multiple_sort` or `order` parameter
  - Nested join selections are also ordered correctly based on `multiple_sort` or `order` parameter
- Add self join support in `construct_join_selections` function
---

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
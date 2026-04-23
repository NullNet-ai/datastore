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

## Hypertable Requirements

When `hypertable: true` is specified, the following requirements are enforced:

1. **Required Fields**:
   - `timestamp: timestamptz()` - The time dimension field
   - `hypertable_timestamp: text()` - Additional timestamp metadata
   - `id` - Unique identifier field

2. **Primary Key**: Must be a composite key with `timestamp` and `id`

3. **Validation**: Automatic validation occurs when `validate_schema()` is called

## Table Definition Formats

### Struct-Based with Diesel Types

Use actual Diesel types for better type safety and IDE support:

#### Basic Hypertable Example

```rust
// Example: metrics_table.rs
use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct MetricsTable;

define_table_schema! {
    table_name: "metrics",
    fields: {
        id: uuid(), primary_key: true,
        timestamp: timestamptz(), primary_key: true,  // Required
        hypertable_timestamp: text(),                 // Required
        metric_name: text(),
        metric_value: DieselType::Numeric,
        tags: nullable(jsonb())
    },
    hypertable: true,  // Enable validation
    indexes: {
        idx_metrics_name: {
            columns: ["metric_name"],
            unique: false,
            type: "btree"
        }
    }
}
```

#### Regular Table Example

```rust
// Example: user_profile_struct.rs
use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct UserProfileTable;

// Using the convenient macro with various types
define_table_schema! {
    table_name: "user_profiles",
    fields: {
        id: integer(), primary_key: true,
        user_id: integer(), indexed: true,
        display_name: nullable_text(),
        bio: nullable_text(),
        avatar_url: varchar(Some(500)),
        email: varchar(Some(255)), indexed: true,
        age: nullable(smallint()),
        salary: nullable(numeric()),
        rating: nullable(float()),
        ip_address: nullable_inet(),
        mac_address: nullable(macaddr()),
        preferences: nullable_jsonb(), default: "{}",
        tags: nullable_text_array(),
        scores: nullable_integer_array(),
        is_public: nullable_boolean(), default: "true",
        is_verified: boolean(), default: "false",
        birth_date: nullable(date()),
        last_login: nullable_timestamptz(),
        created_at: timestamptz(), default: "CURRENT_TIMESTAMP",
        updated_at: timestamptz(), default: "CURRENT_TIMESTAMP"
    },
    // hypertable: false (default)
    indexes: {
        idx_user_profiles_user_id: {
            columns: ["user_id"],
            unique: false,
            type: "btree"
        },
        idx_user_profiles_email: {
            columns: ["email"],
            unique: true,
            type: "btree"
        }
    },
    foreign_keys: {
        user_id -> "users"."id",
        on_delete: "CASCADE"
    }
}
```

### Manual Implementation (for complex cases)

```rust
pub struct ComplexUserProfileTable;

impl DieselTableDefinition for ComplexUserProfileTable {
    fn table_name() -> &'static str {
        "complex_user_profiles"
    }
    
    fn field_definitions() -> Vec<DieselFieldDefinition> {
        vec![
            DieselFieldDefinition {
                name: "id".to_string(),
                diesel_type: DieselType::Integer,
                is_primary_key: true,
                is_nullable: false,
                default_value: None,
                is_indexed: true,
            },
            DieselFieldDefinition {
                name: "user_id".to_string(),
                diesel_type: DieselType::Integer,
                is_primary_key: false,
                is_nullable: false,
                default_value: None,
                is_indexed: true,
            },
            // ... more fields
        ]
    }
}
```

## Indexes and partial indexes (WHERE)

Indexes are defined in the `indexes: { ... }` block. Each index must be named `idx_{table_name}_{column_names}` (e.g. `idx_index_demos_name_region`). You can define:

- **Simple indexes**: single or multiple columns, `type: "btree"` (or `hash`, `gin`, `gist`, `spgist`, `brin`), `unique: true/false`.
- **Partial indexes**: add a `where` clause so the index only includes rows matching a predicate (smaller, faster for filtered queries).

### Where clause format (idiomatic block)

Use the **block format** with unquoted keys. The predicate is expressed as a small expression tree:

| Shape | Meaning | Example |
|-------|--------|--------|
| `{ op, column, value }` | Single comparison | `{ op: "=", column: "status", value: "Active" }` |
| `{ and: [ ... ] }` | All conditions true | `{ and: [ { op: "=", column: "status", value: "Active" }, { op: "=", column: "region", value: "US" } ] }` |
| `{ or: [ ... ] }` | Any condition true | `{ or: [ { op: "=", column: "status", value: "check" }, { op: "=", column: "status", value: "hehe" } ] }` |
| `{ not: ... }` | Negate one expression | `{ not: { op: "IS NOT", column: "deleted_at", value: null } }` |

**Supported ops**: `=`, `!=`, `<`, `<=`, `>`, `>=`, `IN`, `NOT IN`, `LIKE`, `ILIKE`, `IS`, `IS NOT`.  
**Values**: strings, numbers, booleans, `null`, or arrays for `IN`/`NOT IN`.

Generated SQL example:

```sql
CREATE INDEX idx_index_demos_region_name ON index_demos USING btree (region, name)
WHERE (status = 'check' OR status = 'hehe') AND name = 'John Doe';
```

### Full example: index_demos table (all scenarios)

This table demonstrates every index scenario in one place. You can use it as a reference or copy the patterns into your own tables.

```rust
use crate::define_table_schema;
use crate::generated::schema::generator::diesel_schema_definition::{
    types::*, DieselTableDefinition,
};
use crate::{system_fields, system_foreign_keys, system_indexes};

pub struct IndexDemosTable;

define_table_schema! {
    hypertable: false,
    fields: {
        system_fields!(),

        name: nullable(text()),
        region: nullable(text()),
        status: nullable(text()),
        priority: nullable(integer()),
        deleted_at: nullable(timestamptz()),
        category: nullable(text()),
        score: nullable(integer()),
    },
    indexes: {
        system_indexes!("index_demos"),

        // 1. Simple single-column btree (no where)
        idx_index_demos_name: {
            columns: ["name"],
            unique: false,
            type: "btree"
        },

        // 2. Compound (multi-column) btree
        idx_index_demos_region_status: {
            columns: ["region", "status"],
            unique: false,
            type: "btree"
        },

        // 3. Unique index (name must be idx_table_col1_col2)
        idx_index_demos_name_region: {
            columns: ["name", "region"],
            unique: true,
            type: "btree"
        },

        // 4. Partial index: single predicate (status = 'Active')
        idx_index_demos_status_region: {
            columns: ["status", "region"],
            unique: false,
            type: "btree",
            where: {
                op: "=",
                column: "status",
                value: "Active"
            }
        },

        // 5. Partial index: OR (status = 'check' OR status = 'hehe')
        idx_index_demos_status_name: {
            columns: ["status", "name"],
            unique: false,
            type: "btree",
            where: {
                or: [
                    { op: "=", column: "status", value: "check" },
                    { op: "=", column: "status", value: "hehe" }
                ]
            }
        },

        // 6. Partial index: AND (status = 'Active' AND region = 'US')
        idx_index_demos_status_region_priority: {
            columns: ["status", "region", "priority"],
            unique: false,
            type: "btree",
            where: {
                and: [
                    { op: "=", column: "status", value: "Active" },
                    { op: "=", column: "region", value: "US" }
                ]
            }
        },

        // 7. Partial index: (status = 'check' OR status = 'hehe') AND name = 'John Doe'
        idx_index_demos_region_name: {
            columns: ["region", "name"],
            unique: false,
            type: "btree",
            where: {
                and: [
                    {
                        or: [
                            { op: "=", column: "status", value: "check" },
                            { op: "=", column: "status", value: "hehe" }
                        ]
                    },
                    { op: "=", column: "name", value: "John Doe" }
                ]
            }
        },

        // 8. Partial index: IS NULL (deleted_at IS NULL)
        idx_index_demos_status_category: {
            columns: ["status", "category"],
            unique: false,
            type: "btree",
            where: {
                op: "IS",
                column: "deleted_at",
                value: null
            }
        },

        // 9. Partial index: IN (status IN ('Active','Pending'))
        idx_index_demos_status_region_category: {
            columns: ["status", "region", "category"],
            unique: false,
            type: "btree",
            where: {
                op: "IN",
                column: "status",
                value: ["Active", "Pending"]
            }
        },

        // 10. Partial index: NOT (deleted_at IS NOT NULL) = IS NULL
        idx_index_demos_name_status: {
            columns: ["name", "status"],
            unique: false,
            type: "btree",
            where: {
                not: { op: "IS NOT", column: "deleted_at", value: null }
            }
        },

        // 11. Partial index: LIKE (name LIKE 'test%')
        idx_index_demos_name_category: {
            columns: ["name", "category"],
            unique: false,
            type: "btree",
            where: {
                op: "LIKE",
                column: "name",
                value: "test%"
            }
        },

        // 12. Partial index: numeric comparison (priority > 0)
        idx_index_demos_priority_score: {
            columns: ["priority", "score"],
            unique: false,
            type: "btree",
            where: {
                op: ">",
                column: "priority",
                value: 0
            }
        },
    },
    foreign_keys: {}
}

impl DieselTableDefinition for IndexDemosTable {}
```

### Index scenario summary

| # | Scenario | Columns | Where | Generated SQL pattern |
|---|----------|---------|--------|------------------------|
| 1 | Simple | `["name"]` | — | `CREATE INDEX ... ON t ("name");` |
| 2 | Compound | `["region", "status"]` | — | `... ON t ("region", "status");` |
| 3 | Unique | `["name", "region"]` | — | `CREATE UNIQUE INDEX ...` |
| 4 | Partial, single pred | `["status", "region"]` | `status = 'Active'` | `... WHERE "status" = 'Active';` |
| 5 | Partial, OR | `["status", "name"]` | `status = 'check' OR status = 'hehe'` | `... WHERE ... OR ...;` |
| 6 | Partial, AND | `["status", "region", "priority"]` | `status = 'Active' AND region = 'US'` | `... WHERE ... AND ...;` |
| 7 | Partial, AND of OR | `["region", "name"]` | `(status = 'check' OR 'hehe') AND name = 'John Doe'` | `... WHERE (...) AND ...;` |
| 8 | Partial, IS NULL | `["status", "category"]` | `deleted_at IS NULL` | `... WHERE "deleted_at" IS NULL;` |
| 9 | Partial, IN | `["status", "region", "category"]` | `status IN ('Active','Pending')` | `... WHERE "status" IN (...);` |
| 10 | Partial, NOT | `["name", "status"]` | `NOT (deleted_at IS NOT NULL)` | `... WHERE NOT (...);` |
| 11 | Partial, LIKE | `["name", "category"]` | `name LIKE 'test%'` | `... WHERE "name" LIKE 'test%';` |
| 12 | Partial, comparison | `["priority", "score"]` | `priority > 0` | `... WHERE "priority" > 0;` |

### Index naming and modification rules

- **Naming**: Index name must be `idx_{table_name}_{column_names}` (e.g. `idx_index_demos_status_region`). The generator validates this.
- **Existing indexes**: If an index already exists in a migration, you cannot change its definition (columns, type, unique, where). The generator will error and show the existing SQL; update your table definition to match it or create a new index with a new name.

## Validation

```rust
// Validate schema requirements
MetricsTable::validate_schema();  // Panics if hypertable requirements not met
UserProfileTable::validate_schema();    // Always succeeds for regular tables

// Check if table is a hypertable
if MetricsTable::is_hypertable() {
    println!("This is a hypertable!");
}
```

### Duplicate Detection (Aborts on Error)

The generator validates and **aborts** (does not continue) when it detects:

- **Duplicate field names**: Do not redefine system fields (`organization_id`, `created_by`, `tags`, etc.) or add the same field twice.
- **Duplicate index names**: Do not add an index that `system_indexes!` already defines (e.g. `idx_{table}_organization_id`).
- **Duplicate foreign keys**: Do not add a foreign key for columns already in `system_foreign_keys!` (`organization_id`, `created_by`, `updated_by`, `deleted_by`, `requested_by`).

On validation failure, the process stops and returns an error message.

## Available Diesel Types

### Helper Functions
```rust
use crate::schema::generator::diesel_schema_definition::types::*;

// Integer types
smallint()    // SmallInt (i16)
integer()     // Int4 (i32)
bigint()      // Int8 (i64)

// Text types
text()                    // Text (unlimited length)
varchar(Some(255))        // VarChar with length limit
varchar(None)             // VarChar without length limit
char(10)                  // Char with fixed length

// Floating point types
float()       // Float (f32)
double()      // Double (f64)
numeric()     // Numeric (decimal)

// Boolean type
boolean()     // Bool

// Date and time types
date()        // Date
time()        // Time
timestamp()   // Timestamp (without timezone)
timestamptz() // Timestamptz (with timezone)

// JSON types
json()        // Json
jsonb()       // Jsonb (binary JSON, recommended)

// Network types
inet()        // Inet (IP address)
cidr()        // Cidr (network address)
macaddr()     // MacAddr (MAC address)

// Other types
binary()      // Binary data
uuid()        // UUID

// Wrapper types
nullable(text())           // Nullable<Text>
array(text())              // Array<Text>
nullable(array(text()))    // Nullable<Array<Text>>

// Convenience functions for common nullable types
nullable_text()            // Nullable<Text>
nullable_integer()         // Nullable<Int4>
nullable_bigint()          // Nullable<Int8>
nullable_boolean()         // Nullable<Bool>
nullable_timestamp()       // Nullable<Timestamp>
nullable_timestamptz()     // Nullable<Timestamptz>
nullable_jsonb()           // Nullable<Jsonb>
nullable_uuid()            // Nullable<Uuid>
nullable_inet()            // Nullable<Inet>

// Convenience functions for common array types
text_array()               // Array<Text>
integer_array()            // Array<Int4>
nullable_text_array()      // Nullable<Array<Text>>
nullable_integer_array()   // Nullable<Array<Int4>>
```

### Direct DieselType Enum
```rust
DieselType::Integer
DieselType::BigInt
DieselType::Text
DieselType::Bool
DieselType::Timestamptz
DieselType::Jsonb
DieselType::Inet
DieselType::Array(Box::new(DieselType::Text))
DieselType::Nullable(Box::new(DieselType::Text))
```

## Benefits

✅ **Type Safety**: Uses actual Diesel types, preventing typos and mismatches  
✅ **IDE Support**: Full autocomplete and error checking  
✅ **Compile-Time Validation**: Catches errors before runtime  
✅ **Better Tooling**: Works with Rust analyzers and formatters  
✅ **Extensible**: Easy to add custom validation and logic  
✅ **Documentation**: Self-documenting with proper Rust docs  

## Migration Best Practices

### Problem: Duplicate Index/Foreign Key Creation

The issue about migrations recreating indexes and foreign keys can be solved with conditional SQL:

### Solution 1: Conditional Index Creation

```sql
-- Instead of:
CREATE INDEX idx_metrics_name ON metrics (metric_name);

-- Use:
CREATE INDEX IF NOT EXISTS idx_metrics_name ON metrics (metric_name);
```

### Solution 2: Conditional Foreign Key Creation

```sql
-- Check if foreign key exists before creating
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.table_constraints 
        WHERE constraint_name = 'fk_metrics_device_id'
          AND table_name = 'metrics'
    ) THEN
        ALTER TABLE metrics 
        ADD CONSTRAINT fk_metrics_device_id 
        FOREIGN KEY (device_id) REFERENCES devices(id) 
        ON DELETE CASCADE ON UPDATE CASCADE;
    END IF;
END $$;
```

### Solution 3: Migration State Tracking

```sql
-- Create a migration tracking table
CREATE TABLE IF NOT EXISTS schema_migrations (
    version VARCHAR(255) PRIMARY KEY,
    applied_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Check before applying migrations
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM schema_migrations WHERE version = '001_create_metrics'
    ) THEN
        -- Apply migration
        CREATE TABLE metrics (...);
        CREATE INDEX idx_metrics_name ON metrics (metric_name);
        
        -- Mark as applied
        INSERT INTO schema_migrations (version) VALUES ('001_create_metrics');
    END IF;
END $$;
```

### Solution 4: Incremental Schema Updates

```rust
// Instead of regenerating entire schema, use incremental updates
pub fn apply_schema_changes() {
    // Only validate new/changed tables
    NewTable::validate_schema();
    
    // Generate only differential migrations
    let changes = detect_schema_changes();
    apply_incremental_migration(changes);
}
```

## Error Handling

The validation system provides clear error messages:

```rust
// This will panic with a descriptive message:
// "Hypertable 'invalid_table' requires a 'timestamp' field of type Timestamptz"
InvalidHypertable::validate_schema();
```

## Integration with TimescaleDB

After creating the table structure, convert to hypertable:

```sql
-- Convert regular table to hypertable
SELECT create_hypertable('metrics', 'timestamp', if_not_exists => TRUE);

-- Set chunk time interval (optional)
SELECT set_chunk_time_interval('metrics', INTERVAL '1 day');

-- Enable compression (optional)
ALTER TABLE metrics SET (timescaledb.compress = true);
```

## Field Properties

- `name`: The database column name
- `diesel_type`: The Diesel SQL type
- `indexed`: Whether to create an index (optional, default: false)
- `primary_key`: Whether this is the primary key (optional, default: false)
- `foreign_keys`: Foreign key reference in format `table.column` (optional)
- `default`: Default value for the column (optional)

### 5. What Gets Generated

When you run the schema generator, it will:

1. **Generate Model Files**: Creates `{table_name}_model.rs` in `src/database/models/`
2. **Update Schema**: Adds table definitions to `src/database/schema/schema.rs`
3. **Create Migrations**: Generates SQL migration files in `migrations/`
4. **Update Models Module**: Updates `src/database/models/mod.rs` to include new models

### 6. Smart Updates

The generator is designed to be re-runnable:

- **Existing Fields**: Won't duplicate existing fields in `schema.rs`
- **New Fields**: Only adds new fields to existing tables
- **Indexes**: Creates indexes only for new fields marked with `is_index: true`
- **Migrations**: Only creates migrations when actual changes are detected

### 7. Migration Naming

When changes are detected, the generator will:

1. Prompt you for a migration name via terminal input
2. Check if the migration name already exists
3. Ask for a different name if there's a conflict
4. Create the migration with timestamp prefix

### 8. Example Generated Files

#### Model File (`user_profiles_model.rs`)

```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfile {
    pub id: i32,
    pub user_id: i32,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = user_profiles)]
pub struct NewUserProfile {
    pub user_id: i32,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
}
```

#### Schema Addition (`schema.rs`)

```rust
table! {
    user_profiles (id) {
        id -> Int4,
        user_id -> Int4,
        display_name -> Nullable<Text>,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        preferences -> Nullable<Jsonb>,
        tags -> Nullable<Array<Text>>,
        is_public -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
```

#### Migration Files

**up.sql**:
```sql
CREATE TABLE "user_profiles" (
    "id" SERIAL PRIMARY KEY,
    "user_id" INTEGER NOT NULL,
    "display_name" TEXT,
    "bio" TEXT,
    "avatar_url" TEXT,
    "preferences" JSONB DEFAULT '{}',
    "tags" TEXT[],
    "is_public" BOOLEAN DEFAULT true,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX "idx_user_profiles_user_id" ON "user_profiles" ("user_id");
ALTER TABLE "user_profiles" ADD CONSTRAINT "fk_user_profiles_user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE;
```

**down.sql**:
```sql
DROP TABLE IF EXISTS "user_profiles";
```

## Best Practices

1. **File Naming**: Use singular table names for files (e.g., `user_profile.rs` not `user_profiles.rs`)
2. **Field Naming**: Use snake_case for field names
3. **Indexes**: Only add indexes on fields that will be frequently queried
4. **Foreign Keys**: Always specify the full table.column reference in foreign_keys section
5. **Default Values**: Use appropriate SQL default values (e.g., `CURRENT_TIMESTAMP` for timestamps)
6. **Type Safety**: Use the helper functions from `types::*` for better IDE support
7. **Testing**: Test your schema definitions in a development environment first

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Ensure all Diesel types are correctly imported and spelled
2. **Migration Conflicts**: Use descriptive migration names to avoid conflicts
3. **Macro Syntax**: Verify proper formatting of `define_table_schema!` macro syntax
4. **Foreign Key Errors**: Verify that referenced tables exist and are properly defined
5. **Type Mismatches**: Use consistent Diesel types throughout your schema definitions

### Debug Mode

To see detailed output during generation, check the console logs when running with `CREATE_SCHEMA=true`.

## Testing

```bash
# Run schema validation tests
cargo test schema::tables::product_catalog::tests

# Run hypertable example tests
cargo test schema::examples::hypertable_usage::tests

# Run schema generator tests
cargo test schema::generator

# Test hypertable validation
cargo test hypertable_validation
```

## Integration

The schema generator is integrated into the main application and runs during startup when the `CREATE_SCHEMA` environment variable is set to `true`. It's designed to work alongside existing code generation features like proto generation and gRPC controller generation.
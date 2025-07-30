# Schema Generator

The Schema Generator automatically creates Rust models, updates `schema.rs`, and generates database migrations based on table definition files. It supports both comment-based definitions and modern struct-based definitions using actual Diesel types.

## How it Works

1. **Table Definitions**: Create `.rs` files in `src/schema/tables/` with table field definitions
2. **Automatic Generation**: Run with `CREATE_SCHEMA=true cargo run` to generate:
   - Rust model structs in `src/models/`
   - Updated `schema.rs` with new table definitions
   - Database migration files in `migrations/`
3. **Smart Re-runs**: Only generates new migrations when actual schema changes are detected

## Table Definition Formats

### 🆕 Recommended: Struct-Based with Diesel Types

Use actual Diesel types for better type safety and IDE support:

```rust
// Example: user_profile_struct.rs
use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct UserProfileTable;

// Using the convenient macro
define_table_schema! {
    table_name: "user_profiles",
    fields: {
        id: integer(), primary_key: true,
        user_id: integer(), indexed: true,
        display_name: nullable(text()),
        bio: nullable(text()),
        avatar_url: nullable(text()),
        preferences: nullable(jsonb()), default: "{}",
        tags: nullable(array(text())),
        is_public: nullable(boolean()), default: "true",
        created_at: timestamptz(), default: "CURRENT_TIMESTAMP",
        updated_at: timestamptz(), default: "CURRENT_TIMESTAMP"
    },
    indexes: {
        idx_user_profiles_user_id: {
            columns: ["user_id"],
            unique: false,
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

### 📝 Legacy: Comment-Based Format

Still supported for backward compatibility:

```rust
// Example: connections.rs
// Connections table definition
// This file defines the schema for a connections table

// field_name: id
// field_type: Int4
// is_index: true
// joins_with: 
// default_value: 

// field_name: first_name
// field_type: Nullable<Text>
// is_index: false
// joins_with: 
// default_value: 

// field_name: email
// field_type: Nullable<Text>
// is_index: true
// joins_with: 
// default_value: 

// field_name: user_id
// field_type: Nullable<Int4>
// is_index: true
// joins_with: users.id
// default_value: 

// field_name: created_at
// field_type: Nullable<Timestamp>
// is_index: false
// joins_with: 
// default_value: NOW()
```

## Available Diesel Types

### Helper Functions (Recommended)
```rust
use crate::schema::generator::diesel_schema_definition::types::*;

// Basic types
integer()     // Int4
bigint()      // Int8  
text()        // Text
boolean()     // Bool
timestamp()   // Timestamp
timestamptz() // Timestamptz
jsonb()       // Jsonb
inet()        // Inet
uuid()        // Uuid

// Wrapper types
nullable(text())           // Nullable<Text>
array(text())              // Array<Text>
nullable(array(text()))    // Nullable<Array<Text>>
```

### Direct DieselType Enum
```rust
DieselType::Integer
DieselType::BigInt
DieselType::Text
DieselType::VarChar(Some(255))
DieselType::Bool
DieselType::Timestamptz
DieselType::Jsonb
DieselType::Inet
DieselType::Array(Box::new(DieselType::Text))
DieselType::Nullable(Box::new(DieselType::Text))
```

### Legacy Comment-Based Types

- `Text` - Non-nullable text
- `Nullable<Text>` - Nullable text
- `Int4` - Non-nullable 32-bit integer
- `Nullable<Int4>` - Nullable 32-bit integer
- `Int8` - Non-nullable 64-bit integer
- `Nullable<Int8>` - Nullable 64-bit integer
- `BigInt` - Non-nullable big integer
- `Nullable<BigInt>` - Nullable big integer
- `Bool` - Non-nullable boolean
- `Nullable<Bool>` - Nullable boolean
- `Timestamp` - Non-nullable timestamp
- `Nullable<Timestamp>` - Nullable timestamp
- `Timestamptz` - Non-nullable timestamp with timezone
- `Nullable<Timestamptz>` - Nullable timestamp with timezone
- `Jsonb` - Non-nullable JSONB
- `Nullable<Jsonb>` - Nullable JSONB
- `Inet` - Non-nullable IP address
- `Nullable<Inet>` - Nullable IP address
- `Array<Text>` - Array of text
- `Nullable<Array<Text>>` - Nullable array of text

## Benefits of Struct-Based Approach

✅ **Type Safety**: Uses actual Diesel types, preventing typos and mismatches  
✅ **IDE Support**: Full autocomplete and error checking  
✅ **Compile-Time Validation**: Catches errors before runtime  
✅ **Better Tooling**: Works with Rust analyzers and formatters  
✅ **Extensible**: Easy to add custom validation and logic  
✅ **Documentation**: Self-documenting with proper Rust docs  

## Field Properties

- `field_name`/`name`: The database column name
- `field_type`/`diesel_type`: The Diesel SQL type
- `is_index`/`is_indexed`: Whether to create an index (optional, default: false)
- `primary_key`: Whether this is the primary key (optional, default: false)
- `joins_with`: Foreign key reference in format `table.column` (optional)
- `default_value`/`default`: Default value for the column (optional)

### 5. What Gets Generated

When you run the schema generator, it will:

1. **Generate Model Files**: Creates `{table_name}_model.rs` in `src/models/`
2. **Update Schema**: Adds table definitions to `src/schema/schema.rs`
3. **Create Migrations**: Generates SQL migration files in `migrations/`
4. **Update Models Module**: Updates `src/models/mod.rs` to include new models

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

#### Model File (`connections_model.rs`)

```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = connections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Connection {
    pub id: i32,
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = connections)]
pub struct NewConnection {
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}
```

#### Schema Addition (`schema.rs`)

```rust
table! {
    connections (id) {
        id -> Int4,
        first_name -> Nullable<Text>,
        email -> Nullable<Text>,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
    }
}
```

#### Migration Files

**up.sql**:
```sql
-- Your SQL goes here

CREATE TABLE "connections" (
    "id" INTEGER NOT NULL,
    "first_name" TEXT,
    "email" TEXT,
    "user_id" INTEGER,
    "created_at" TIMESTAMP DEFAULT NOW()
);

CREATE INDEX "idx_connections_id" ON "connections" ("id");
CREATE INDEX "idx_connections_email" ON "connections" ("email");
CREATE INDEX "idx_connections_user_id" ON "connections" ("user_id");
```

**down.sql**:
```sql
-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS "idx_connections_user_id";
DROP INDEX IF EXISTS "idx_connections_email";
DROP INDEX IF EXISTS "idx_connections_id";
DROP TABLE IF EXISTS "connections";
```

## Best Practices

1. **File Naming**: Use singular table names for files (e.g., `connection.rs` not `connections.rs`)
2. **Field Naming**: Use snake_case for field names
3. **Indexes**: Only add indexes on fields that will be frequently queried
4. **Foreign Keys**: Always specify the full table.column reference for joins_with
5. **Default Values**: Use appropriate SQL default values (e.g., `NOW()` for timestamps)
6. **Testing**: Test your schema definitions in a development environment first

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Make sure all Diesel types are correctly spelled
2. **Migration Conflicts**: Use descriptive migration names to avoid conflicts
3. **Schema Parsing**: Ensure proper formatting of field definitions
4. **Foreign Key Errors**: Verify that referenced tables exist

### Debug Mode

To see detailed output during generation, check the console logs when running with `CREATE_SCHEMA=true`.

## Integration

The schema generator is integrated into the main application and runs during startup when the `CREATE_SCHEMA` environment variable is set to `true`. It's designed to work alongside existing code generation features like proto generation and gRPC controller generation.
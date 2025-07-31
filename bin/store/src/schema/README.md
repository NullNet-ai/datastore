# Schema Generator with Hypertable Support

This module provides a Diesel-based schema generator with built-in support for TimescaleDB hypertables and comprehensive validation.

## Features

- **Hypertable Support**: Define time-series tables with automatic validation
- **Compile-time Validation**: Ensures hypertable requirements are met
- **Migration Safety**: Prevents duplicate index and foreign key creation
- **Type Safety**: Full Rust type system integration

## Hypertable Requirements

When `hypertable: true` is specified, the following requirements are enforced:

1. **Required Fields**:
   - `timestamp: timestamptz()` - The time dimension field
   - `hypertable_timestamp: text()` - Additional timestamp metadata
   - `id` - Unique identifier field

2. **Primary Key**: Must be a composite key with `timestamp` and `id`

3. **Validation**: Automatic validation occurs when `validate_schema()` is called

## Usage Examples

### Basic Hypertable

```rust
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

### Regular Table

```rust
pub struct UsersTable;

define_table_schema! {
    table_name: "users",
    fields: {
        id: uuid(), primary_key: true,
        email: text(),
        name: text(),
        created_at: timestamptz(), default: "CURRENT_TIMESTAMP"
    },
    // hypertable: false (default)
    indexes: {
        idx_users_email: {
            columns: ["email"],
            unique: true,
            type: "btree"
        }
    }
}
```

## Validation

```rust
// Validate schema requirements
MetricsTable::validate_schema();  // Panics if hypertable requirements not met
UsersTable::validate_schema();    // Always succeeds for regular tables

// Check if table is a hypertable
if MetricsTable::is_hypertable() {
    println!("This is a hypertable!");
}
```

## Migration Best Practices

### Problem: Duplicate Index/Foreign Key Creation

The issue you mentioned about migrations recreating indexes and foreign keys can be solved with conditional SQL:

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

## File Structure

```
src/schema/
├── generator/
│   └── diesel_schema_definition.rs  # Core schema generator
├── tables/
│   ├── product_catalog.rs           # Example table definitions
│   └── ...                          # Other table definitions
├── examples/
│   └── hypertable_usage.rs          # Usage examples
└── README.md                        # This file
```

## Testing

```bash
# Run schema validation tests
cargo test schema::tables::product_catalog::tests

# Run hypertable example tests
cargo test schema::examples::hypertable_usage::tests
```
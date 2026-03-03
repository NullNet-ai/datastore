//! Migration-from-DB library: config, store client, migration runner, metrics, and shared state for the API.

pub mod api;
pub mod config;
pub mod metrics;
pub mod migration_runner;
pub mod state;
pub mod store_client;
pub mod table_order;
pub mod tables_order;

pub use config::Config;
pub use metrics::{extract_fk_constraint_from_error, MigrationMetrics};
pub use migration_runner::{run_migration, ErrorLog};
pub use state::{MigrationPhase, MigrationState, MigrationStateSnapshot, SharedMigrationState};
pub use store_client::StoreClient;
pub use table_order::{format_rust_output, generate_table_order};
pub use tables_order::{MIGRATE_CIRCULAR_FK_COLS, MIGRATE_TABLES_ORDER, MIGRATE_UNIQUE_INDEXES};

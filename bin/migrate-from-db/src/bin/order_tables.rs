//! Connects to the database and writes migrate_tables_order.rs (table order + circular FKs + unique indexes).
//! Uses the same logic as POST /generate-tables-order; run from CLI for one-off generation.
//!
//! Env:
//! - MIGRATE_FROM_DATABASE_URL (or MIGRATE_ORDER_DATABASE_URL) - Postgres URL
//! - MIGRATE_ORDER_SCHEMA       - Schema to consider (default: public)
//! - MIGRATE_TO_DATABASE_URL    - Optional: read unique indexes from this DB (destination); if unset, uses source DB
//! - MIGRATE_ORDER_OUTPUT       - Output file path (default: <crate>/migrate_tables_order.rs so GET /tables sees it)
//!
//! CLI: --destination-url <URL> overrides env for where to read unique indexes (use if .env is not loaded).

use std::env;
use std::fs;
use std::path::Path;

fn parse_destination_url_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--destination-url" && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
        if args[i].starts_with("--destination-url=") {
            return Some(args[i].trim_start_matches("--destination-url=").to_string());
        }
        i += 1;
    }
    None
}

/// Redact password for logging: show only host/database.
fn redact_url(url: &str) -> String {
    if let Some(rest) = url.strip_prefix("postgres://") {
        let without_user = rest.find('@').map(|i| &rest[i + 1..]).unwrap_or(rest);
        format!("postgres://***@{}", without_user)
    } else if let Some(rest) = url.strip_prefix("postgresql://") {
        let without_user = rest.find('@').map(|i| &rest[i + 1..]).unwrap_or(rest);
        format!("postgresql://***@{}", without_user)
    } else {
        "***".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load .env from crate dir first so MIGRATE_TO_DATABASE_URL is set even when run from workspace root
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    if env_path.exists() {
        dotenv::from_path(&env_path).ok();
    }
    dotenv::dotenv().ok();

    let database_url = env::var("MIGRATE_ORDER_DATABASE_URL")
        .or_else(|_| env::var("MIGRATE_FROM_DATABASE_URL"))
        .map_err(|_| "MIGRATE_FROM_DATABASE_URL or MIGRATE_ORDER_DATABASE_URL required")?;
    let schema = env::var("MIGRATE_ORDER_SCHEMA").unwrap_or_else(|_| "public".to_string());
    let output_path = env::var("MIGRATE_ORDER_OUTPUT").unwrap_or_else(|_| {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("migrate_tables_order.rs")
            .display()
            .to_string()
    });

    // CLI overrides env so unique indexes work even when .env isn't loaded
    let dest_url =
        parse_destination_url_from_args().or_else(|| env::var("MIGRATE_TO_DATABASE_URL").ok());
    if let Some(ref u) = dest_url {
        eprintln!(
            "order-tables: reading unique indexes from destination DB ({})",
            redact_url(u)
        );
    } else {
        eprintln!("order-tables: no destination URL (set MIGRATE_TO_DATABASE_URL or pass --destination-url <URL>); unique indexes from source (may be empty)");
    }
    let (tables, circular_fk_cols, unique_indexes) =
        migrate_from_db::generate_table_order(&database_url, &schema, dest_url.as_deref()).await?;
    let content = migrate_from_db::format_rust_output(&tables, &circular_fk_cols, &unique_indexes);
    fs::write(&output_path, content)?;
    println!(
        "Wrote {} tables ({} with unique indexes) to {}",
        tables.len(),
        unique_indexes.len(),
        output_path
    );

    Ok(())
}

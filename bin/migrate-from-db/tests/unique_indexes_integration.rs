//! Integration test for unique index detection in generate_table_order.
//!
//! Reads DB URL from .env in the crate root (bin/migrate-from-db/.env). Uses, in order:
//! MIGRATE_FROM_DATABASE_URL, then MIGRATE_TO_DATABASE_URL. If neither is set, the test is skipped.
//!
//! .env is loaded from bin/migrate-from-db/ automatically. Run from anywhere:
//!   cargo test -p migrate-from-db unique_indexes -- --ignored --nocapture

use migrate_from_db::generate_table_order;
use std::env;

const TEST_TABLE: &str = "migrate_test_unique_idx_tbl";

fn database_url_from_env() -> Option<String> {
    // Load .env from the migrate-from-db crate root (bin/migrate-from-db/.env) so it works from any cwd.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let env_path = std::path::Path::new(manifest_dir).join(".env");
    let _ = dotenv::from_path(env_path);
    env::var("MIGRATE_FROM_DATABASE_URL")
        .ok()
        .or_else(|| env::var("MIGRATE_TO_DATABASE_URL").ok())
}

#[tokio::test]
#[ignore = "run with MIGRATE_FROM_DATABASE_URL or MIGRATE_TO_DATABASE_URL in .env"]
async fn unique_indexes_detected_from_real_database() {
    let url = match database_url_from_env() {
        Some(u) => u,
        None => {
            eprintln!(
                "skip: set MIGRATE_FROM_DATABASE_URL or MIGRATE_TO_DATABASE_URL in .env to run"
            );
            return;
        }
    };

    let mut url_owned = url.clone();
    if !url.contains("sslmode=") {
        url_owned.push_str(if url.contains('?') {
            "&sslmode=disable"
        } else {
            "?sslmode=disable"
        });
    }

    let (client, connection) = tokio_postgres::connect(&url_owned, tokio_postgres::NoTls)
        .await
        .expect("connect to test database");

    tokio::spawn(async move {
        let _ = connection.await;
    });

    // Create a table with a unique partial index (similar to district_admins).
    client
        .batch_execute(&format!(
            r#"
            DROP TABLE IF EXISTS "{0}";
            CREATE TABLE "{0}" (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                district_id UUID NOT NULL,
                district_admin_id UUID NOT NULL,
                status TEXT NOT NULL DEFAULT 'Active'
            );
            CREATE UNIQUE INDEX "idx_{0}_district_admin"
                ON "{0}" USING btree(district_id, district_admin_id)
                WHERE status = 'Active';
            "#,
            TEST_TABLE
        ))
        .await
        .expect("create test table and unique index");

    let (tables, _circular_fk_cols, unique_indexes) = generate_table_order(&url, "public", None)
        .await
        .expect("generate_table_order");

    // Our test table should be in the table list.
    assert!(
        tables.iter().any(|t| t == TEST_TABLE),
        "expected {} in tables, got: {:?}",
        TEST_TABLE,
        tables
    );

    // unique_indexes should contain our table and the index we created.
    let indexes = unique_indexes.get(TEST_TABLE).unwrap_or_else(|| {
        panic!(
            "unique_indexes should contain the test table {}; keys: {:?}",
            TEST_TABLE,
            unique_indexes.keys().collect::<Vec<_>>()
        )
    });
    assert!(
        !indexes.is_empty(),
        "expected at least one unique index for {}, got empty; all unique_indexes: {:?}",
        TEST_TABLE,
        unique_indexes
    );

    let (index_name, index_def) = indexes
        .iter()
        .find(|(name, def)| name.contains("district_admin") || def.contains("CREATE UNIQUE INDEX"))
        .expect("expected an index with district_admin in name or CREATE UNIQUE INDEX in def");

    assert!(
        index_def.contains("CREATE UNIQUE INDEX"),
        "index def should be full CREATE UNIQUE INDEX statement, got: {}",
        index_def
    );
    assert!(
        index_def.contains("district_id") && index_def.contains("district_admin_id"),
        "index def should mention columns, got: {}",
        index_def
    );

    // Cleanup.
    client
        .batch_execute(&format!(r#"DROP TABLE IF EXISTS "{}";"#, TEST_TABLE))
        .await
        .expect("drop test table");

    eprintln!(
        "ok: unique index detected: {} -> ({} , def len {})",
        TEST_TABLE,
        index_name,
        index_def.len()
    );
}

//! One-shot migration from env (for scripting / make run).
//! Use the API (POST /migrate) when running the migration service.

use std::sync::Arc;

use migrate_from_db::{run_migration, Config, ErrorLog};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let config = Config::from_env()?;
    let error_log = ErrorLog::open(&config.error_log_path)?;
    eprintln!("Errors will be appended to: {}", config.error_log_path);

    let state = Arc::new(std::sync::RwLock::new(
        migrate_from_db::MigrationState::default(),
    ));
    run_migration(config, state, error_log, None, None).await
}

//! Migration API server: POST /migrate, GET /status, GET /tables.
//! Run migration by sending a request to /migrate with URLs and credentials.
//! When MIGRATE_METRICS_PORT is set (e.g. 9090), exposes Prometheus /metrics on that port.

use std::net::SocketAddr;
use std::sync::Arc;

use migrate_from_db::{api, api::AppState, MigrationMetrics, MigrationState, SharedMigrationState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let migration_state: Arc<SharedMigrationState> =
        Arc::new(std::sync::RwLock::new(MigrationState::default()));

    let metrics = match std::env::var("MIGRATE_METRICS_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
    {
        Some(port) => match MigrationMetrics::new() {
            Ok(m) => {
                let m = Arc::new(m);
                migrate_from_db::metrics::spawn_metrics_server(Arc::clone(&m), port);
                migrate_from_db::metrics::spawn_rows_per_second_updater(Arc::clone(&m));
                Some(m)
            }
            Err(e) => {
                eprintln!("Metrics init failed: {} (continuing without metrics)", e);
                None
            }
        },
        None => None,
    };

    let error_log_path = std::env::var("MIGRATE_ERROR_LOG").unwrap_or_else(|_| {
        std::env::current_dir()
            .map(|d| d.join("migrate_errors.log").to_string_lossy().to_string())
            .unwrap_or_else(|_| "migrate_errors.log".to_string())
    });

    let app_state = Arc::new(AppState {
        migration_state,
        metrics,
        error_log_path,
    });

    let port: u16 = std::env::var("MIGRATE_API_PORT")
        .unwrap_or_else(|_| "3090".to_string())
        .parse()?;
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;

    let app = api::router(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    eprintln!("Migration API listening on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

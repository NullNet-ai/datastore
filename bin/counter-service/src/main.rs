//! Counter service binary: gRPC server + HTTP API (migrate route) with Redis (deadpool-redis) for unique code generation.

use counter_service::server::CodeServiceImpl;
use deadpool_redis::Config;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load .env from crate directory first, then CWD (so bin/counter-service/.env is used when run from workspace root)
    let crate_env = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    if crate_env.exists() {
        dotenv::from_path(&crate_env).ok();
    }
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
    let cfg = Config::from_url(redis_url.clone());
    let pool = cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

    let grpc_addr: SocketAddr = std::env::var("CODE_SERVICE_GRPC_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:50051".into())
        .parse()?;
    let http_addr: SocketAddr = std::env::var("CODE_SERVICE_HTTP_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()?;

    let svc = CodeServiceImpl::new(pool.clone()).into_service();
    let grpc_server = Server::builder().add_service(svc).serve(grpc_addr);

    let app = counter_service::http_api::router(pool);
    let http_server = axum::serve(
        tokio::net::TcpListener::bind(http_addr).await?,
        app,
    );

    tracing::info!("Counter service gRPC listening on {}", grpc_addr);
    tracing::info!("Counter service HTTP (migrate) listening on {}", http_addr);

    tokio::select! {
        res = grpc_server => res.map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.into() }),
        res = http_server => res.map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.into() }),
    }
}

//! Prometheus metrics for migration. Exposes /metrics when MIGRATE_METRICS_PORT is set.
//!
//! Metrics:
//! - migration_rows_total{table, status="success|error"}
//! - migration_rows_per_second{table}
//! - migration_duration_seconds{table}
//! - migration_fk_violations_total{table, constraint}
//! - migration_retry_total{table}
//! - migration_progress_percent{table}
//! - migration_second_pass_total{table, status="success|error"}

use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounterVec, IntGaugeVec, Opts, Registry,
    TextEncoder,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Shared state for computing rows/sec: table -> count of rows inserted (for rate calculation).
pub type RowsCountSnapshot = Arc<std::sync::Mutex<HashMap<String, u64>>>;

pub struct MigrationMetrics {
    pub registry: Registry,
    /// migration_rows_total{table, status="success|error"}
    pub migration_rows_total: IntCounterVec,
    /// migration_rows_per_second{table} — updated every 1s by background task
    pub migration_rows_per_second: IntGaugeVec,
    /// migration_duration_seconds{table} — time spent on each table (insert pass)
    pub migration_duration_seconds: HistogramVec,
    /// migration_fk_violations_total{table, constraint}
    pub migration_fk_violations_total: IntCounterVec,
    /// migration_retry_total{table} — increment when retrying a table
    #[allow(dead_code)]
    pub migration_retry_total: IntCounterVec,
    /// migration_progress_percent{table} — 0–100
    pub migration_progress_percent: IntGaugeVec,
    /// migration_second_pass_total{table, status="success|error"}
    pub migration_second_pass_total: IntCounterVec,

    /// Current row count per table (for rows_per_second background task).
    pub rows_count: RowsCountSnapshot,
}

impl MigrationMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Registry::new();

        let migration_rows_total = IntCounterVec::new(
            Opts::new(
                "migration_rows_total",
                "Total rows processed per table (insert pass), by status",
            ),
            &["table", "status"],
        )?;
        registry.register(Box::new(migration_rows_total.clone()))?;

        let migration_rows_per_second = IntGaugeVec::new(
            Opts::new(
                "migration_rows_per_second",
                "Approximate rows inserted per second per table (updated every 1s)",
            ),
            &["table"],
        )?;
        registry.register(Box::new(migration_rows_per_second.clone()))?;

        let migration_duration_seconds = HistogramVec::new(
            HistogramOpts::new(
                "migration_duration_seconds",
                "Time spent migrating each table (insert pass)",
            )
            .buckets(vec![1.0, 5.0, 10.0, 30.0, 60.0, 120.0, 300.0, 600.0]),
            &["table"],
        )?;
        registry.register(Box::new(migration_duration_seconds.clone()))?;

        let migration_fk_violations_total = IntCounterVec::new(
            Opts::new(
                "migration_fk_violations_total",
                "Foreign key constraint violations per table",
            ),
            &["table", "constraint"],
        )?;
        registry.register(Box::new(migration_fk_violations_total.clone()))?;

        let migration_retry_total = IntCounterVec::new(
            Opts::new("migration_retry_total", "Retry attempts per table"),
            &["table"],
        )?;
        registry.register(Box::new(migration_retry_total.clone()))?;

        let migration_progress_percent = IntGaugeVec::new(
            Opts::new(
                "migration_progress_percent",
                "Migration progress per table (0–100)",
            ),
            &["table"],
        )?;
        registry.register(Box::new(migration_progress_percent.clone()))?;

        let migration_second_pass_total = IntCounterVec::new(
            Opts::new(
                "migration_second_pass_total",
                "Second pass (patch) results per table, by status",
            ),
            &["table", "status"],
        )?;
        registry.register(Box::new(migration_second_pass_total.clone()))?;

        Ok(Self {
            registry,
            migration_rows_total,
            migration_rows_per_second,
            migration_duration_seconds,
            migration_fk_violations_total,
            migration_retry_total,
            migration_progress_percent,
            migration_second_pass_total,
            rows_count: Arc::new(std::sync::Mutex::new(HashMap::new())),
        })
    }

    /// Increment row count for a table (used by background task for rows_per_second).
    pub fn inc_row_count(&self, table: &str) {
        let mut m = self.rows_count.lock().unwrap_or_else(|e| e.into_inner());
        *m.entry(table.to_string()).or_insert(0) += 1;
    }

    /// Gather metrics in Prometheus text format.
    pub fn gather(&self) -> Result<String, prometheus::Error> {
        let metric_families = self.registry.gather();
        let encoder = TextEncoder::new();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}

/// Spawns a task that every 1s updates migration_rows_per_second from rows_count snapshot.
pub fn spawn_rows_per_second_updater(metrics: Arc<MigrationMetrics>) {
    tokio::spawn(async move {
        let mut last: HashMap<String, u64> = HashMap::new();
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let current = {
                let m = metrics.rows_count.lock().unwrap_or_else(|e| e.into_inner());
                m.clone()
            };
            for (table, count) in &current {
                let rate = count.saturating_sub(*last.get(table).unwrap_or(&0));
                metrics
                    .migration_rows_per_second
                    .with_label_values(&[table])
                    .set(rate as i64);
            }
            last = current;
        }
    });
}

/// Spawn a TCP server that serves /metrics on the given port.
pub fn spawn_metrics_server(metrics: Arc<MigrationMetrics>, port: u16) {
    tokio::spawn(async move {
        let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Metrics server: failed to bind :{}: {}", port, e);
                return;
            }
        };
        eprintln!("Metrics: http://0.0.0.0:{}/metrics", port);
        loop {
            let (mut stream, _) = match listener.accept().await {
                Ok(accept) => accept,
                Err(e) => {
                    eprintln!("Metrics server accept error: {}", e);
                    continue;
                }
            };
            let metrics = Arc::clone(&metrics);
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                let _ = stream.read(&mut buf).await;
                let body = metrics.gather().unwrap_or_else(|e| format!("# error: {}", e));
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(response.as_bytes()).await;
                let _ = stream.shutdown().await;
            });
        }
    });
}

/// If the error message looks like an FK violation, return Some(constraint_name).
/// Otherwise None.
pub fn extract_fk_constraint_from_error(msg: &str) -> Option<String> {
    let lower = msg.to_lowercase();
    if !lower.contains("foreign key") && !lower.contains("violates") {
        return None;
    }
    // Try to extract constraint name, e.g. "violates foreign key constraint \"fk_foo\""
    if let Some(start) = msg.find("constraint") {
        let after = &msg[start + "constraint".len()..];
        let after = after.trim_start_matches(|c: char| c == ' ' || c == '"' || c == '\'');
        let end = after
            .find(|c: char| c == '"' || c == '\'' || c == ' ' || c == '\n')
            .unwrap_or(after.len());
        let name = after[..end].trim_matches('"').trim_matches('\'').to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }
    Some("unknown".to_string())
}

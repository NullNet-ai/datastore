//! Shared migration state for the API (status route and migration runner).

use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize)]
pub enum MigrationPhase {
    Idle,
    Migration,
    Patch,
    Done,
    Error,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TableError {
    pub row_index: Option<usize>,
    pub row_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TableStats {
    pub records_total: u64,
    pub records_inserted: u64,
    pub errors_count: u64,
    pub errors: Vec<TableError>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchTableStats {
    pub patched: u64,
    pub errors_count: u64,
    pub errors: Vec<TableError>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchPhaseStats {
    pub total_patches: u64,
    pub patched_count: u64,
    pub errors_count: u64,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub by_table: HashMap<String, PatchTableStats>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrationStateSnapshot {
    pub phase: MigrationPhase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_error: Option<String>,
    pub tables: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub table_stats: HashMap<String, TableStats>,
    pub patch_phase: PatchPhaseStats,
}

pub struct MigrationState {
    pub phase: MigrationPhase,
    /// Wall-clock time when migration started (for display and duration).
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Errors encountered during migration start (login, DB connect, etc.).
    pub start_error: HashSet<String>,
    pub tables: Vec<String>,
    pub table_stats: HashMap<String, TableStats>,
    pub patch_phase: PatchPhaseStats,
}

impl Default for MigrationState {
    fn default() -> Self {
        Self {
            phase: MigrationPhase::Idle,
            started_at: None,
            start_error: HashSet::new(),
            tables: Vec::new(),
            table_stats: HashMap::new(),
            patch_phase: PatchPhaseStats::default(),
        }
    }
}

impl MigrationState {
    pub fn snapshot(&self) -> MigrationStateSnapshot {
        let now = chrono::Utc::now();
        let duration_secs = self.started_at.map(|t| {
            (now - t).num_milliseconds() as f64 / 1000.0
        });
        let started_at = self.started_at
            .map(|t| t.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string());
        MigrationStateSnapshot {
            phase: self.phase.clone(),
            started_at,
            duration_secs,
            start_error: if self.start_error.is_empty() {
                None
            } else {
                Some(self.start_error.iter().cloned().collect::<Vec<_>>().join("; "))
            },
            tables: self.tables.clone(),
            table_stats: self.table_stats.clone(),
            patch_phase: self.patch_phase.clone(),
        }
    }
}

pub type SharedMigrationState = RwLock<MigrationState>;

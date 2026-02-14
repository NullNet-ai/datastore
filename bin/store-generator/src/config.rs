//! Configuration for store-generator.
//!
//! Configuration is resolved in order of precedence:
//! 1. `store-generator.toml` config file (in cwd, store_dir, or workspace)
//! 2. Environment variables (STORE_DIR, etc.)
//! 3. Defaults

use serde::Deserialize;
use std::env;
use std::path::Path;

pub const CONFIG_FILENAME: &str = "store-generator.toml";

/// Project configuration for store-generator.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// Root directory of the store crate. All paths are relative to this.
    pub store_dir: Option<String>,

    /// Override for schema tables directory (relative to store_dir)
    pub schema_tables_dir: Option<String>,
    /// Override for migrations directory
    pub migrations_dir: Option<String>,
    /// Override for models directory
    pub models_dir: Option<String>,
}

impl Config {
    /// Load config from the given path.
    pub fn load_from_path(path: &Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::Io(e))?;
        toml::from_str(&content).map_err(ConfigError::Parse)
    }

    /// Discover and load config. Searches:
    /// - Current working directory
    /// - STORE_DIR if set
    /// - Parent directories (workspace root)
    pub fn discover() -> Self {
        let cwd = env::current_dir().unwrap_or_default();
        let store_dir_env = env::var("STORE_DIR").ok();

        // 1. Config in cwd
        let cwd_config = cwd.join(CONFIG_FILENAME);
        if cwd_config.exists() {
            if let Ok(cfg) = Self::load_from_path(&cwd_config) {
                return cfg;
            }
        }

        // 2. Config next to store_dir (e.g. bin/store/store-generator.toml)
        if let Some(ref sd) = store_dir_env {
            let p = Path::new(sd).join(CONFIG_FILENAME);
            if p.exists() {
                if let Ok(cfg) = Self::load_from_path(&p) {
                    return cfg;
                }
            }
        }

        // 3. Config at workspace root (e.g. crdt-workspace/store-generator.toml)
        let mut dir = cwd.clone();
        for _ in 0..10 {
            let p = dir.join(CONFIG_FILENAME);
            if p.exists() {
                if let Ok(cfg) = Self::load_from_path(&p) {
                    return cfg;
                }
            }
            if !dir.pop() {
                break;
            }
        }

        // 4. Build from env vars / defaults
        let mut cfg = Config::default();
        cfg.store_dir = store_dir_env;
        cfg
    }

    /// Get the effective store directory.
    pub fn store_dir(&self) -> String {
        self.store_dir
            .clone()
            .or_else(|| env::var("STORE_DIR").ok())
            .unwrap_or_else(|| ".".to_string())
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(e) => write!(f, "config IO error: {}", e),
            ConfigError::Parse(e) => write!(f, "config parse error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

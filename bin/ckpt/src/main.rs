//! Local checkpoint tool using content-addressed storage (hash + manifest).
//!
//! Stores file blobs by SHA-256 hash and checkpoint manifests mapping paths to hashes.
//! Enables save/restore/diff of tracked files without git.

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const ROOT_DIR: &str = ".mycheckpoints";
const BLOBS_DIR: &str = ".mycheckpoints/blobs";
const CKPTS_DIR: &str = ".mycheckpoints/checkpoints";

#[derive(Parser)]
#[command(
    name = "ckpt",
    version,
    about = "Local checkpoint tool (hash + manifest)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize checkpoint storage
    Init,

    /// Save a checkpoint
    Save {
        /// Human-readable name for the checkpoint
        #[arg(long)]
        name: String,

        /// Comma-separated list of file paths to track
        #[arg(long)]
        files: String,
    },

    /// List checkpoints
    List,

    /// Diff two checkpoints by id
    Diff { id_a: String, id_b: String },

    /// Restore a checkpoint by id
    Restore {
        id: String,

        /// If set, delete files that exist in working dir but not in checkpoint manifest
        #[arg(long)]
        delete_extra: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Manifest {
    id: u64,
    created_at: String, // ISO8601 UTC
    name: String,
    files: BTreeMap<String, String>, // path -> sha256hex
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Init => cmd_init(),
        Command::Save { name, files } => cmd_save(&name, &files),
        Command::List => cmd_list(),
        Command::Diff { id_a, id_b } => cmd_diff(&id_a, &id_b),
        Command::Restore { id, delete_extra } => cmd_restore(&id, delete_extra),
    }
}

fn ensure_store() -> Result<()> {
    fs::create_dir_all(BLOBS_DIR).context("create blobs dir")?;
    fs::create_dir_all(CKPTS_DIR).context("create checkpoints dir")?;
    Ok(())
}

fn cmd_init() -> Result<()> {
    ensure_store()?;
    println!("Initialized {}", ROOT_DIR);
    Ok(())
}

fn cmd_save(name: &str, files_csv: &str) -> Result<()> {
    ensure_store()?;

    let mut manifest = Manifest {
        id: next_id()?,
        created_at: Utc::now().to_rfc3339(),
        name: name.to_string(),
        files: BTreeMap::new(),
    };

    let paths = parse_csv_paths(files_csv);
    if paths.is_empty() {
        return Err(anyhow!("No files provided. Use --files a.txt,b.txt"));
    }

    for path in paths {
        let path_str = path.to_string_lossy().to_string();
        let bytes = fs::read(&path).with_context(|| format!("read file: {}", path.display()))?;
        let hash = sha256_hex(&bytes);

        // Store blob if missing (dedupe)
        let blob_path = Path::new(BLOBS_DIR).join(&hash);
        if !blob_path.exists() {
            atomic_write(&blob_path, &bytes)
                .with_context(|| format!("write blob: {}", blob_path.display()))?;
        }

        manifest.files.insert(path_str, hash);
    }

    let filename = format!(
        "{}_{}_{}.json",
        manifest.id,
        sanitize_for_filename(&manifest.created_at),
        sanitize_for_filename(&manifest.name)
    );
    let out_path = Path::new(CKPTS_DIR).join(filename);

    let json = serde_json::to_vec_pretty(&manifest).context("serialize manifest")?;
    atomic_write(&out_path, &json).context("write manifest")?;

    println!(
        "Saved checkpoint {} ({}) with {} file(s)",
        manifest.id,
        manifest.name,
        manifest.files.len()
    );
    Ok(())
}

fn cmd_list() -> Result<()> {
    ensure_store()?;

    let mut entries: Vec<PathBuf> = fs::read_dir(CKPTS_DIR)
        .context("read checkpoints dir")?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().map(|x| x == "json").unwrap_or(false))
        .collect();

    // Sort by filename (id first)
    entries.sort();

    if entries.is_empty() {
        println!("No checkpoints found.");
        return Ok(());
    }

    for p in entries {
        let data = fs::read(&p).with_context(|| format!("read {}", p.display()))?;
        let m: Manifest =
            serde_json::from_slice(&data).with_context(|| format!("parse {}", p.display()))?;
        println!(
            "id={}  at={}  name={}  files={}",
            m.id,
            m.created_at,
            m.name,
            m.files.len()
        );
    }

    Ok(())
}

/// Load manifest by id (finds file in CKPTS_DIR whose filename starts with "<id>_")
fn load_manifest_by_id(id: &str) -> Result<Manifest> {
    let id_str = id.to_string();
    let ckpts = Path::new(CKPTS_DIR);

    if !ckpts.exists() {
        return Err(anyhow!(
            "Checkpoints dir does not exist. Run `ckpt init` first."
        ));
    }

    for entry in fs::read_dir(ckpts).context("read checkpoints dir")? {
        let path = entry?.path();
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if name.starts_with(&format!("{}_", id_str)) && name.ends_with(".json") {
                let data = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
                return serde_json::from_slice(&data)
                    .with_context(|| format!("parse manifest {}", path.display()));
            }
        }
    }

    Err(anyhow!("Checkpoint with id '{}' not found", id))
}

fn cmd_diff(id_a: &str, id_b: &str) -> Result<()> {
    ensure_store()?;

    let m_a = load_manifest_by_id(id_a).with_context(|| format!("load checkpoint {}", id_a))?;
    let m_b = load_manifest_by_id(id_b).with_context(|| format!("load checkpoint {}", id_b))?;

    let set_a: HashSet<&String> = m_a.files.keys().collect();
    let set_b: HashSet<&String> = m_b.files.keys().collect();

    let added: Vec<_> = set_b.difference(&set_a).collect();
    let removed: Vec<_> = set_a.difference(&set_b).collect();
    let modified: Vec<_> = set_a
        .intersection(&set_b)
        .filter(|path| m_a.files.get((*path).as_str()) != m_b.files.get((*path).as_str()))
        .collect();

    println!("Diff {} -> {}:", id_a, id_b);
    if !added.is_empty() {
        println!("  Added:");
        for p in &added {
            println!("    + {}", p);
        }
    }
    if !removed.is_empty() {
        println!("  Removed:");
        for p in &removed {
            println!("    - {}", p);
        }
    }
    if !modified.is_empty() {
        println!("  Modified:");
        for p in &modified {
            println!("    ~ {}", p);
        }
    }
    if added.is_empty() && removed.is_empty() && modified.is_empty() {
        println!("  (no changes)");
    }

    Ok(())
}

fn cmd_restore(id: &str, delete_extra: bool) -> Result<()> {
    ensure_store()?;

    let manifest = load_manifest_by_id(id).with_context(|| format!("load checkpoint {}", id))?;
    let blobs = Path::new(BLOBS_DIR);

    for (path_str, hash) in &manifest.files {
        let blob_path = blobs.join(hash);
        let bytes = fs::read(&blob_path)
            .with_context(|| format!("read blob {} (file: {})", hash, path_str))?;

        let target = Path::new(path_str);
        atomic_write(target, &bytes).with_context(|| format!("restore file: {}", path_str))?;
    }

    if delete_extra {
        // Collect paths that exist on disk and are in the "tracked set"
        // The tracked set = all paths we could have in any checkpoint (from manifest.files)
        // For delete_extra, we delete files that:
        // - exist in the current working directory
        // - are in the same "tracked roots" as manifest paths
        // - but are NOT in manifest.files
        //
        // Simplified: for each directory containing a restored file, list files
        // and delete those not in manifest. We'll be conservative: only delete
        // files that share a parent dir with a manifest path and are not in manifest.
        let mut dirs_to_check: HashSet<PathBuf> = HashSet::new();
        for path_str in manifest.files.keys() {
            if let Some(parent) = Path::new(path_str).parent() {
                dirs_to_check.insert(parent.to_path_buf());
            }
        }

        for dir in dirs_to_check {
            if !dir.exists() {
                continue;
            }
            let entries = match fs::read_dir(&dir) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let path_str = path.to_string_lossy().to_string();
                    if !manifest.files.contains_key(&path_str) {
                        if let Err(e) = fs::remove_file(&path) {
                            eprintln!("Warning: could not delete extra file {}: {}", path_str, e);
                        } else {
                            println!("Deleted extra: {}", path_str);
                        }
                    }
                }
            }
        }
    }

    println!(
        "Restored checkpoint {} ({} file(s))",
        id,
        manifest.files.len()
    );
    Ok(())
}

fn next_id() -> Result<u64> {
    let mut max_id: u64 = 0;
    let ckpts = Path::new(CKPTS_DIR);

    if ckpts.exists() {
        for entry in fs::read_dir(ckpts).context("read checkpoints dir")? {
            let path = entry?.path();
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if let Some((id_part, _rest)) = name.split_once('_') {
                    if let Ok(id) = id_part.parse::<u64>() {
                        max_id = max_id.max(id);
                    }
                }
            }
        }
    }

    Ok(max_id + 1)
}

fn parse_csv_paths(csv: &str) -> Vec<PathBuf> {
    csv.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    hex::encode(digest)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent {}", parent.display()))?;
    }

    let tmp = path.with_extension("tmp");
    {
        let mut f =
            fs::File::create(&tmp).with_context(|| format!("create tmp {}", tmp.display()))?;
        f.write_all(bytes).context("write bytes")?;
        f.sync_all().ok(); // best-effort
    }
    fs::rename(&tmp, path)
        .with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}

fn sanitize_for_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => c,
            _ => '-',
        })
        .collect()
}

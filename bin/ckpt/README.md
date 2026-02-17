# ckpt

Local checkpoint tool using content-addressed storage. Save, restore, and diff file snapshots without git.

## Overview

ckpt stores file content by SHA-256 hash and keeps checkpoint manifests that map file paths to hashes. It supports quick rollbacks, comparisons between checkpoints, and deduplication. Use it to create restore points before risky changes or to track file snapshots in any project.

## Installation

```bash
cargo install ckpt
```

For a private registry:

```bash
cargo install ckpt --registry dnamicro
```

## Commands

### init

Initialize the checkpoint store (creates `.mycheckpoints/blobs` and `.mycheckpoints/checkpoints` in the current directory).

```bash
ckpt init
```

### save

Create a checkpoint of the given files.

```bash
ckpt save --name "pre-refactor" --files "src/main.rs,src/lib.rs"
ckpt save --name "backup" --files "config.json,data/file1.csv,data/file2.csv"
```

Paths are relative to the current working directory.

### list

List all saved checkpoints (id, timestamp, name, file count).

```bash
ckpt list
```

### diff

Compare two checkpoints and show added, removed, and modified files.

```bash
ckpt diff 1 2
```

### restore

Restore files from a checkpoint by id.

```bash
# Restore files (overwrites only files present in the checkpoint)
ckpt restore 1

# Restore and delete files that exist on disk but are not in the checkpoint
# Use when you want the working directory to exactly match the checkpoint
ckpt restore 1 --delete-extra
```

**`--delete-extra`**: In each directory containing restored files, removes any files that are not in the checkpoint manifest. Useful for a full rollback when new files were added after the checkpoint.

## Storage

| Path | Purpose |
|------|---------|
| `.mycheckpoints/blobs/<sha256>` | Content-addressed file blobs (deduplicated) |
| `.mycheckpoints/checkpoints/<id>_<timestamp>_<name>.json` | Checkpoint manifests (path → hash) |

Manifest format:

```json
{
  "id": 1,
  "created_at": "2026-02-17T00:57:26+00:00",
  "name": "pre-refactor",
  "files": {
    "path/to/file.rs": "a1b2c3d4..."
  }
}
```

## License

MIT OR Apache-2.0

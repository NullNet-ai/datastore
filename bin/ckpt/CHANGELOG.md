# Changelog

All notable changes to the ckpt crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.2
### Author
Kashan

### Changed
- **README**: Rewritten to be project-agnostic – generic examples and usage, no project-specific content.
- **README**: Added full documentation for `restore --delete-extra`.

## 0.1.1
### Author
Kashan

### Fixed
- **Diff**: Corrected BTreeMap key lookup in `cmd_diff` – use `.get((*path).as_str())` instead of indexing to fix `Borrow` trait errors.
- **List**: Fixed use-after-move in diff output – iterate over `&added`, `&removed`, `&modified` so vectors remain available for `is_empty()` check.

## 0.1.0
### Author
Kashan

### Added
- **CLI commands**: `init`, `save`, `list`, `diff`, `restore`
- **Content-addressed storage**: Blobs stored at `.mycheckpoints/blobs/<sha256hex>`, manifests at `.mycheckpoints/checkpoints/<id>_<timestamp>_<name>.json`
- **Deduplication**: Blobs are stored by SHA-256 hash; no duplicate writes
- **Atomic writes**: Temp file + rename for safe file writes
- **`restore --delete-extra`**: Option to remove files on disk that are not in the checkpoint manifest

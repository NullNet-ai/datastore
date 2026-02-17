# ckpt

Local checkpoint tool using content-addressed storage (hash + manifest). Save, restore, and diff file snapshots without git.

## Installation

```bash
cargo install ckpt
```

## Usage

```bash
ckpt init                                    # Initialize .mycheckpoints/{blobs,checkpoints}
ckpt save --name "my-checkpoint" --files "a.rs,b.rs"
ckpt list                                    # List saved checkpoints
ckpt diff <id_a> <id_b>                      # Compare two checkpoints
ckpt restore <id> [--delete-extra]           # Restore files from checkpoint
```

## Storage

- Blobs: `.mycheckpoints/blobs/<sha256hex>`
- Manifests: `.mycheckpoints/checkpoints/<id>_<timestamp>_<name>.json`
- Deduplication: blobs are stored by content hash; no duplicate writes.

## License

MIT OR Apache-2.0

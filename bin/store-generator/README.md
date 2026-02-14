# store-generator

Standalone code generator for store schema, models, migrations, proto, table_enum, and grpc_controller. Can be used as a CLI binary or as a library.

## Configuration

Configuration is resolved in order of precedence:

1. **`store-generator.toml`** config file
2. **Environment variables** (e.g. `STORE_DIR`)
3. **Defaults**

### Config file

Place `store-generator.toml` at the workspace root or in the store directory:

```toml
# Root of the store crate
store_dir = "bin/store"

# Optional path overrides (relative to store_dir)
# schema_tables_dir = "src/database/schema/tables"
# migrations_dir = "migrations"
# models_dir = "src/generated/models"
```

Config is discovered from: current directory → `STORE_DIR` → parent dirs up to workspace root.

### Environment variables

| Variable | Description |
|----------|-------------|
| `STORE_DIR` | Root of the store crate (e.g. `bin/store`) |
| `CREATE_SCHEMA` | Set to `true` to run schema generation |
| `GENERATE_PROTO` | Set to `true` to generate proto files |
| `GENERATE_GRPC` | Set to `true` to generate gRPC controller |
| `GENERATE_TABLE_ENUM` | Set to `true` to generate table enum |

## CLI usage

From workspace root:

```bash
# Using config file (store-generator.toml at workspace root)
CREATE_SCHEMA=true cargo run -p store-generator

# Override via env
STORE_DIR=bin/store CREATE_SCHEMA=true GENERATE_PROTO=true cargo run -p store-generator
```

Or via Make:

```bash
make store-generator-schema
make store-generator-proto
make store-generator-all
```

## Library usage

Add to `Cargo.toml`:

```toml
[dependencies]
store-generator = { path = "../store-generator" }

# Or from crates.io after publishing:
# store-generator = "0.1"
```

Programmatic usage:

```rust
use store_generator::{generator, templates};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Schema generation
    generator::run_schema()?;

    // Proto generation
    let schema_path = store_generator::paths::schema_file();
    let output_dir = store_generator::paths::proto_output_dir();
    templates::proto_generator::generate_protos(&schema_path, &output_dir);

    // Table enum
    templates::table_enum::run_generator()?;

    Ok(())
}
```

### Config for library users

Load config explicitly when integrating into your build:

```rust
use store_generator::Config;

let config = Config::discover();
println!("Store dir: {}", config.store_dir());
```

## Publishing

### To dnamicro registry

1. Ensure `~/.cargo/config.toml` or `.cargo/config.toml` includes the dnamicro registry (see project root).
2. From the store-generator directory:
   ```bash
   cd bin/store-generator
   cargo publish
   ```

### To crates.io

1. Uncomment and set `repository` in `Cargo.toml`
2. Remove or change `publish = ["dnamicro"]` to allow crates.io
3. Run `cargo publish` from the store-generator directory

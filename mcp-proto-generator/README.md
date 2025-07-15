# MCP Proto Generator

A standalone Model Context Protocol (MCP) server that generates Protocol Buffer (.proto) files from Diesel schema content (SQL DDL). This allows Claude to generate protobuf definitions directly from database schemas.

## Features

- **Standalone MCP Server**: Works independently of any existing project
- **SQL to Proto Conversion**: Converts SQL DDL statements to Protocol Buffer definitions
- **CRUD Operations**: Generates complete CRUD request/response messages
- **Type Mapping**: Intelligent mapping from SQL types to protobuf types
- **Claude Integration**: Direct integration with Claude Desktop

## Installation

**Install from Crates.io (Recommended):**
```bash
cargo install mcp-diesel-proto-generator
```

The binary will be installed to `~/.cargo/bin/mcp-diesel-proto-generator`.

**Build from Source:**
```bash
git clone <repository-url>
cd mcp-diesel-proto-generator
cargo build --release
```

## Claude Desktop Configuration

Add the following to your Claude Desktop configuration file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows:** `%APPDATA%/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "proto-generator": {
      "command": "/Users/yourusername/.cargo/bin/mcp-diesel-proto-generator",
      "args": []
    }
  }
}
```

**Important:** Replace `/Users/yourusername` with your actual home directory path, or use the full path where the binary is installed.

## Usage with Claude

Once configured, you can use the following commands in Claude:

### Generate Proto from Schema Content

```
Generate a proto file from this Diesel schema:

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    content TEXT,
    published_at TIMESTAMP
);
```

### Custom Package Name

```
Generate a proto file with package name "blog" from this schema:

[your schema here]
```

## Supported SQL Types

The MCP server maps SQL types to protobuf types as follows:

| SQL Type | Protobuf Type |
|----------|---------------|
| INT, INTEGER, SERIAL | int32 |
| BIGINT, BIGSERIAL | int64 |
| SMALLINT | int32 |
| FLOAT | float |
| DOUBLE, DECIMAL, NUMERIC | double |
| BOOLEAN, BOOL | bool |
| TEXT, VARCHAR, CHAR | string |
| UUID | string |
| TIMESTAMP | Timestamp |
| DATE, TIME | string |
| JSON, JSONB | string |
| BYTEA | bytes |

## Generated Proto Structure

The generated proto files include:

1. **Entity Messages**: One message per table with all fields
2. **CRUD Request/Response Messages**: Complete CRUD operations for each entity
3. **Common Structures**: Shared parameter and query structures
4. **Service Definition**: gRPC service with all CRUD operations
5. **Timestamp Message**: Custom timestamp representation

## Example Output

For a `users` table, the generator creates:

- `Users` message (entity)
- `CreateUsersRequest/Response`
- `GetUsersRequest/Response` 
- `UpdateUsersRequest/Response`
- `DeleteUsersRequest/Response`
- Service methods for all operations

## Troubleshooting

### MCP Server Not Found
- Ensure the binary path in `claude_desktop_config.json` is absolute and correct
- If installed via `cargo install`, the binary should be at `~/.cargo/bin/mcp-diesel-proto-generator`
- Restart Claude Desktop after configuration changes

### Schema Parsing Issues
- Ensure your SQL DDL is valid
- The parser expects standard `CREATE TABLE` statements
- Complex constraints and triggers may not be fully parsed

### Permission Issues
- Ensure the binary has execute permissions: `chmod +x target/release/mcp-proto-generator`

## Development

To modify or extend the MCP server:

1. Clone the repository and edit `src/main.rs`
2. Rebuild with `cargo build --release`
3. Update your Claude Desktop configuration to point to the new binary
4. Restart Claude Desktop to pick up changes

## License

This project is provided as-is for educational and development purposes.